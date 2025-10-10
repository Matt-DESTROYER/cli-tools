use std::env;

fn main() {
    match env::var("TERM").unwrap_or_default().as_str() {
        "xterm" | "color" => print!("\x1B[2J\x1B[1;1H"),
        _ => {}
    };
}
