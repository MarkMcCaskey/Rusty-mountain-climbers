pub mod manager;
pub mod intro;

trait State {
    fn init(&mut self);
    fn cleanup(&mut self);
    fn pause(&mut self);
    fn resume(&mut self);
    fn handle_events(&mut self);
    fn update(&mut self);
    fn draw(&mut self);
}

enum States {
    Intro,
    Outro,
    Generic,
}


impl State for States {
    fn init(&mut self) {}
    fn cleanup(&mut self) {}
    fn pause(&mut self) {}
    fn resume(&mut self) {}
    fn handle_events(&mut self) {}
    fn update(&mut self) {}
    fn draw(&mut self) {}
}
