use piston_window::keyboard::Key;

pub struct Control {
    state: bool,
    k: Key,
}

impl Control {
    pub fn set(&mut self, val: bool) {
        self.state = val;
    }
    pub fn get(&self) -> bool {
        self.state
    }
    pub fn new(k: Key) -> Control {
        Control {
            state: false,
            k,
        }
    }
    pub fn key(&self) -> Key {
        self.k
    }
}

