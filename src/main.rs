extern crate piston_window;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate clap;

use piston_window::*;
use log4rs::*;
use log::LogLevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};

mod io;
mod game;

use io::arguments;

fn main() {
    let arguments = io::arguments::read_arguments();

    let trace_mode = arguments.is_present("trace");

    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{h({l})} {m} {n}")))
        .build();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(if trace_mode {
                                                            LogLevelFilter::Trace
                                                        } else {
                                                            LogLevelFilter::Info
                                                        }))
        .unwrap();

    // these values should come from configuration
    let mut window: PistonWindow =
        WindowSettings::new("Mountain Climbers", [640, 480]).exit_on_esc(true).build().unwrap();
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([1.0; 4], g);
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [0.0, 0.0, 100.0, 100.0],
                      c.transform,
                      g);
        });
    }
}
