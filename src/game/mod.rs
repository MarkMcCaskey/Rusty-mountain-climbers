use log4rs::*;
use log::LogLevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};
use piston_window::*;

use io;
use state::manager::*;

pub struct Game {
    state_manager: StateManager,
    logger: Config,
    window: PistonWindow,
    debug: bool,
}


impl Game {
    pub fn new() -> Game {

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

        Game {
            state_manager: StateManager::new(),
            logger: config,
            window: window,
            debug: arguments.is_present("debug"),
        }
    }

    pub fn update(&mut self) {
        self.state_manager.handle_events();
    }

    pub fn render(&mut self) {
        while let Some(e) = self.window.next() {
            self.window.draw_2d(&e, |c, g| {
                clear([1.0; 4], g);
                rectangle([1.0, 0.0, 0.0, 1.0], // red
                          [0.0, 0.0, 100.0, 100.0],
                          c.transform,
                          g);
            });
        }
    }
}
