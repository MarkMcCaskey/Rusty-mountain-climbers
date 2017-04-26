use state::*;

struct IntroState {}

impl State for IntroState {
    fn init(&mut self) {}
    fn cleanup(&mut self) {}
    fn pause(&mut self) {}
    fn resume(&mut self) {}
    fn handle_events(&mut self) {}
    fn update(&mut self) {}
    fn draw(&mut self) {}
}
