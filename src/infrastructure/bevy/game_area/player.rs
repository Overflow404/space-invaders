use bevy::ecs::component::Component;

use crate::domain::player::Player;

#[derive(Component)]
pub struct PlayerComponent(pub Player);

#[derive(Component)]
pub struct PlayerView;
