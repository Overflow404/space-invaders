use crate::domain::shield_formation::ShieldFormation;
use crate::infrastructure::bevy::shield::{ShieldComponent, SHIELD_X};
use bevy::{
    asset::AssetServer,
    ecs::{
        component::Component,
        resource::Resource,
        system::{Commands, Res},
    }
    ,
};

#[derive(Resource)]
pub struct ShieldFormationResource(pub ShieldFormation);

#[derive(Component)]
pub struct ShieldFormationComponent;

impl ShieldFormationComponent {
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
            let x_step = SHIELD_X + (index as f32 * shield_step);

            commands.spawn(ShieldComponent::make_shield(&asset_server, x_step));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::shield_formation::ShieldFormation;
    use crate::infrastructure::bevy::shield_formation::{
        ShieldComponent, ShieldFormationComponent, ShieldFormationResource,
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

        app.add_systems(Startup, ShieldFormationComponent::spawn_shields.chain());

        app.insert_resource(ShieldFormationResource(ShieldFormation::new()));

        app.init_asset::<Image>();

        app.update();

        app
    }

    #[test]
    fn should_display_the_shields() -> Result<(), Box<dyn Error>> {
        let mut app = setup();
        let mut query = app.world_mut().query::<&ShieldComponent>();

        let shields_count = query.iter(app.world()).count();

        assert_eq!(shields_count, 4);

        Ok(())
    }
}
