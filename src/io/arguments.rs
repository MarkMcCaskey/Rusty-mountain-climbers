//! Command line arguments
//! Used to change behavior when launching

use clap::{Arg, App, ArgMatches};

// Parses command line arguments
pub fn read_arguments<'input>() -> ArgMatches<'input> {
    App::new("mountain-climbers")
        .version("0.1")
        .author("Mark McCaskey and Crocodoctopus")
        .about("A small game")
        .arg(Arg::with_name("trace")
                 .short("t")
                 .long("trace")
                 .help("Runs with verbose log messages")
                 .takes_value(false))
        .get_matches()
}
