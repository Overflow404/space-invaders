use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Player {
    width: f32,
    height: f32,
    x: f32,
    y: f32,
}

impl Player {
    pub fn new(width: f32, height: f32) -> Self {
        Player {
            width,
            height,
            x: 0.0,
            y: 0.0,
        }
    }
}
