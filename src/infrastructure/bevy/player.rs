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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::bevy::game_area::GAME_AREA_WIDTH;
    use crate::infrastructure::bevy::projectile::{ProjectileMovementTimer, ProjectileView};

    fn setup() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(AssetPlugin::default());

        app.init_resource::<ButtonInput<KeyCode>>();
        app.init_resource::<Time>();
        app.insert_resource(PlayerResource(Player::new()));
        app.insert_resource(ProjectileMovementTimer(Timer::from_seconds(
            1.0,
            TimerMode::Once,
        )));

        app.init_asset::<Image>();

        app.add_systems(Startup, PlayerView::spawn_player);
        app.add_systems(Update, (PlayerView::on_move, PlayerView::on_fire));

        app.update();
        app
    }

    #[test]
    fn player_should_move_right_on_input() -> Result<(), Box<dyn std::error::Error>> {
        let mut app = setup();

        let start_x = app
            .world_mut()
            .query::<&Transform>()
            .single(app.world())?
            .translation
            .x;

        let mut input = app
            .world_mut()
            .get_resource_mut::<ButtonInput<KeyCode>>()
            .ok_or("ButtonInput resource missing")?;

        input.press(KeyCode::ArrowRight);

        app.update();

        let end_x = app
            .world_mut()
            .query::<&Transform>()
            .single(app.world())?
            .translation
            .x;

        assert!(end_x > start_x, "Player should move right");

        Ok(())
    }

    #[test]
    fn player_should_move_left_on_input() -> Result<(), Box<dyn std::error::Error>> {
        let mut app = setup();

        let start_x = app
            .world_mut()
            .query::<&Transform>()
            .single(app.world())?
            .translation
            .x;

        app.world_mut()
            .get_resource_mut::<ButtonInput<KeyCode>>()
            .ok_or("ButtonInput resource missing")?
            .press(KeyCode::ArrowLeft);

        app.update();

        let end_x = app
            .world_mut()
            .query::<&Transform>()
            .single(app.world())?
            .translation
            .x;

        assert!(end_x < start_x, "Player should move left");

        Ok(())
    }

    #[test]
    fn player_should_not_move_out_of_bounds() -> Result<(), Box<dyn std::error::Error>> {
        let mut app = setup();

        let boundary = (GAME_AREA_WIDTH / 2.0) - (PLAYER_WIDTH / 2.0);

        let player_entity = app
            .world_mut()
            .query_filtered::<Entity, With<PlayerView>>()
            .single(app.world())?;

        let mut transform = app
            .world_mut()
            .get_mut::<Transform>(player_entity)
            .ok_or("Player Transform missing")?;

        transform.translation.x = boundary;

        app.world_mut()
            .get_resource_mut::<ButtonInput<KeyCode>>()
            .ok_or("Input missing")?
            .press(KeyCode::ArrowRight);

        app.update();

        let end_x = app
            .world_mut()
            .query::<&Transform>()
            .single(app.world())?
            .translation
            .x;

        assert!(
            (end_x - boundary).abs() < 0.001,
            "Player should be clamped at boundary"
        );

        Ok(())
    }

    #[test]
    fn player_should_spawn_projectile_when_firing() -> Result<(), Box<dyn std::error::Error>> {
        let mut app = setup();

        let initial_count = app
            .world_mut()
            .query::<&ProjectileView>()
            .iter(app.world())
            .len();

        assert_eq!(initial_count, 0);

        app.world_mut()
            .get_resource_mut::<ButtonInput<KeyCode>>()
            .ok_or("ButtonInput resource missing")?
            .press(KeyCode::Space);

        app.update();

        let final_count = app
            .world_mut()
            .query::<&ProjectileView>()
            .iter(app.world())
            .len();

        assert_eq!(final_count, 1, "A projectile should spawn");

        let player_res = app
            .world()
            .get_resource::<PlayerResource>()
            .ok_or("PlayerResource missing")?;

        assert!(
            player_res.0.is_firing(),
            "Player resource should be marked as firing"
        );

        Ok(())
    }

    #[test]
    fn player_should_not_fire_if_cooldown_is_active() -> Result<(), Box<dyn std::error::Error>> {
        let mut app = setup();

        app.world_mut()
            .get_resource_mut::<PlayerResource>()
            .ok_or("PlayerResource missing")?
            .0
            .toggle_fire();

        app.world_mut()
            .get_resource_mut::<ButtonInput<KeyCode>>()
            .ok_or("ButtonInput resource missing")?
            .press(KeyCode::Space);

        app.update();

        let count = app
            .world_mut()
            .query::<&ProjectileView>()
            .iter(app.world())
            .len();

        assert_eq!(
            count, 0,
            "Should not spawn projectile if cooldown is active"
        );

        Ok(())
    }
}
