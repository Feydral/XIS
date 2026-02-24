use std::collections::HashSet;
use std::time::Duration;

pub struct Input {
    // held_keys: currently pressed keys
    // down_keys: keys that were pressed this frame
    // up_keys: keys that were released this frame
}

impl Input {
    pub fn new() -> Self {
        Self {
            // held_keys: HashSet::new(),
            // down_keys: HashSet::new(),
            // up_keys: HashSet::new(),
        }
    }

    pub fn update(&mut self) {

    }

    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.held_keys.contains(&key)
    }

    pub fn is_key_down(&self, key: KeyCode) -> bool {
        self.down_keys.contains(&key)
    }

    pub fn is_key_up(&self, key: KeyCode) -> bool {
        self.up_keys.contains(&key)
    }
}
