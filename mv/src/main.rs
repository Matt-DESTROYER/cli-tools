use std::{
    env,
    fs,
    io::{
        ErrorKind,
        Write,
        stdin,
        stdout
    },
    path::PathBuf
};

#[derive(PartialEq)]
enum MVBackup {
    None,
    Numbered,
    Existing,
    Simple
}

struct MVOpts {
    backup: bool,
    backup_control: MVBackup,
    backup_suffix: String,
    debug: bool,
    interactive: bool,
    no_clobber: bool,
    strip_trailing_slashes: bool,
    verbose: bool
}

fn input(prompt: &str) -> Option<String> {
    print!("{}", prompt);
    let _ = stdout().flush();

    let mut user_input: String = String::new();
    match stdin().read_line(&mut user_input) {
        Ok(_) => Some(user_input.trim().to_string()),
        Err(_) => None
    }
}

fn prompt(prompt: &str) -> bool {
    let user_input: String = match input(prompt) {
        Some(input) => input,
        None => return false
    };

    return matches!(user_input.to_lowercase().as_str(), "y" | "ye" | "yes");
}

fn backup(path: &PathBuf, options: &MVOpts) -> Option<String> {
    if path.is_dir() {
        return None;
    }

    let original_file = match path.file_name() {
        Some(name) => name.to_string_lossy(),
        None => return None
    };
    
    let parent = match path.parent() {
        Some(parent) => parent,
        None => return None
    };

    match options.backup_control {
        MVBackup::None => return None,
        MVBackup::Simple => {
            let new_name = format!("{}{}", original_file, options.backup_suffix);
            let mut backup_file = parent.to_path_buf();
            backup_file.push(&new_name);
            match fs::rename(path, &backup_file) {
                Ok(_) => return Some(new_name),
                Err(_) => match fs::copy(path, &backup_file) {
                    Ok(_) => return Some(new_name),
                    Err(_) => return None
                }
            }
        }
        MVBackup::Existing | MVBackup::Numbered => {
            let files = match fs::read_dir(parent) {
                Ok(entries) => entries,
                Err(_) => return None
            };

            let numbered_prefix = format!("{}.~", original_file);
            let numbered_suffix = "~".to_string();

            let simple_name = format!("{}{}", original_file, options.backup_suffix);

            let mut latest_backup: i64 = 0;
            let mut found_simple: bool = false;
            for entry in files {
                match entry {
                    Ok(entry) => {
                        let entry_path = entry.path();
                        let filename = match entry_path.file_name() {
                            Some(name) => name.to_string_lossy(),
                            None => continue
                        };
                        match filename.strip_prefix(&numbered_prefix) {
                            Some(filename) => {
                                match filename.strip_suffix(&numbered_suffix) {
                                    Some(filename) => {
                                        if filename.is_empty() {
                                            continue;
                                        }
                                        let n: i64 = match filename.parse() {
                                            Ok(n) => n,
                                            Err(_) => continue
                                        };
                                        
                                        if n > latest_backup {
                                            latest_backup = n;
                                        }
                                    },
                                    None => continue
                                }
                            },
                            None => {
                                if filename == simple_name {
                                    found_simple = true;
                                }
                            }
                        }
                    },
                    Err(_) => continue
                }
            }

            let mut backup_file = parent.to_path_buf();
            if options.backup_control == MVBackup::Numbered || latest_backup != 0 || found_simple {
                backup_file.push(format!("{}.~{}~", original_file, latest_backup + 1));
            } else {
                backup_file.push(format!("{}{}", original_file, options.backup_suffix));
            }

            let new_name = match backup_file.file_name() {
                Some(name) => name.to_string_lossy().to_string(),
                None => return None
            };

            match fs::rename(path, &backup_file) {
                Ok(_) => return Some(new_name),
                Err(_) => match fs::copy(path, &backup_file) {
                    Ok(_) => return Some(new_name),
                    Err(_) => return None
                }
            }
        }
    };
}

fn mv(options: &MVOpts, paths: &Vec<PathBuf>) {
    if paths.len() < 2 {
        println!("\x1b[0;91mError: Not enough paths supplied, expected at least 2 paths.\x1b[0m");
        return;
    }

    let paths: Vec<PathBuf> = if options.strip_trailing_slashes {
        paths
            .iter()
            .map(|path| path.components().collect())
            .collect()
    } else {
        paths.clone()
    };

    let target = paths.last().unwrap();

    if !target.is_dir() {
        if paths.len() > 2 {
            println!("\x1b[0;91mError: Too many paths supplied for rename operation.\x1b[0m");
        }
        match fs::rename(&paths[0], &paths[1]) {
            Ok(_) => {
                if options.verbose {
                    println!("renamed '{}' -> '{}'",
                        paths[0].to_string_lossy(),
                        paths[1].to_string_lossy());
                }
            },
            Err(_) =>
                println!("\x1b[0;91mError: Failed to rename '{}' to '{}'.\x1b[0m",
                    paths[0].to_string_lossy(),
                    paths[1].to_string_lossy())
        }
        return;
    }
    
    let mut crosses_devices = false;
    for path in paths.iter().take(
        paths.len()
            .saturating_sub(1)
    ) {
        let file_name = match path.file_name() {
            Some(name) => name,
            None => {
                println!("\x1b[0;91mError: Invalid path '{}'.\x1b[0m", path.to_string_lossy());
                continue;
            }
        };
        let destination_path: &PathBuf = &target.join(file_name);
        if destination_path.exists() {
            if options.interactive {
                if !prompt(format!("overwrite '{}'", destination_path.to_string_lossy()).as_str()) {
                    continue;
                }
            } else if options.no_clobber {
                continue;
            }

            if options.backup {
                if let Some(backup_file) = backup(destination_path, options) {
                    if options.verbose {
                        println!("backup '{}' -> '{}'",
                            path.to_string_lossy(),
                            backup_file);
                    }
                }
            }
        }

        if !crosses_devices {
            match fs::rename(path, destination_path) {
                Ok(_) => {
                    if options.verbose {
                        println!("renamed '{}' -> '{}'",
                            path.to_string_lossy(),
                            destination_path.to_string_lossy());
                    }
                    continue;
                },
                Err(err) => match err.kind() {
                    ErrorKind::CrossesDevices => crosses_devices = true,
                    _ => {}
                }
            }
        }

        match fs::copy(path, destination_path) {
            Ok(_) => {
                if options.debug {
                    println!("copied '{}' -> '{}'",
                        path.to_string_lossy(),
                        destination_path.to_string_lossy());
                } else if options.verbose {
                    println!("renamed '{}' -> '{}'",
                        path.to_string_lossy(),
                        destination_path.to_string_lossy());
                }
                match fs::remove_file(path) {
                    Ok(_) => {
                        if options.debug {
                            println!("removed '{}'", path.to_string_lossy());
                        }
                    },
                    Err(_) => println!("\x1b[0;91mError: Failed to remove original file '{}' after copying.\x1b[0m", path.to_string_lossy())
                }
            },
            Err(_) => println!("\x1b[0;91mError: Failed to copy '{}' to '{}'.\x1b[0m", path.to_string_lossy(), destination_path.to_string_lossy())
        }
    }
}

fn main() {
    let mut options: MVOpts = MVOpts {
        backup: false,
        backup_control: MVBackup::Existing,
        backup_suffix: "~".to_string(),
        debug: false,
        interactive: true,
        no_clobber: false,
        strip_trailing_slashes: false,
        verbose: false
    };

    let mut paths: Vec<PathBuf> = Vec::new();

    let args: Vec<String> = env::args().collect();
    let mut expanded_args: Vec<String> = Vec::new();

    match env::var("VERSION_CONTROL") {
        Ok(version) => expanded_args.push(format!("--backup={}", version)),
        Err(_) => {}
    }

    for arg in args.iter().skip(1) {
        if arg.len() > 2 && arg.starts_with('-') && !arg.starts_with("--") {
            for ch in arg.chars().skip(1) {
                expanded_args.push(format!("-{}", ch));
            }
        } else {
            expanded_args.push(arg.to_string());
        }
    }

    for arg in expanded_args {
        match arg.as_str() {
            "-b" | "--backup" => options.backup = true,
            "--debug" => {
                options.debug = true;
                options.verbose = true;
            },
            "-f" | "--force" => options.interactive = false,
            "-i" | "--interactive" => options.interactive = true,
            "-n" | "--no-clobber" => options.no_clobber = true,
            "--strip-trailing-slashes" => options.strip_trailing_slashes = true,
            "--verbose" => options.verbose = true,
            arg if arg.starts_with("--backup=") => {
                let version = match arg.strip_prefix("--backup=") {
                    Some(version) => version.to_lowercase(),
                    None => continue
                };
                match version.as_str() {
                    "t" | "numbered" => options.backup_control = MVBackup::Numbered,
                    "nil" | "existing" => options.backup_control = MVBackup::Existing,
                    "never" | "simple" => options.backup_control = MVBackup::Simple,
                    "none" | "off" => options.backup_control = MVBackup::None,
                    _ => {
                        println!("\x1b[0;91mError: Unknown version control '{}'.\x1b[0m", arg);
                        return;
                    }
                }
            },
            arg if !arg.starts_with('-') =>
                paths.push(PathBuf::from(arg)),
            _ => {
                println!("\x1b[0;91mError: Unknown argument '{}'.\x1b[0m", arg);
                return;
            }
        }
    }

    mv(&options, &paths);
}
