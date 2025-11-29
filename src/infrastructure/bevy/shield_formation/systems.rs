use crate::infrastructure::bevy::shield::components::{ShieldBundle, ShieldComponent};
use crate::infrastructure::bevy::shield::resources::SHIELD_X;
use crate::infrastructure::bevy::shield_formation::resources::ShieldFormationResource;
use bevy::asset::AssetServer;
use bevy::ecs::system::{Commands, Res};

pub fn spawn_shields_system(
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

        commands.spawn(ShieldBundle::new(&asset_server, x));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::shield_formation::ShieldFormation;
    use crate::infrastructure::bevy::shield_formation::resources::ShieldFormationResource;
    use bevy::app::App;
    use bevy::asset::{AssetApp, AssetPlugin};
    use bevy::image::Image;
    use bevy::MinimalPlugins;
    use bevy_test::{count_components, run_system};

    fn setup() -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()))
            .init_asset::<Image>();
        app
    }

    #[test]
    fn should_spawn_the_configured_number_of_shields() -> Result<(), Box<dyn std::error::Error>> {
        let mut app = setup();
        app.insert_resource(ShieldFormationResource(ShieldFormation::new()));

        run_system(&mut app, spawn_shields_system)?;

        let shields_count = count_components::<ShieldComponent>(&mut app);

        assert_eq!(shields_count, 4);

        Ok(())
    }

    #[test]
    fn should_not_spawn_the_shields_if_no_shields_available() -> Result<(), Box<dyn std::error::Error>> {
        //TODO when shields will be destroyable this would make sense
        Ok(())
    }
}
