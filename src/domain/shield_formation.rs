use crate::domain::shield::Shield;

pub struct ShieldFormation {
    shields: Vec<Shield>,
}

impl ShieldFormation {
    pub fn new() -> Self {
        Self {
            shields: vec![Shield::new(), Shield::new(), Shield::new(), Shield::new()],
        }
    }

    pub(crate) fn get_shields(&self) -> Vec<Shield> {
        self.shields.clone()
    }
}
