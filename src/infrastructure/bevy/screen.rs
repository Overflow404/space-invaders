use bevy::{
    asset::AssetServer,
    ecs::{component::Component, system::Commands},
    math::Vec2,
    sprite::Sprite,
    ui::{FlexDirection, Node, UiRect, Val, ZIndex},
    utils::default,
};

#[derive(Component)]
pub struct ScreenView;

impl ScreenView {
    pub fn render(commands: &mut Commands, asset_server: &AssetServer) {
        commands.spawn((
            Sprite {
                image: asset_server.load("tv.png"),
                custom_size: Some(Vec2::new(1200.0, 700.0)),
                ..default()
            },
            ZIndex(-2),
        ));

        commands.spawn((
            Self,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(15.0)),
                ..default()
            },
        ));
    }
}
