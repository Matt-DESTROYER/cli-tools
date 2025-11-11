use std::{
    env,
    path
};

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

fn rm(options: &RMOpts, paths: &Vec<String>) {
}

fn main() {
    let mut options: RMOpts = RMOpts {
        force: false,
        prompt: RMPrompting::Always,
        recursive: false,
        directories: false,
        verbose: false,
        help: false,
        version: false
    };

    let mut paths: Vec<String> = Vec::new();

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
            "--interactive=always" => options.prompt = RMPrompting::Always,
            "-r" | "-R" | "--recursive" => options.recursive = true,
            "-d" | "--dir" => options.directories = true,
            "-v" | "--verbose" => options.verbose = true,
            "--help" => options.help = true,
            "--version" => options.version = true,
            arg if !arg.starts_with('-') && !arg.starts_with("--") =>
                paths.append(&mut vec![arg.to_string()]),
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
