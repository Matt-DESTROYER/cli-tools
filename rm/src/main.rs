use std::{
    env,
    fs,
    io::{
        Result,
        stdin,
        stdout,
        Write
    },
    path::{
        Path,
        PathBuf
    }
};

pub trait PathExt {
    fn is_empty(&self) -> Result<bool>;
}

impl PathExt for Path {
    fn is_empty(&self) -> Result<bool> {
        match fs::metadata(self) {
            Ok(metadata) => {
                if !metadata.is_dir() {
                    return Ok(false);
                }
            },
            Err(err) => return Err(err)
        }
        let mut files = match fs::read_dir(self) {
            Ok(entries) => entries,
            Err(err) => return Err(err)
        };
        return Ok(files.next().is_none());
    }
}

#[derive(PartialEq)]
enum RMPrompting {
    Never,
    Once,
    Always
}

struct RMOpts {
    force: bool,
    prompt: RMPrompting,
    recursive: bool,
    directories: bool,
    verbose: bool,
    help: bool,
    version: bool
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

fn rm(options: &RMOpts, paths: &Vec<PathBuf>) {
    for path in paths {
        if !path.exists() {
            println!("\x1b[0;91mrm: cannot remove '{}': No such file or directory\x1b[0m",
                path.to_string_lossy());
            continue;
        }

        if path.is_dir() {
            if options.prompt == RMPrompting::Always &&
                !prompt(
                    format!("rm: remove directory '{}'? ",
                        path.to_string_lossy()).as_str()) {
                continue;
            }
            if options.recursive {
                match fs::remove_dir_all(path) {
                    Ok(_) => continue,
                    Err(_) => println!("\x1b[0;91mrm: cannot remove '{}': an unexpected error occurred\x1b[0m",
                        path.to_string_lossy())
                }
            } else if options.directories == true {
                match path.is_empty() {
                    Ok(_) => {},
                    Err(_) => continue
                }
                match fs::remove_dir(path) {
                    Ok(_) => {
                        if options.verbose {
                            println!("removed empty directory '{}'", path.to_string_lossy());
                        }
                    },
                    Err(_) => continue
                }
            }
        } else if path.is_file() {
            if options.prompt == RMPrompting::Always &&
                !prompt(
                    format!("rm: remove regular file '{}'? ",
                        path.to_string_lossy()).as_str()) {
                continue;
            }
            match fs::remove_file(path) {
                Ok(_) => {
                    if options.verbose {
                        println!("removed '{}'", path.to_string_lossy());
                    }
                },
                Err(_) => {}
            }
        } else {
            println!("\x1b[0;91mrm: cannot remove '{}': No such file or directory\x1b[0m",
                path.to_string_lossy());
        }
    }
}

fn main() {
    let mut options: RMOpts = RMOpts {
        force: false,
        prompt: RMPrompting::Never,
        recursive: false,
        directories: false,
        verbose: false,
        help: false,
        version: false
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
            "-f" | "--force" => options.force = true,
            "-i" => options.prompt = RMPrompting::Always,
            "-I" => options.prompt = RMPrompting::Once,
            "--interactive=never" => options.prompt = RMPrompting::Never,
            "--interactive=once" => options.prompt = RMPrompting::Once,
            "--interactive" | "--interactive=always" => options.prompt = RMPrompting::Always,
            "-r" | "-R" | "--recursive" => options.recursive = true,
            "-d" | "--dir" => options.directories = true,
            "-v" | "--verbose" => options.verbose = true,
            "--help" => options.help = true,
            "--version" => options.version = true,
            arg if !arg.starts_with('-') && !arg.starts_with("--") =>
                paths.push(PathBuf::from(arg.to_string())),
            _ => {
                println!("\x1b[0;91mError: Unknown argument '{}'.\x1b[0m", arg);
                return;
            }
        }
    }

    if paths.len() == 0 {
        println!("\x1b[0;91mError: No paths supplied.\x1b[0m");
        return;
    }
    
    rm(&options, &paths);
}
