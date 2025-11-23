use crate::infrastructure::bevy::player::{PlayerResource, PlayerView};
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

pub const PROJECTILE_SPEED: f32 = 500.0;
pub const PROJECTILE_DURATION: f32 = 1.2;
const PROJECTILE_WIDTH: f32 = 5.0;
const PROJECTILE_HEIGHT: f32 = 15.0;

#[derive(Resource)]
pub struct ProjectileMovementTimer(pub Timer);

#[derive(Component)]
pub struct ProjectileView {
    start_position: Vec3,
}

type ProjectileFilter = (With<ProjectileView>, Without<PlayerView>);
type ProjectileData = (Entity, &'static mut Transform);
type PlayerFilter = (With<PlayerView>, Without<ProjectileView>);
type PlayerData = &'static Transform;

#[derive(SystemParam)]
pub struct FireContext<'w, 's> {
    pub commands: Commands<'w, 's>,
    pub time: Res<'w, Time>,
    pub timer: ResMut<'w, ProjectileMovementTimer>,
    pub keyboard: Res<'w, ButtonInput<KeyCode>>,
    pub player_resource: ResMut<'w, PlayerResource>,
    pub player_query: Query<'w, 's, PlayerData, PlayerFilter>,
    pub projectile_query: Query<'w, 's, ProjectileData, ProjectileFilter>,
}

impl ProjectileView {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            start_position: Vec3::new(x, y, 0.0),
        }
    }

    pub fn spawn_projectile(&self) -> (ProjectileView, Sprite, Transform) {
        (
            ProjectileView::new(0.0, 0.0),
            Sprite {
                color: Color::srgb(255.0, 255.0, 255.0),
                custom_size: Some(Vec2::new(PROJECTILE_WIDTH, PROJECTILE_HEIGHT)),
                ..default()
            },
            Transform::from_translation(self.start_position),
        )
    }
}
