#[cfg(windows)]
use std::process::Command;

fn main() {
    #[cfg(windows)] {
        match Command::new("cmd")
                .args(["/c", "cls"])
                .status() {
            Ok(_) => return,
            Err(_) => {}
        }
    }
    #[cfg(not(windows))]
    print!("\x1B[2J\x1B[1;1H");
}
