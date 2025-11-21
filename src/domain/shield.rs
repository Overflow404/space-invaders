#[derive(Clone)]
pub struct Shield {}

impl Default for Shield {
    fn default() -> Self {
        Self::new()
    }
}

impl Shield {
    pub fn new() -> Self {
        Self {}
    }
}
