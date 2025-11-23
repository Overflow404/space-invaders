use bevy::color::Color;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{default, Component, Query, Res, Sprite, Time, Transform, With};

const PROJECTILE_SPEED: f32 = 500.0;
pub const ENEMY_PROJECTILE_DURATION: f32 = 1.2;
const PROJECTILE_WIDTH: f32 = 5.0;
const PROJECTILE_HEIGHT: f32 = 15.0;

#[derive(Component)]
pub struct EnemyProjectileView {
    start_position: Vec3,
}

impl EnemyProjectileView {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            start_position: Vec3::new(x, y, 0.0),
        }
    }

    pub fn make_projectile(&self) -> (EnemyProjectileView, Sprite, Transform) {
        (
            EnemyProjectileView {
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

    pub fn on_move(time: Res<Time>, mut query: Query<&mut Transform, With<EnemyProjectileView>>) {
        for mut transform in query.iter_mut() {
            transform.translation.y -= PROJECTILE_SPEED * time.delta_secs();
        }
    }
}

#[cfg(test)]
mod test {
    use crate::infrastructure::bevy::enemy::EnemyProjectileMovementTimer;
    use crate::infrastructure::bevy::enemy_projectile::{
        EnemyProjectileView, PROJECTILE_HEIGHT, PROJECTILE_WIDTH,
    };
    use bevy::app::App;
    use bevy::color::Color;
    use bevy::ecs::system::RunSystemOnce;
    use bevy::math::{Vec2, Vec3};
    use bevy::prelude::{Time, Timer, TimerMode, Transform};
    use bevy::MinimalPlugins;
    use std::time::Duration;

    fn setup() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        app.init_resource::<Time>();
        app.insert_resource(EnemyProjectileMovementTimer(Timer::from_seconds(
            1.0,
            TimerMode::Once,
        )));

        app
    }
    #[test]
    fn should_advance_projectiles_downwards() -> Result<(), Box<dyn std::error::Error>> {
        let mut app = setup();

        let projectile = app
            .world_mut()
            .spawn((
                EnemyProjectileView::new(0.0, 0.0),
                Transform::from_xyz(0.0, 0.0, 0.0),
            ))
            .id();

        app.init_resource::<Time>();

        let mut time = app.world_mut().resource_mut::<Time>();
        time.advance_by(Duration::from_secs_f32(0.1));

        app.world_mut()
            .run_system_once(EnemyProjectileView::on_move)
            .map_err(|e| format!("Cannot run system: {e}"))?;

        let transform = app
            .world()
            .get::<Transform>(projectile)
            .ok_or("Cannot get transform")?;

        println!("transform {:?}", transform);

        assert!(
            (transform.translation.y.abs() - 50.0).abs() < 0.001,
            "Projectile should have moved up by 50 units"
        );

        Ok(())
    }

    #[test]
    fn should_make_projectile() {
        let start_x = 100.0;
        let start_y = 200.0;
        let enemy_projectile_view = EnemyProjectileView::new(start_x, start_y);

        let (component, sprite, transform) = enemy_projectile_view.make_projectile();

        assert_eq!(component.start_position, Vec3::new(start_x, start_y, 0.0));

        assert_eq!(transform.translation.x, start_x);
        assert_eq!(transform.translation.y, start_y);

        assert_eq!(
            sprite.custom_size,
            Some(Vec2::new(PROJECTILE_WIDTH, PROJECTILE_HEIGHT)),
        );

        assert_eq!(sprite.color, Color::srgb(1.0, 1.0, 1.0),);
    }
}
