use std::{
    env,
    fs,
    path::Path
};

fn main() {
    let mut files: Vec<String> = Vec::new();

    let args: Vec<String> = env::args().collect();
    for arg in args.iter().skip(1) {
        match arg.as_str() {
            arg if arg.starts_with("--") => {},
            arg if arg.starts_with('-') => {},
            arg => files.push(arg.to_string())
        }
    }

    for file in files {
        let path: &Path = Path::new(&file);
        if !path.exists() {
            println!("\x1b[0;91mError: Path not found '{}'.\x1b[0m", path.file_name().unwrap().to_string_lossy());
            continue;
        }
        let content = fs::read_to_string(file).unwrap();
        println!("{}", content);
    }
}
