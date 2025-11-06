use bevy::{asset::Handle, ecs::resource::Resource, image::Image};

#[derive(Resource)]
pub struct BackgroundImage(pub Handle<Image>);
