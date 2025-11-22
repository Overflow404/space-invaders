use bevy::{
    asset::AssetServer,
    color::Color,
    ecs::{
        component::Component,
        entity::Entity,
        query::With,
        resource::Resource,
        system::{Commands, Query, Res},
    },
    ui::{
        AlignItems, BackgroundColor, FlexDirection, JustifyContent, Node, UiRect, Val, ZIndex,
        widget::ImageNode,
    },
    utils::default,
};

use crate::{
    domain::shield_formation::ShieldFormation, infrastructure::bevy::game_area::GameAreaView,
};

const SHIELD_IMAGE: &str = "shield.png";
#[derive(Resource)]
pub struct ShieldFormationResource(pub ShieldFormation);

#[derive(Component)]
pub struct ShieldFormationView;

impl ShieldFormationView {
    pub fn spawn_shields(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        game_area_query: Query<Entity, With<GameAreaView>>,
        shield_formation_res: Res<ShieldFormationResource>,
    ) {
        if let Ok(game_area) = game_area_query.single() {
            let shields = shield_formation_res.0.get_shields();
            commands.entity(game_area).with_children(|parent| {
                parent
                    .spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(15.0),
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::SpaceAround,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb_u8(60, 60, 60)),
                        ZIndex(-1),
                    ))
                    .with_children(|shield_area| {
                        for _ in 0..shields.len() {
                            shield_area.spawn((
                                ImageNode {
                                    image: asset_server.load(SHIELD_IMAGE),
                                    ..default()
                                },
                                Node {
                                    height: Val::Percent(100.0),
                                    width: Val::Percent(10.0),
                                    margin: UiRect::right(Val::Px(25.0)),
                                    ..default()
                                },
                            ));
                        }
                    });
            });
        }
    }
}
