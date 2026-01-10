use std::io::{self, Write};

#[cfg(windows)]
use std::process::Command;

fn main() {
    #[cfg(windows)] {
        match Command::new("cmd")
                .args(["/c", "cls"])
                .status() {
            Ok(status) => {
                if status.success() {
                    return;
                }
            },
            Err(_) => {}
        }
    }

    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().ok();
}
