use crate::infrastructure::bevy::player::{PlayerResource, PlayerView};
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

pub const PROJECTILE_TIME_IN_SECONDS: f32 = 1.5;

#[derive(Resource)]
pub struct ProjectileMovementTimer(pub Timer);

#[derive(Component)]
pub struct Projectile;

pub struct ProjectileView {
    start_pos: Vec3,
}

#[derive(SystemParam)]
pub struct FireContext<'w, 's> {
    pub commands: Commands<'w, 's>,
    pub time: Res<'w, Time>,
    pub keyboard: Res<'w, ButtonInput<KeyCode>>,
    pub player_res: ResMut<'w, PlayerResource>,
    pub timer: ResMut<'w, ProjectileMovementTimer>,
    pub player_query: Query<'w, 's, &'static Transform, (With<PlayerView>, Without<Projectile>)>,
    pub projectile_query:
        Query<'w, 's, &'static mut Transform, (With<Projectile>, Without<PlayerView>)>,
}

impl ProjectileView {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            start_pos: Vec3::new(x, y, 0.0),
        }
    }

    pub fn spawn_projectile(&self) -> (Projectile, Sprite, Transform) {
        (
            Projectile,
            Sprite {
                color: Color::srgb_u8(190, 12, 12),
                custom_size: Some(Vec2::new(5.0, 15.0)),
                ..default()
            },
            Transform::from_translation(self.start_pos),
        )
    }
}
