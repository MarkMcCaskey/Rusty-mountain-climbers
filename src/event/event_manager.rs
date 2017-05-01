use event::*;

pub struct EventManager {
	events: Vec<GameEvents>,
	it: i32,
}

impl EventManager {
	pub fn new() -> EventManager {
		EventManager { }
		// construct the EventManager (eventually)
	}

	pub fn push_event(&mut self, game_event: GameEvent) {
		self.events.push(game_event);
	}

	pub fn next(&mut self) -> GameEvent {
		self.it += 1;
		return self.events[self.it - 1];
	}

	pub fn clear(&mut self) {
		self.it = 0;
		self.events.truncate(0);
	}
}