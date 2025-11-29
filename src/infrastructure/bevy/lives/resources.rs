use crate::domain::lives::Lives;
use bevy::prelude::Resource;

#[derive(Resource)]
pub struct LivesResource(pub Lives);
