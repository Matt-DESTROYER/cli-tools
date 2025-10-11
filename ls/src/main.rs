use std::{
    env,
    fs::{read_dir, metadata, Metadata},
    path::{
        Path,
        PathBuf
    }
};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

struct LSOpts {
    all: bool,
    comma_separated: bool,
    reverse: bool,
    group_directories_first: bool,
    recursive: bool,
    quote_name: bool
}

fn ls(directory: &Path, options: &LSOpts) {
    if !directory.exists() {
        println!("\x1b[0;91mError: Path not found '{}'.\x1b[0m", directory.to_string_lossy());
        return;
    }

    let mut paths: Vec<PathBuf> = read_dir(directory).unwrap()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .collect();

    if options.group_directories_first {
        paths.sort_by(|a, b| {
            b.is_dir()
                .cmp(&a.is_dir())
                .then_with(|| a.cmp(b))
        });
    } else {
        paths.sort();
    }

    let mut to_recurse: Vec<PathBuf> = Vec::new();

    for (i, path) in paths.iter().enumerate() {
        let file_name = path
            .file_name().unwrap()
            .to_string_lossy();

        if !path.exists() {
            println!("\x1b[0;91mError: Path not found '{}'.\x1b[0m", file_name);
            continue;
        }

        if !options.all && file_name.starts_with(".") {
            continue;
        }

        let metadata: Metadata = metadata(path).unwrap();
        if metadata.is_dir() {
            if options.recursive {
                to_recurse.append(&mut vec![path.clone()]);
            }
            print!("{}", "\x1b[0;94m"); // intense blue
        }

        #[cfg(unix)] {
            if !metadata.is_dir() {
                if metadata.permissions().mode() & 0o111 != 0 {
                    print!("{}", "\x1b[0;92m"); // intense green
                }
            }
        }
        #[cfg(not(unix))] {
            match path.extension() {
                Some(extension) => {
                    if extension == "exe" {
                        print!("{}", "\x1b[0;92m"); // intense green
                    }
                },
                None => {}
            }
        }
        
        if options.quote_name {
            print!("\"");
        }
        print!("{}", file_name);
        if options.quote_name {
            print!("\"");
        }
        print!("{}", "\x1b[0m"); // reset colouring

        if options.comma_separated && i != paths.len() - 1 {
            print!(", ");
        } else {
            print!(" ");
        }
    }

    if to_recurse.len() > 0 {
        print!("\n\n");
    }
    for (i, path) in to_recurse.iter().enumerate() {
        let path_name = path.to_string_lossy();

        if !path.exists() {
            println!("\x1b[0;91mError: Path not found '{}'.\x1b[0m", path_name);
            continue;
        }

        println!("{}:", path_name);
        ls(&path, options);

        if i != to_recurse.len() - 1 {
            print!("\n\n");
        }
    }
}

fn main() {
    let mut options = LSOpts {
        all: false,
        comma_separated: false,
        reverse: false,
        group_directories_first: false,
        recursive: false,
        quote_name: false
    };

    let mut directories: Vec<String> = Vec::new();

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
            "-a" | "--all" => options.all = true,
            "-m" => options.comma_separated = true,
            "-r" | "--reverse" => options.reverse = true,
            "--group-directories-first" =>
                options.group_directories_first = true,
            "-R" | "--recursive" => options.recursive = true,
            "-Q" | "--quote-name" => options.quote_name = true,
            arg if !arg.starts_with('-') && !arg.starts_with("--") =>
                directories.append(&mut vec![arg.to_string()]),
            _ => {
                println!("\x1b[0;91mError: Unknown argument '{}'.\x1b[0m", arg);
                return;
            }
        }
    }

    if directories.len() == 0 {
        ls(&Path::new("./"), &options);
    } else if directories.len() == 1 {
        ls(&Path::new(&directories[0]), &options);
    } else {
        for (i, directory) in directories.iter().enumerate() {
            let path: &Path = Path::new(&directory);
            if !path.exists() {
                println!("\x1b[0;91mError: Path not found '{}'.\x1b[0m", path.to_string_lossy());
                if i != directories.len() - 1 {
                    print!("\n");
                }
                continue;
            }

            println!("{}:", directory);
            ls(path, &options);
            
            if i != directories.len() - 1 {
                print!("\n\n");
            }
        }
    }
}
