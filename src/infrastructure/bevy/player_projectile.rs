use crate::infrastructure::bevy::game_area::GAME_AREA_HEIGHT;
use crate::infrastructure::bevy::player::PlayerResource;
use bevy::prelude::*;

pub const PROJECTILE_SPEED: f32 = 500.0;
pub const PROJECTILE_DURATION: f32 = 1.2;
const PROJECTILE_WIDTH: f32 = 5.0;
const PROJECTILE_HEIGHT: f32 = 15.0;

#[derive(Resource)]
pub struct PlayerProjectileMovementTimer(pub Timer);

#[derive(Component)]
pub struct PlayerProjectileView {
    start_position: Vec3,
}
impl PlayerProjectileView {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            start_position: Vec3::new(x, y, 0.0),
        }
    }

    pub fn spawn_projectile(&self) -> (PlayerProjectileView, Sprite, Transform) {
        (
            PlayerProjectileView {
                start_position: self.start_position,
            },
            Sprite {
                color: Color::srgb(1.0, 1.0, 1.0),
                custom_size: Some(Vec2::new(PROJECTILE_WIDTH, PROJECTILE_HEIGHT)),
                ..default()
            },
            Transform::from_translation(self.start_position),
        )
    }

    pub fn on_move(time: Res<Time>, mut query: Query<&mut Transform, With<PlayerProjectileView>>) {
        for mut transform in query.iter_mut() {
            transform.translation.y += PROJECTILE_SPEED * time.delta_secs();
        }
    }

    pub fn on_destroy(
        mut commands: Commands,
        time: Res<Time>,
        mut timer: ResMut<PlayerProjectileMovementTimer>,
        mut player_resource: ResMut<PlayerResource>,
        query: Query<(Entity, &Transform), With<PlayerProjectileView>>,
    ) {
        if !player_resource.0.is_firing() {
            return;
        }

        timer.0.tick(time.delta());
        let top_bound = GAME_AREA_HEIGHT / 2.0;
        let mut reset_needed = false;

        for (entity, transform) in query.iter() {
            if transform.translation.y > top_bound {
                commands.entity(entity).despawn();
                reset_needed = true;
            }
        }

        if timer.0.just_finished() {
            for (entity, _) in query.iter() {
                commands.entity(entity).despawn();
            }
            reset_needed = true;
        }

        if reset_needed {
            player_resource.0.toggle_fire();
            timer.0.reset();
        }
    }
}
