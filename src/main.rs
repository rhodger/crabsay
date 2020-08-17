//! Main file for handling user input.
//!
//! All other logic is handled by other files.

use clap::{App, Arg};

mod crab;

/// User input is handled using `clap`, which also generates help messages and
/// version information.
fn main() {
    let matches = App::new("crabsay")
                    .version("1.1.0")
                    .arg(Arg::with_name("text")
                            .index(1)
                            .required(true)
                            .help("input string"))
                    .arg(Arg::with_name("length")
                            .required(false)
                            .short("l")
                            .long("length")
                            .default_value("40")
                            .help("line length"))
                    .get_matches();

    let text = matches.value_of("text").unwrap();

    // Here the maximum length of each line of text is defined - it can be
    // specified at the command-line by the user or left to default to 40.
    let length: u8 = match matches.value_of("length") {
        Some(x) => x.parse().unwrap_or(40),
        None => 40
    };

    // This calls the necessary function from `crab.rs` to format the input
    // correctly, the prints it out.
    println!("{}", crab::format(text, length));
}
