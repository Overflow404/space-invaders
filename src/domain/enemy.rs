use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Enemy {
    width: f32,
    height: f32,
    x: f32,
    y: f32,
}

impl Enemy {
    pub fn new(width: f32, height: f32) -> Self {
        Enemy {
            width,
            height,
            x: 0.0,
            y: 0.0,
        }
    }
}
