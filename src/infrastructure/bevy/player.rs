use bevy::prelude::*;

use crate::infrastructure::bevy::game_area::{GAME_AREA_HEIGHT, GAME_AREA_WIDTH};
use crate::infrastructure::bevy::projectile::ProjectileMovementTimer;
use crate::{domain::player::Player, infrastructure::bevy::projectile::ProjectileView};

pub const PLAYER_IMAGE: &str = "player-green.png";
const PLAYER_X: f32 = 0.0;
const PLAYER_Y: f32 = -(GAME_AREA_HEIGHT / 2.0) * 0.78;
const PLAYER_WIDTH: f32 = GAME_AREA_WIDTH * 0.045;
const PLAYER_HEIGHT: f32 = GAME_AREA_HEIGHT * 0.043;
const DISTANCE_BETWEEN_PLAYER_AND_PROJECTILE: f32 = 25.0;

#[derive(Resource)]
pub struct PlayerResource(pub Player);

#[derive(Component)]
pub struct PlayerView;

impl Default for PlayerView {
    fn default() -> Self {
        Self::new()
    }
}

impl PlayerView {
    pub fn new() -> Self {
        PlayerView
    }

    pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.spawn((
            PlayerView,
            Sprite {
                image: asset_server.load(PLAYER_IMAGE),
                custom_size: Some(Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT)),
                ..default()
            },
            Transform::from_xyz(PLAYER_X, PLAYER_Y, 0.0),
        ));
    }

    pub fn on_move(
        keyboard: Res<ButtonInput<KeyCode>>,
        mut player_query: Query<&mut Transform, With<PlayerView>>,
        time: Res<Time>,
    ) {
        let speed = 300.0;
        let delta = speed * time.delta_secs();

        for mut transform in player_query.iter_mut() {
            if keyboard.pressed(KeyCode::ArrowLeft) {
                transform.translation.x -= delta;
            }
            if keyboard.pressed(KeyCode::ArrowRight) {
                transform.translation.x += delta;
            }

            let boundary = (GAME_AREA_WIDTH / 2.0) - (PLAYER_WIDTH / 2.0);
            transform.translation.x = transform.translation.x.clamp(-boundary, boundary);
        }
    }

    pub fn on_fire(
        mut commands: Commands,
        keyboard: Res<ButtonInput<KeyCode>>,
        mut player_resource: ResMut<PlayerResource>,
        player_query: Query<&Transform, With<PlayerView>>,
        mut timer: ResMut<ProjectileMovementTimer>,
    ) {
        if keyboard.pressed(KeyCode::Space) && !player_resource.0.is_firing() {
            for transform in player_query.iter() {
                let translation = transform.translation;

                let projectile_view = ProjectileView::new(
                    translation.x,
                    translation.y + DISTANCE_BETWEEN_PLAYER_AND_PROJECTILE,
                );

                commands.spawn(projectile_view.spawn_projectile());

                player_resource.0.toggle_fire();
                timer.0.reset();
            }
        }
    }
}
