use crate::infrastructure::bevy::shield::components::ShieldBundle;
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
    use crate::infrastructure::bevy::shield::components::ShieldComponent;
    use crate::infrastructure::bevy::shield_formation::resources::ShieldFormationResource;
    use bevy::app::{App, Startup};
    use bevy::asset::AssetPlugin;
    use bevy::image::Image;
    use bevy::prelude::AssetApp;
    use bevy_test::{count_components, minimal_app};

    fn setup() -> App {
        let mut app = minimal_app(false);
        app.add_plugins(AssetPlugin::default())
            .init_asset::<Image>();
        app
    }

    #[cfg(test)]
    mod spawn_shields_system {
        use super::*;

        #[test]
        fn should_spawn_the_configured_number_of_shields() {
            let mut app = setup();
            app.insert_resource(ShieldFormationResource(ShieldFormation::new()));

            app.add_systems(Startup, spawn_shields_system);
            app.update();

            assert_eq!(count_components::<ShieldComponent>(&mut app), 4);
        }
    }
}
