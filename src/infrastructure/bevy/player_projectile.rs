use crate::infrastructure::bevy::game_area::GAME_AREA_HEIGHT;
use crate::infrastructure::bevy::player::PlayerResource;
use bevy::prelude::*;

pub const PROJECTILE_SPEED: f32 = 500.0;
pub const PLAYER_PROJECTILE_DURATION: f32 = 1.2;
const PROJECTILE_WIDTH: f32 = 5.0;
const PROJECTILE_HEIGHT: f32 = 15.0;

#[derive(Resource)]
pub struct PlayerProjectileMovementTimerResource(pub Timer);

#[derive(Message)]
pub struct DespawnPlayerProjectileMessage(pub Entity);

#[derive(Component)]
pub struct PlayerProjectileComponent {
    start_position: Vec3,
}

impl PlayerResource {
    pub fn on_move(
        mut commands: Commands,
        time: Res<Time>,
        mut timer: ResMut<PlayerProjectileMovementTimerResource>,
        mut player_resource: ResMut<PlayerResource>,
        mut query: Query<(Entity, &mut Transform), With<PlayerProjectileComponent>>,
    ) {
        for (_, mut transform) in query.iter_mut() {
            transform.translation.y += PROJECTILE_SPEED * time.delta_secs();
        }

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
                if !reset_needed {
                    commands.entity(entity).despawn();
                    reset_needed = true;
                }
            }
        }

        if reset_needed {
            player_resource.0.toggle_fire();
            timer.0.reset();
        }
    }

    pub fn on_despawn_player_projectile_message(
        mut commands: Commands,
        mut despawn_player_projectile_message: MessageReader<DespawnPlayerProjectileMessage>,
    ) {
        for message in despawn_player_projectile_message.read() {
            let player_projectile_entity = message.0;
            commands.entity(player_projectile_entity).despawn();
        }
    }
}
impl PlayerProjectileComponent {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            start_position: Vec3::new(x, y, 0.0),
        }
    }

    pub fn make_player_projectile(&self) -> (PlayerProjectileComponent, Sprite, Transform) {
        (
            PlayerProjectileComponent {
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

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::player::Player;
    use crate::infrastructure::bevy::player_projectile::PlayerProjectileComponent;
    use bevy::ecs::system::RunSystemOnce;
    use std::time::Duration;

    fn setup() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        app.init_resource::<Time>();
        app.insert_resource(PlayerResource(Player::new()));
        app.insert_resource(PlayerProjectileMovementTimerResource(Timer::from_seconds(
            1.0,
            TimerMode::Once,
        )));

        app
    }

    #[test]
    fn should_move_projectile_upwards() -> Result<(), Box<dyn std::error::Error>> {
        let mut app = setup();

        let projectile = app
            .world_mut()
            .spawn((
                PlayerProjectileComponent::new(0.0, 0.0),
                Transform::from_xyz(0.0, 0.0, 0.0),
            ))
            .id();

        let mut time = app.world_mut().resource_mut::<Time>();
        time.advance_by(Duration::from_secs_f32(0.1));

        app.world_mut()
            .run_system_once(PlayerResource::on_move)
            .map_err(|e| format!("Cannot run system: {e}"))?;

        let transform = app
            .world()
            .get::<Transform>(projectile)
            .ok_or("Cannot get transform")?;

        assert!(
            (transform.translation.y - 50.0).abs() < 0.001,
            "Projectile should have moved up by 50 units"
        );

        Ok(())
    }

    #[test]
    fn should_despawn_when_out_of_bounds() {
        let mut app = setup();
        app.add_systems(Update, PlayerResource::on_move);

        app.world_mut()
            .resource_mut::<PlayerResource>()
            .0
            .toggle_fire();

        let out_of_bounds_y = (GAME_AREA_HEIGHT / 2.0) + 10.0;

        app.world_mut().spawn((
            PlayerProjectileComponent::new(0.0, 0.0),
            Transform::from_xyz(0.0, out_of_bounds_y, 0.0),
        ));

        let mut time = app.world_mut().resource_mut::<Time>();
        time.advance_by(Duration::from_secs_f32(0.01));
        app.update();

        let count = app
            .world_mut()
            .query::<&PlayerProjectileComponent>()
            .iter(app.world())
            .len();
        assert_eq!(count, 0, "Projectile should be despawned");

        let player = app.world().resource::<PlayerResource>();
        assert!(
            !player.0.is_firing(),
            "Player state should be reset to not firing"
        );
    }

    #[test]
    fn should_despawn_when_timer_finishes() -> Result<(), Box<dyn std::error::Error>> {
        let mut app = setup();

        app.world_mut()
            .resource_mut::<PlayerResource>()
            .0
            .toggle_fire();

        app.world_mut().spawn((
            PlayerProjectileComponent::new(0.0, 0.0),
            Transform::from_xyz(0.0, 0.0, 0.0),
        ));

        let mut time = app.world_mut().resource_mut::<Time>();
        time.advance_by(Duration::from_secs_f32(2.0));

        app.world_mut()
            .run_system_once(PlayerResource::on_move)
            .map_err(|e| format!("Cannot run system: {e}"))?;

        let count = app
            .world_mut()
            .query::<&PlayerProjectileComponent>()
            .iter(app.world())
            .len();

        assert_eq!(count, 0, "Projectile should be despawned due to timeout");

        let player = app.world().resource::<PlayerResource>();
        assert!(!player.0.is_firing(), "Player state should be reset");

        Ok(())
    }
    #[test]
    fn should_create_the_player_projectile_bundle() {
        let start_x = 100.0;
        let start_y = 50.0;
        let view = PlayerProjectileComponent::new(start_x, start_y);

        let (component, sprite, transform) = view.make_player_projectile();

        assert_eq!(transform.translation.x, start_x);
        assert_eq!(transform.translation.y, start_y);
        assert_eq!(
            sprite.custom_size,
            Some(Vec2::new(PROJECTILE_WIDTH, PROJECTILE_HEIGHT))
        );
        assert_eq!(sprite.color, Color::srgb(1.0, 1.0, 1.0));
        assert_eq!(component.start_position, Vec3::new(start_x, start_y, 0.0));
    }

    #[test]
    fn should_despawn_after_hitting_the_enemy() {
        let mut app = setup();
        app.add_message::<DespawnPlayerProjectileMessage>();
        app.add_systems(
            Update,
            PlayerResource::on_despawn_player_projectile_message,
        );

        let dummy_entity = app.world_mut().spawn_empty().id();
        assert!(app.world().get_entity(dummy_entity).is_ok());

        app.world_mut()
            .write_message(DespawnPlayerProjectileMessage(dummy_entity));

        app.update();

        assert!(app.world().get_entity(dummy_entity).is_err());
    }
}
