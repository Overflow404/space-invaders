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
