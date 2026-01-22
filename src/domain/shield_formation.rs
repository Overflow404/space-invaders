use crate::domain::shield::Shield;

const NUMBER_OF_SHIELDS: usize = 4;

pub struct ShieldFormation {
    shields: Vec<Shield>,
}

impl Default for ShieldFormation {
    fn default() -> Self {
        Self::new()
    }
}

impl ShieldFormation {
    pub fn new() -> Self {
        Self {
            shields: (0..NUMBER_OF_SHIELDS).map(|_| Shield::new()).collect(),
        }
    }

    pub(crate) fn get_shields(&self) -> Vec<Shield> {
        self.shields.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_shield_formation() -> ShieldFormation {
        ShieldFormation::new()
    }

    #[test]
    fn new_shield_formation_contains_four_shields() {
        let formation = create_shield_formation();
        assert_eq!(formation.get_shields().len(), 4);
    }
}
