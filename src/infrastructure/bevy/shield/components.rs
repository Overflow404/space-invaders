use crate::infrastructure::bevy::shield::resources::{
    SHIELD_HEIGHT, SHIELD_IMAGE, SHIELD_WIDTH, SHIELD_Y,
};
use bevy::asset::AssetServer;
use bevy::math::Vec2;
use bevy::prelude::{default, Bundle, Component, Sprite, Transform};

#[derive(Component, PartialEq, Debug)]
pub struct ShieldComponent;

#[derive(Bundle)]
pub struct ShieldBundle {
    pub shield: ShieldComponent,
    pub sprite: Sprite,
    pub transform: Transform,
}

impl ShieldBundle {
    pub fn new(asset_server: &AssetServer, x: f32) -> Self {
        Self {
            shield: ShieldComponent,
            sprite: Sprite {
                image: asset_server.load(SHIELD_IMAGE),
                custom_size: Some(Vec2::new(SHIELD_WIDTH, SHIELD_HEIGHT)),
                ..default()
            },
            transform: Transform::from_xyz(x, SHIELD_Y, 0.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::app::App;
    use bevy::asset::{AssetApp, AssetPlugin, AssetServer, Handle};
    use bevy::image::Image;
    use bevy::math::Vec2;
    use bevy::MinimalPlugins;

    #[test]
    fn should_create_the_bundle() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()))
            .init_asset::<Image>();

        let asset_server = app.world().resource::<AssetServer>().clone();
        let x = 250.0;

        let bundle = ShieldBundle::new(&asset_server, x);

        assert_eq!(bundle.shield, ShieldComponent);

        assert_eq!(bundle.transform.translation.x, x);
        assert_eq!(bundle.transform.translation.y, SHIELD_Y);
        assert_eq!(bundle.transform.translation.z, 0.0);

        assert_eq!(
            bundle.sprite.custom_size,
            Some(Vec2::new(SHIELD_WIDTH, SHIELD_HEIGHT)),
        );

        assert!(matches!(bundle.sprite.image, Handle::Strong(_)));
    }
}
