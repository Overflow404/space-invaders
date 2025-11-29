use crate::domain::shield_formation::ShieldFormation;
use bevy::prelude::Resource;

#[derive(Resource)]
pub struct ShieldFormationResource(pub ShieldFormation);
