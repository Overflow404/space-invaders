use crate::infrastructure::bevy::game_area::{GAME_AREA_HEIGHT, GAME_AREA_WIDTH};
use bevy::asset::AssetServer;
use bevy::math::Vec2;
use bevy::prelude::{default, Component, Sprite, Transform};

pub const SHIELD_IMAGE: &str = "shield.png";
pub const SHIELD_WIDTH: f32 = GAME_AREA_WIDTH * 0.09;
pub const SHIELD_HEIGHT: f32 = GAME_AREA_HEIGHT * 0.11;
pub const SHIELD_Y: f32 = -(GAME_AREA_HEIGHT / 2.0) * 0.58;
pub const SHIELD_X: f32 = -(GAME_AREA_WIDTH / 2.0) * 0.68;

#[derive(Component)]
pub struct ShieldComponent;

impl ShieldComponent {
    pub fn make_shield(
        asset_server: &AssetServer,
        x_step: f32,
    ) -> (ShieldComponent, Sprite, Transform) {
        (
            ShieldComponent,
            Sprite {
                image: asset_server.load(SHIELD_IMAGE),
                custom_size: Some(Vec2::new(SHIELD_WIDTH, SHIELD_HEIGHT)),
                ..default()
            },
            Transform::from_xyz(x_step, SHIELD_Y, 0.0),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::infrastructure::bevy::shield::{
        ShieldComponent, SHIELD_HEIGHT, SHIELD_WIDTH, SHIELD_Y,
    };
    use bevy::app::App;
    use bevy::asset::{AssetApp, AssetPlugin, AssetServer, Handle};
    use bevy::image::Image;
    use bevy::math::Vec2;
    use bevy::MinimalPlugins;

    fn setup() -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()));
        app.init_asset::<Image>();

        app
    }

    #[test]
    fn should_create_the_shield_bundle() {
        let app = setup();

        let asset_server = app.world().resource::<AssetServer>();

        let x_step = 250.0;

        let (_, sprite, transform) = ShieldComponent::make_shield(asset_server, x_step);

        assert_eq!(transform.translation.x, 250.0);
        assert_eq!(transform.translation.y, SHIELD_Y);
        assert_eq!(transform.translation.z, 0.0);

        assert_eq!(
            sprite.custom_size,
            Some(Vec2::new(SHIELD_WIDTH, SHIELD_HEIGHT)),
        );

        assert!(matches!(sprite.image, Handle::Strong(_)));
    }
}
