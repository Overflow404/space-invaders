use bevy::{
    asset::AssetServer,
    color::Color,
    ecs::{
        component::Component,
        entity::Entity,
        hierarchy::ChildSpawnerCommands,
        query::With,
        resource::Resource,
        system::{Commands, Query, Res},
    },
    time::Timer,
    ui::{
        AlignItems, BackgroundColor, FlexDirection, JustifyContent, Node, UiRect, Val,
        widget::ImageNode,
    },
    utils::default,
};

use crate::{
    domain::enemy_formation::EnemyFormation, infrastructure::bevy::game_area::GameAreaView,
};

#[derive(Resource)]
pub struct EnemyFormationResource(pub EnemyFormation);

#[derive(Resource)]
pub struct EnemyFormationMovementTimer(pub Timer);

#[derive(Component)]
pub struct EnemyFormationView;

impl EnemyFormationView {
    pub fn spawn_enemy_formation(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        enemy_formation_res: Res<EnemyFormationResource>,
        game_area_query: Query<Entity, With<GameAreaView>>,
    ) {
        if let Ok(game_area) = game_area_query.single() {
            commands.entity(game_area).with_children(|parent| {
                parent
                    .spawn((
                        Self,
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(85.0),
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::FlexStart,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb_u8(12, 12, 12)),
                    ))
                    .with_children(|formation_container| {
                        Self::on_update(formation_container, &asset_server, &enemy_formation_res);
                    });
            });
        }
    }

    pub fn on_update(
        parent: &mut ChildSpawnerCommands,
        asset_server: &AssetServer,
        enemy_formation_res: &EnemyFormationResource,
    ) {
        let grid = enemy_formation_res.0.get_enemies();

        for (x, row) in grid.iter().enumerate() {
            parent
                .spawn((Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0 / 15.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceAround,
                    align_items: AlignItems::Center,
                    margin: UiRect::axes(Val::Px(7.0), Val::Px(7.0)),
                    ..default()
                },))
                .with_children(|row_container| {
                    for (y, _) in row.iter().enumerate() {
                        if grid[x][y].is_some() {
                            row_container.spawn((
                                ImageNode {
                                    image: asset_server.load("red.png"),
                                    ..default()
                                },
                                Node {
                                    width: Val::Px(30.0),
                                    height: Val::Percent(100.0),
                                    margin: UiRect::axes(Val::Px(12.0), Val::Px(0.0)),
                                    ..default()
                                },
                            ));
                        } else {
                            row_container.spawn((Node {
                                width: Val::Px(30.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },));
                        }
                    }
                });
        }
    }
}
