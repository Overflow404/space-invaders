use crate::domain::shield_formation::ShieldFormation;
use crate::infrastructure::bevy::game_area::{GAME_AREA_HEIGHT, GAME_AREA_WIDTH};
use bevy::math::Vec2;
use bevy::prelude::{Sprite, Transform};
use bevy::{
    asset::AssetServer,
    ecs::{
        component::Component,
        resource::Resource,
        system::{Commands, Res},
    },
    utils::default,
};

const SHIELD_IMAGE: &str = "shield.png";
const SHIELD_WIDTH: f32 = 100.0;
const SHIELD_HEIGHT: f32 = 80.0;
const SHIELD_Y: f32 = -(GAME_AREA_HEIGHT / 2.0) * 0.58;
const SHIELD_X: f32 = -(GAME_AREA_WIDTH / 2.0) * 0.68;

#[derive(Resource)]
pub struct ShieldFormationResource(pub ShieldFormation);

#[derive(Component)]
pub struct ShieldFormationView;

#[derive(Component)]
pub struct ShieldView;

impl ShieldFormationView {
    pub fn spawn_shields(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        shield_formation_res: Res<ShieldFormationResource>,
    ) {
        let shields = shield_formation_res.0.get_shields();

        if shields.is_empty() {
            return;
        }

        let total_span = (-SHIELD_X) - SHIELD_X;
        let shield_step = total_span / (shields.len() as f32 - 1.0);

        for (index, _) in shields.iter().enumerate() {
            let x = SHIELD_X + (index as f32 * shield_step);

            commands.spawn((
                ShieldView,
                Sprite {
                    image: asset_server.load(SHIELD_IMAGE),
                    custom_size: Some(Vec2::new(SHIELD_WIDTH, SHIELD_HEIGHT)),
                    ..default()
                },
                Transform::from_xyz(x, SHIELD_Y, 0.0),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::shield_formation::ShieldFormation;
    use crate::infrastructure::bevy::shield_formation::{
        ShieldFormationResource, ShieldFormationView, ShieldView,
    };
    use bevy::app::{App, Startup};
    use bevy::asset::{AssetApp, AssetPlugin};
    use bevy::image::Image;
    use bevy::prelude::IntoScheduleConfigs;
    use bevy::MinimalPlugins;
    use std::error::Error;

    fn setup() -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()));

        app.add_systems(Startup, ShieldFormationView::spawn_shields.chain());

        app.insert_resource(ShieldFormationResource(ShieldFormation::new()));

        app.init_asset::<Image>();

        app.update();

        app
    }

    #[test]
    fn should_display_the_shields() -> Result<(), Box<dyn Error>> {
        let mut app = setup();
        let mut query = app.world_mut().query::<&ShieldView>();

        let shields_count = query.iter(app.world()).count();

        assert_eq!(shields_count, 4);

        Ok(())
    }
}
