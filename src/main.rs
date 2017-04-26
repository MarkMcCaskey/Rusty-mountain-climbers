extern crate piston_window;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate clap;
extern crate time;

use piston_window::*;
use log4rs::*;
use log::LogLevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};
use time::{Duration, PreciseTime};

mod io;
mod game;
mod state;

const DT_TIME_US: i64 = ((1.0 * 1000.0 * 1000.0) / 60.0) as i64;

fn main() {
    let dt = Duration::microseconds(DT_TIME_US);

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

    let mut render = false;
    let mut last_time = PreciseTime::now();
    let mut accumulated_time = Duration::zero();
    loop {
        accumulated_time = accumulated_time + last_time.to(PreciseTime::now());
        last_time = PreciseTime::now();

        while accumulated_time.num_microseconds() > dt.num_microseconds() {
            update();
            accumulated_time = accumulated_time - dt;
            render = true;
        }

        if render {
            render_game();
            render = false;
        }
    }
    /*
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
    }*/
}

fn update() {}

fn render_game() {}
