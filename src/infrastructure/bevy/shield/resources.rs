use crate::infrastructure::bevy::game_area::resources::{GAME_AREA_HEIGHT, GAME_AREA_WIDTH};
use crate::infrastructure::bevy::shield::resources::ShieldPart::{
    Empty, Full, InnerLeft, InnerRight, OuterLeft, OuterRight,
};

pub const SHIELD_WIDTH: f32 = GAME_AREA_WIDTH * 0.09;
pub const SHIELD_HEIGHT: f32 = GAME_AREA_HEIGHT * 0.11;
pub const SHIELD_Y: f32 = -(GAME_AREA_HEIGHT / 2.0) * 0.58;
pub const SHIELD_X: f32 = -(GAME_AREA_WIDTH / 2.0) * 0.68;

pub const SHIELD_PART_ROWS: usize = 3;
pub const SHIELD_PART_COLUMNS: usize = 4;

pub const SHIELD_PART_SIZE: usize = 6;
#[derive(PartialEq)]
pub enum ShieldPart {
    OuterLeft,
    OuterRight,
    Full,
    InnerLeft,
    InnerRight,
    Empty,
}
pub const SHIELD_LAYOUT: [[ShieldPart; SHIELD_PART_COLUMNS]; SHIELD_PART_ROWS] = [
    [OuterLeft, Full, Full, OuterRight],
    [Full, InnerLeft, InnerRight, Full],
    [Full, Empty, Empty, Full],
];
