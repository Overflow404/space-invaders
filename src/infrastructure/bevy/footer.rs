use crate::infrastructure::bevy::game_area::{GAME_AREA_HEIGHT, GAME_AREA_WIDTH};
use bevy::color::Color;
use bevy::math::Vec2;
use bevy::prelude::{Commands, Component, Transform};
use bevy::sprite::Sprite;
use bevy::utils::default;

const LINE_THICKNESS: f32 = 5.0;
const LINE_LENGTH: f32 = GAME_AREA_WIDTH;
const LINE_X: f32 = 0.0;
const LINE_Y: f32 = -(GAME_AREA_HEIGHT / 2.0) * 0.90;

#[derive(Component)]
pub struct FooterView;

impl FooterView {
    pub fn spawn_footer(mut commands: Commands) {
        commands.spawn((
            FooterView,
            Sprite {
                color: Color::srgb(0.2039, 1.0, 0.0),
                custom_size: Some(Vec2::new(LINE_LENGTH, LINE_THICKNESS)),
                ..default()
            },
            Transform::from_xyz(LINE_X, LINE_Y, 0.0),
        ));
    }
}
#[cfg(test)]
mod tests {
    use crate::infrastructure::bevy::footer::FooterView;
    use bevy::app::{App, Startup};
    use bevy::MinimalPlugins;
    use std::error::Error;

    fn setup() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_systems(Startup, FooterView::spawn_footer);
        app.update();
        app
    }

    #[test]
    fn should_display_the_footer() -> Result<(), Box<dyn Error>> {
        let mut app = setup();
        let mut query = app.world_mut().query::<&FooterView>();

        query.single(app.world())?;

        Ok(())
    }
}
