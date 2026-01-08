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

struct MVOpts {
    debug: bool,
    interactive: bool,
    no_clobber: bool,
    strip_trailing_slashes: bool,
    verbose: bool
}

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    match stdout().flush() {
        Ok(_) => {},
        Err(_) => {}
    }
    let mut user_input: String = String::new();
    match stdin().read_line(&mut user_input) {
        Ok(_) => {},
        Err(_) => {}
    }
    // TODO: find better method than `.trim()` as this would strip intentional spaces in addition to the `\n` (not really an issue here, but unintuitive)
    return user_input.trim().to_string();
}

fn prompt(prompt: &str) -> bool {
    let user_input: String = input(prompt);

    return match user_input.to_lowercase().as_str() {
        "y" | "ye" | "yes" => true,
        _ => false
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
        let destination_path: &PathBuf = &target.join(path.file_name().expect("Invalid file name."));
        if destination_path.exists() {
            if options.interactive {
                if !prompt(format!("overwrite '{}'", destination_path.to_string_lossy()).as_str()) {
                    continue;
                }
            } else if options.no_clobber {
                continue;
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
                            println!("removed '{}", path.to_string_lossy());
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
        debug: false,
        interactive: true,
        no_clobber: false,
        strip_trailing_slashes: false,
        verbose: false
    };

    let mut paths: Vec<PathBuf> = Vec::new();

    let args: Vec<String> = env::args().collect();
    let mut expanded_args: Vec<String> = Vec::new();
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
            "--debug" => {
                options.debug = true;
                options.verbose = true;
            },
            "-f" | "--force" => options.interactive = false,
            "-i" | "--interactive" => options.interactive = true,
            "-n" | "--no-clobber" => options.no_clobber = true,
            "--strip-trailing-slashes" => options.strip_trailing_slashes = true,
            "--verbose" => options.verbose = true,
            arg if !arg.starts_with('-') && !arg.starts_with("--") =>
                paths.push(PathBuf::from(arg)),
            _ => {
                println!("\x1b[0;91mError: Unknown argument '{}'.\x1b[0m", arg);
                return;
            }
        }
    }

    mv(&options, &paths);
}
