use bevy::prelude::*;

use crate::infrastructure::bevy::game_area::{GAME_AREA_HEIGHT, GAME_AREA_WIDTH};
use crate::infrastructure::bevy::projectile::PROJECTILE_SPEED;
use crate::{
    domain::player::Player,
    infrastructure::bevy::projectile::{FireContext, ProjectileView},
};

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

    pub fn on_fire(mut ctx: FireContext) {
        if ctx.keyboard.pressed(KeyCode::Space) && !ctx.player_res.0.is_firing() {
            for player_transform in ctx.player_query.iter() {
                let player_pos = player_transform.translation;

                let projectile_view = ProjectileView::new(
                    player_pos.x,
                    player_pos.y + DISTANCE_BETWEEN_PLAYER_AND_PROJECTILE,
                );

                ctx.commands.spawn(projectile_view.spawn_projectile());

                ctx.player_res.0.toggle_fire();
            }
        }

        if ctx.player_res.0.is_firing() {
            ctx.timer.0.tick(ctx.time.delta());

            for mut transform in ctx.projectile_query.iter_mut() {
                transform.translation.y += PROJECTILE_SPEED * ctx.time.delta_secs();
            }
        }

        if ctx.timer.0.just_finished() {
            ctx.player_res.0.toggle_fire();
            ctx.timer.0.reset();
        }
    }
}
