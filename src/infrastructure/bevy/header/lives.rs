use bevy::ecs::component::Component;

use crate::domain::lives::Lives;

#[derive(Component)]
pub struct LivesComponent(pub Lives);

#[derive(Component)]
pub struct LivesView;
