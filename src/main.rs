extern crate piston_window;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate clap;
extern crate time;
extern crate tokio_core;
extern crate tokio;
extern crate futures;

use time::{Duration, PreciseTime};

mod io;
mod game;
mod state;
mod job_manager;

use game::*;

const DT_TIME_US: i64 = ((1.0 * 1000.0 * 1000.0) / 60.0) as i64;

fn main() {
    let dt = Duration::microseconds(DT_TIME_US);

    let mut game = Game::new();

    let mut render = false;
    let mut last_time = PreciseTime::now();
    let mut accumulated_time = Duration::zero();
    loop {
        accumulated_time = accumulated_time + last_time.to(PreciseTime::now());
        last_time = PreciseTime::now();

        while accumulated_time.num_microseconds() > dt.num_microseconds() {
            game.update();
            accumulated_time = accumulated_time - dt;
            render = true;
        }

        if render {
            game.render();
            render = false;
        }
    }

}
