use crate::infrastructure::bevy::footer::resources::{
    LINE_COLOR, LINE_LENGTH, LINE_THICKNESS, LINE_X, LINE_Y,
};
use bevy::math::Vec2;
use bevy::prelude::{Bundle, Component, Transform};
use bevy::sprite::Sprite;
use bevy::utils::default;

#[derive(Bundle)]
pub struct FooterBundle {
    pub footer: FooterComponent,
    pub sprite: Sprite,
    pub transform: Transform,
}

#[derive(Component)]
pub struct FooterComponent;

impl FooterBundle {
    pub fn new() -> Self {
        Self {
            footer: FooterComponent,
            sprite: Sprite {
                color: LINE_COLOR,
                custom_size: Some(Vec2::new(LINE_LENGTH, LINE_THICKNESS)),
                ..default()
            },
            transform: Transform::from_xyz(LINE_X, LINE_Y, 0.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_the_footer_bundle() {
        let bundle = FooterBundle::new();

        assert_eq!(bundle.transform.translation.x, LINE_X);
        assert_eq!(bundle.transform.translation.y, LINE_Y);
        assert_eq!(bundle.transform.translation.z, 0.0);

        assert_eq!(
            bundle.sprite.custom_size,
            Some(Vec2::new(LINE_LENGTH, LINE_THICKNESS))
        );

        assert_eq!(bundle.sprite.color, LINE_COLOR);
    }
}
