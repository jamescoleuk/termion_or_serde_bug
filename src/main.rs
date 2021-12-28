use std::io::stdout;
use termion::raw::IntoRawMode;

fn main() {
    println!("Hello, world!");

    // This causes a failure when run via a Command.
    stdout().into_raw_mode().unwrap();
}
