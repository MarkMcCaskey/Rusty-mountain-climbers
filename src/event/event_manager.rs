use event::*;

pub struct EventManager {
    events: Vec<GameEvent>,
    it: i32,
}

impl EventManager {
    pub fn new() -> EventManager {
        EventManager {}
        // construct the EventManager (eventually)
    }

    pub fn push_event(&mut self, game_event: GameEvent) {
        self.events.push(game_event);
    }

    pub fn clear(&mut self) {
        self.it = 0;
        self.events.truncate(0);
    }
}

impl Iterator for EventManager {
    type Item = GameEvent;

    fn next(&mut self) -> Option<GameEvent> {
        self.it += 1;

        if self.events.len() >= self.it {
            let index = (self.it - 1) as usize;
            Some(self.events[index])
        } else {
            self.it = 0;
            None
        }
    }
}
