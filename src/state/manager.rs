use state::*;

struct StateManager {
    states: Vec<States>,
}

impl StateManager {
    pub fn new() -> StateManager {
        StateManager { states: vec![] }
    }

    pub fn push_state(&mut self, state: States) {
        self.states.push(state);
    }

    pub fn pop_state(&mut self) {
        self.states.pop();
    }

    pub fn handle_events(&mut self) {
        use std::ops::Index;

        if self.states.is_empty() {
            () //do nothing
        } else {
            self.states[0].handle_events();
        }
    }
}
