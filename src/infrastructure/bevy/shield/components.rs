use crate::infrastructure::bevy::shield::resources::ShieldPart::Full;
use crate::infrastructure::bevy::shield::resources::ShieldPart::InnerLeft;
use crate::infrastructure::bevy::shield::resources::ShieldPart::InnerRight;
use crate::infrastructure::bevy::shield::resources::ShieldPart::OuterLeft;
use crate::infrastructure::bevy::shield::resources::ShieldPart::OuterRight;
use crate::infrastructure::bevy::shield::resources::{
    SHIELD_HEIGHT, SHIELD_LAYOUT, SHIELD_PART_COLUMNS, SHIELD_PART_ROWS, SHIELD_PART_SIZE,
    SHIELD_WIDTH, SHIELD_Y,
};
use bevy::asset::Assets;
use bevy::color::{Color, ColorToPacked};
use bevy::image::{Image, ImageSampler, ImageSamplerDescriptor};
use bevy::math::Vec2;
use bevy::prelude::{Bundle, Component, Sprite, Transform, default};
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

#[derive(Component, PartialEq, Debug)]
pub struct ShieldComponent;

#[derive(Bundle)]
pub struct ShieldBundle {
    pub shield: ShieldComponent,
    pub sprite: Sprite,
    pub transform: Transform,
}

impl ShieldBundle {
    pub fn new(images: &mut Assets<Image>, x: f32) -> Self {
        let shield_image = Self::make_texture(Color::srgb(0.0, 1.0, 0.0));

        let texture_handle = images.add(shield_image);

        Self {
            shield: ShieldComponent,
            sprite: Sprite {
                image: texture_handle,
                custom_size: Some(Vec2::new(SHIELD_WIDTH, SHIELD_HEIGHT)),
                ..default()
            },
            transform: Transform::from_xyz(x, SHIELD_Y, 0.0),
        }
    }

    fn make_texture(color: Color) -> Image {
        let width = SHIELD_PART_COLUMNS * SHIELD_PART_SIZE;
        let height = SHIELD_PART_ROWS * SHIELD_PART_SIZE;
        let mut data = vec![0u8; width * height * 4];

        for x in 0..SHIELD_PART_ROWS {
            for y in 0..SHIELD_PART_COLUMNS {
                match SHIELD_LAYOUT[x][y] {
                    Full => Self::fill_part(
                        &mut data,
                        width,
                        y * SHIELD_PART_SIZE,
                        x * SHIELD_PART_SIZE,
                        color,
                        Vec::new(),
                    ),
                    OuterLeft => {
                        let to_skip = Vec::from([(0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (2, 0)]);
                        Self::fill_part(
                            &mut data,
                            width,
                            y * SHIELD_PART_SIZE,
                            x * SHIELD_PART_SIZE,
                            color,
                            to_skip,
                        )
                    }
                    OuterRight => {
                        let to_skip = Vec::from([(0, 3), (0, 4), (0, 5), (1, 4), (1, 5), (2, 5)]);
                        Self::fill_part(
                            &mut data,
                            width,
                            y * SHIELD_PART_SIZE,
                            x * SHIELD_PART_SIZE,
                            color,
                            to_skip,
                        )
                    }
                    InnerRight => {
                        let to_skip = Vec::from([
                            (2, 0),
                            (2, 1),
                            (3, 0),
                            (3, 1),
                            (3, 2),
                            (4, 0),
                            (4, 1),
                            (4, 2),
                            (4, 3),
                            (5, 0),
                            (5, 1),
                            (5, 2),
                            (5, 3),
                            (5, 4),
                        ]);
                        Self::fill_part(
                            &mut data,
                            width,
                            y * SHIELD_PART_SIZE,
                            x * SHIELD_PART_SIZE,
                            color,
                            to_skip,
                        )
                    }
                    InnerLeft => {
                        let to_skip = Vec::from([
                            (2, 4),
                            (2, 5),
                            (3, 3),
                            (3, 4),
                            (3, 5),
                            (4, 2),
                            (4, 3),
                            (4, 4),
                            (4, 5),
                            (5, 1),
                            (5, 2),
                            (5, 3),
                            (5, 4),
                            (5, 5),
                        ]);
                        Self::fill_part(
                            &mut data,
                            width,
                            y * SHIELD_PART_SIZE,
                            x * SHIELD_PART_SIZE,
                            color,
                            to_skip,
                        )
                    }
                    _ => {}
                }
            }
        }

        let mut image = Image::new(
            Extent3d {
                width: width as u32,
                height: height as u32,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            data,
            TextureFormat::Rgba8UnormSrgb,
            default(),
        );

        image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor::nearest());

        image
    }

    fn fill_part(
        data: &mut Vec<u8>,
        img_width: usize,
        start_x: usize,
        start_y: usize,
        color: Color,
        to_skip: Vec<(usize, usize)>,
    ) {
        let rgba = color.to_linear().to_u8_array();

        for y in 0..SHIELD_PART_SIZE {
            for x in 0..SHIELD_PART_SIZE {
                if to_skip.contains(&(y, x)) {
                    continue;
                }

                let pixel_index = ((start_y + y) * img_width + (start_x + x)) * 4;

                data[pixel_index] = rgba[0];
                data[pixel_index + 1] = rgba[1];
                data[pixel_index + 2] = rgba[2];
                data[pixel_index + 3] = rgba[3];
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::infrastructure::bevy::shield::components::{ShieldBundle, ShieldComponent};
    use crate::infrastructure::bevy::shield::resources::SHIELD_Y;
    use bevy::asset::{AssetApp, AssetPlugin, Assets};
    use bevy::image::Image;
    use bevy_test::TestAppBuilder;

    #[test]
    fn should_create_the_shield_bundle() {
        let mut app = TestAppBuilder::new().build();
        app.add_plugins(AssetPlugin::default())
            .init_asset::<Image>();

        let mut images = app.world_mut().resource_mut::<Assets<Image>>();

        let x = 250.0;

        let bundle = ShieldBundle::new(&mut images, x);

        assert_eq!(bundle.shield, ShieldComponent);

        assert_eq!(bundle.transform.translation.x, x);
        assert_eq!(bundle.transform.translation.y, SHIELD_Y);
        assert_eq!(bundle.transform.translation.z, 0.0);
    }
}
