pub struct Player {
    is_firing: bool,
}

impl Player {
    pub fn new() -> Self {
        Player { is_firing: false }
    }

    pub fn toggle_fire(&mut self) {
        self.is_firing = !self.is_firing
    }

    pub fn is_firing(&self) -> bool {
        self.is_firing
    }
}
