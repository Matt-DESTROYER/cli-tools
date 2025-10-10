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

fn ls(dir: &PathBuf, options: &LSOpts) {
    let mut paths: Vec<PathBuf> = read_dir(dir).unwrap()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .collect();
    paths.sort();

    let mut to_recurse: Vec<PathBuf> = Vec::new();

    for (i, path) in paths.iter().enumerate() {
        let file_name = path
            .file_name().unwrap()
            .to_string_lossy();

        if file_name.starts_with(".") && !options.all {
            continue;
        }

        let metadata: Metadata = metadata(path).unwrap();
        if options.recursive && metadata.is_dir() {
            to_recurse.append(&mut vec![path.clone()]);
        }

        // unix specific file colouring
        #[cfg(unix)] {
            if !metadata.is_dir() {
                if metadata.permissions().mode() & 0o111 != 0 {
                    print!("{}", "\x1b[0;92m"); // intense green
                }
            }
        }

        // non-unix file colouring
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
        if options.comma_separated && i == paths.len() - 1 {
            print!(", ");
        } else {
            print!(" ");
        }
    }

    if paths.len() != 0 {
        print!("\n");
    }

    if to_recurse.len() > 0 {
        print!("\n");
    }
    for (i, path) in to_recurse.iter().enumerate() {
        let path_name = path.to_string_lossy();
        
        println!("{}:", path_name);
        ls(&path, options);

        if i != to_recurse.len() - 1 {
            print!("\n");
        }
    }
}

fn main() {
    //let cwd: PathBuf = env::current_dir().unwrap();

    let mut options = LSOpts {
        all: false,
        comma_separated: false,
        reverse: false,
        group_directories_first: false,
        recursive: false,
        quote_name: false
    };

    let args: Vec<String> = env::args().collect();
    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "-a" | "--all" => options.all = true,
            "-m" => options.comma_separated = true,
            "-r" | "--reverse" => options.reverse = true,
            "--group-directories-first" => options.group_directories_first = true,
            "-R" | "--recursive" => options.recursive = true,
            "-Q" | "--quote-name" => options.quote_name = true,
            _ => {
                print!("{}", "\x1b[0;30m"); // red
                println!("Error: Unknown argument '{}'", arg);
                print!("{}", "\x1b[0m"); // reset colouring
                return;
            }
        }
    }

    println!("recursive: {}", options.recursive);

    ls(&Path::new("./").to_path_buf(), &options);
}
