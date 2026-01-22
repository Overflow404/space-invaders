use crate::infrastructure::bevy::bevy_renderer::resources::{
    WINDOW_HEIGHT, WINDOW_NAME, WINDOW_WIDTH,
};
use crate::infrastructure::bevy::bevy_renderer::systems::{camera_system, window_scale_system};
use crate::infrastructure::bevy::enemy_formation::plugin::EnemyFormationPlugin;
use crate::infrastructure::bevy::enemy_projectile::plugin::EnemyProjectilePlugin;
use crate::infrastructure::bevy::footer::plugin::FooterPlugin;
use crate::infrastructure::bevy::game_area::plugin::GameAreaPlugin;
use crate::infrastructure::bevy::header::plugin::HeaderPlugin;
use crate::infrastructure::bevy::lives::plugin::LivesPlugin;
use crate::infrastructure::bevy::player::plugin::PlayerPlugin;
use crate::infrastructure::bevy::player_projectile::plugin::PlayerProjectilePlugin;
use crate::infrastructure::bevy::score::plugin::ScorePlugin;
use crate::infrastructure::bevy::shield_formation::plugin::ShieldFormationPlugin;
use crate::infrastructure::renderer::Renderer;
use bevy::DefaultPlugins;
use bevy::app::{App, Plugin, PluginGroup, PostUpdate, Startup};
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::utils::default;
use bevy::window::{PresentMode, Window, WindowPlugin, WindowResolution};

pub struct BevyRenderer;

pub struct SpaceInvadersPlugin;

impl Plugin for SpaceInvadersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera_system)
            .add_systems(PostUpdate, window_scale_system)
            .add_plugins((
                PlayerPlugin,
                HeaderPlugin,
                ScorePlugin,
                LivesPlugin,
                GameAreaPlugin,
                EnemyFormationPlugin,
                ShieldFormationPlugin,
                FooterPlugin,
                EnemyProjectilePlugin,
                PlayerProjectilePlugin,
            ));
    }
}

impl Renderer for BevyRenderer {
    fn render(&self) {
        App::new()
            .add_plugins(Self::window_plugin_config())
            .add_plugins(SpaceInvadersPlugin)
            .run();
    }
}

impl BevyRenderer {
    pub fn new() -> Self {
        Self
    }

    fn window_plugin_config() -> impl PluginGroup {
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32),
                    title: WINDOW_NAME.to_string(),
                    present_mode: PresentMode::Fifo,
                    ..default()
                }),
                ..default()
            })
            .disable::<LogPlugin>()
    }
}

impl Default for BevyRenderer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::bevy::bevy_renderer::components::CameraComponent;
    use crate::infrastructure::bevy::enemy::components::EnemyComponent;
    use crate::infrastructure::bevy::footer::components::FooterComponent;
    use crate::infrastructure::bevy::game_area::components::GameAreaComponent;
    use crate::infrastructure::bevy::header::components::HeaderComponent;
    use crate::infrastructure::bevy::lives::components::LivesViewComponent;
    use crate::infrastructure::bevy::player::components::PlayerComponent;
    use crate::infrastructure::bevy::score::components::{
        ScoreLabelComponent, ScoreValueComponent,
    };
    use crate::infrastructure::bevy::shield::components::ShieldComponent;
    use bevy_test::{TestAppBuilder, contains_single_component, count_components};

    fn setup() -> App {
        TestAppBuilder::new()
            .with_assets()
            .with_input()
            .with_plugin(WindowPlugin::default())
            .without_auto_update()
            .with_setup(|app| {
                app.init_resource::<UiScale>();
                app.add_plugins(SpaceInvadersPlugin);
                app.update();
            })
            .build()
    }

    #[test]
    fn should_spawn_all_the_components() {
        let mut app = setup();

        assert!(contains_single_component::<CameraComponent>(&mut app));
        assert!(contains_single_component::<HeaderComponent>(&mut app));
        assert!(contains_single_component::<ScoreLabelComponent>(&mut app));
        assert!(contains_single_component::<ScoreValueComponent>(&mut app));
        assert!(contains_single_component::<LivesViewComponent>(&mut app));
        assert!(contains_single_component::<GameAreaComponent>(&mut app));
        assert!(contains_single_component::<PlayerComponent>(&mut app));
        assert!(contains_single_component::<FooterComponent>(&mut app));
        assert_eq!(count_components::<EnemyComponent>(&mut app), 55);
        assert_eq!(count_components::<ShieldComponent>(&mut app), 4);
    }
}
