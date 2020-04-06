use clap::{App, Arg};

mod crab;

fn main() {
    let matches = App::new("crabsay")
                    .version("0.1.0")
                    .arg(Arg::with_name("text")
                            .index(1)
                            .required(true)
                            .help("input string"))
                    .get_matches();

    let text = matches.value_of("text").unwrap();

    println!("{}", crab::format(text));
}
