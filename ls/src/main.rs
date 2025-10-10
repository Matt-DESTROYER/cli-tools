use std::{
    env,
    fs::read_dir,
    path::PathBuf
};

fn main() {
    /*
    let args: Vec<String> = std::env::args().collect();

    for arg in args {
        println!("{}", arg);
    }
    */

    let cwd: PathBuf = env::current_dir().unwrap();

    let paths = read_dir(cwd).unwrap();
    for entry in paths {
        let path = entry.unwrap()
            .path();

        let file_name = path
            .file_name().unwrap()
            .to_string_lossy();

        // colour directories intense blue
        if path.is_dir() {
            print!("{}", "\x1b[0;94m");
        }

        // colour executables intense green
        match path.extension() {
            Some(extension) => {
                if extension == "exe" {
                    print!("{}", "\x1b[0;92m");
                }
            },
            None => {}
        }
        
        print!("{} ", file_name);
        // reset colouring
        print!("{}", "\x1b[0m");
    }
}
