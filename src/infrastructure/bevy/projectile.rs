use crate::infrastructure::bevy::player::{PlayerContainerView, PlayerResource, PlayerView};
use bevy::ecs::system::SystemParam;
use bevy::input::ButtonInput;
use bevy::prelude::{
    Commands, ComputedNode, KeyCode, Query, Res, ResMut, Time, Window, With, Without,
};
use bevy::{
    color::Color,
    ecs::{component::Component, resource::Resource},
    time::Timer,
    ui::{AlignItems, BackgroundColor, FlexDirection, JustifyContent, Node, Val},
    utils::default,
};

pub const PROJECTILE_TIME_IN_SECONDS: f32 = 1.0;

#[derive(Resource)]
pub struct ProjectileMovementTimer(pub Timer);

#[derive(Component)]
pub struct ProjectileView {
    x: f32,
    y: f32,
}

#[derive(SystemParam)]
pub struct FireContext<'w, 's> {
    pub commands: Commands<'w, 's>,
    pub time: Res<'w, Time>,
    pub keyboard: Res<'w, ButtonInput<KeyCode>>,
    pub player_res: ResMut<'w, PlayerResource>,
    pub timer: ResMut<'w, ProjectileMovementTimer>,
    pub player_query: Query<'w, 's, &'static Node, (With<PlayerView>, Without<ProjectileView>)>,
    pub parent_query:
        Query<'w, 's, &'static ComputedNode, (With<PlayerContainerView>, Without<PlayerView>)>,
    pub window_query: Query<'w, 's, &'static Window>,
    pub projectile_query:
        Query<'w, 's, &'static mut Node, (With<ProjectileView>, Without<PlayerView>)>,
}

impl ProjectileView {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn increase_x(&mut self) {
        self.x += 1.0;
    }

    pub fn increase_y(&mut self) {
        self.y += 1.0;
    }

    pub(crate) fn spawn_projectile(&self) -> (Self, Node, BackgroundColor) {
        (
            ProjectileView::new(0.0, 0.0),
            Node {
                left: Val::Px(self.x),
                top: Val::Px(self.y),
                width: Val::Px(10.0),
                height: Val::Px(10.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb_u8(190, 12, 12)),
        )
    }
}
