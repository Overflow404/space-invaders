#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeaponState {
    Ready,
    Firing,
}

pub trait Fireable {
    fn start_firing(&mut self);
    fn reload(&mut self);
    fn can_fire(&self) -> bool;
    fn is_firing(&self) -> bool;
    fn toggle_fire(&mut self);
}
