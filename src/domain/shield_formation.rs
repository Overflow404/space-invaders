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
mod test {

    use super::*;

    #[test]
    fn should_create_the_shield_formation() {
        let formation = ShieldFormation::new();
        assert_eq!(formation.get_shields().len(), NUMBER_OF_SHIELDS);
    }
}
