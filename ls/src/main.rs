use std::{
    env,
    fs,
    path::PathBuf
};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

fn main() {
    let cwd: PathBuf = env::current_dir().unwrap();
    let mut all: bool = false;
    let mut comma_separated: bool = false;
    let mut quote_name: bool = false;

    let mut paths: Vec<PathBuf> = fs::read_dir(cwd).unwrap()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .collect();

    let args: Vec<String> = std::env::args().collect();
    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "-a" | "--all" => all = true,
            "-m" => comma_separated = true,
            "-r" | "--reverse" => paths.reverse(),
            "--group-directories-first" => paths
                .sort_by(|a, b| b.is_dir().cmp(&a.is_dir())),
            "-Q" | "--quote-name" => quote_name = true,
            _ => {
                print!("{}", "\x1b[0;30m"); // red
                println!("Error: Unknown argument '{}'", arg);
                print!("{}", "\x1b[0m"); // reset colouring
                return;
            }
        }
    }

    let mut paths_iterator = paths.iter().peekable();
    while let Some(path) = paths_iterator.next() {
        let file_name = path
            .file_name().unwrap()
            .to_string_lossy();

        if file_name.starts_with(".") && !all {
            continue;
        }

        // unix file colouring
        #[cfg(unix)] {
            let metadata = fs::metadata(&path);
            if metadata.is_dir() {
                print!("{}", "\x1b[0;94m"); // intense blue
            } else {
                match metadata {
                    Ok(metadata) => {
                        if metadata.permissions().mode() & 0o111 != 0 {
                            print!("{}", "\x1b[0;92m"); // intense green
                        }
                    },
                    Err(_) => {}
                }
            }
        }

        // non-unix file colouring
        #[cfg(not(unix))] {
            if path.is_dir() {
                print!("{}", "\x1b[0;94m"); // intense blue
            }
            match path.extension() {
                Some(extension) => {
                    if extension == "exe" {
                        print!("{}", "\x1b[0;92m"); // intense green
                    }
                },
                None => {}
            }
        }
        
        if quote_name {
            print!("\"");
        }
        print!("{}", file_name);
        if quote_name {
            print!("\"");
        }
        print!("{}", "\x1b[0m"); // reset colouring
        if comma_separated && paths_iterator.peek().is_some() {
            print!(", ");
        } else {
            print!(" ");
        }
    }

    print!("\n");
}
