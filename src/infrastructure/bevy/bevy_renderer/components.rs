use crate::infrastructure::bevy::bevy_renderer::resources::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::infrastructure::bevy::header::resources::HEADER_HEIGHT;
use bevy::camera::{OrthographicProjection, ScalingMode};
use bevy::prelude::{Bundle, Camera2d, Component, Projection, Transform};

#[derive(Component, PartialEq, Debug)]
pub struct CameraComponent;

#[derive(Bundle)]
pub struct CameraBundle {
    pub camera: CameraComponent,
    pub camera_2d: Camera2d,
    pub projection: Projection,
    pub transform: Transform,
}

impl CameraBundle {
    pub fn new() -> Self {
        Self {
            camera: CameraComponent,
            camera_2d: Camera2d,
            projection: Projection::from(OrthographicProjection {
                scaling_mode: ScalingMode::AutoMin {
                    min_width: WINDOW_WIDTH,
                    min_height: WINDOW_HEIGHT,
                },
                ..OrthographicProjection::default_2d()
            }),
            transform: Transform::from_xyz(0.0, HEADER_HEIGHT, 0.0),
        }
    }
}

impl Default for CameraBundle {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::math::Vec3;

    #[test]
    fn should_create_the_camera_bundle() {
        let bundle = CameraBundle::new();

        assert_eq!(bundle.camera, CameraComponent);

        assert_eq!(bundle.transform.translation.x, 0.0);
        assert_eq!(bundle.transform.translation.y, HEADER_HEIGHT,);
        assert_eq!(bundle.transform.translation.z, 0.0);
        assert_eq!(
            bundle.transform.translation,
            Vec3::new(0.0, HEADER_HEIGHT, 0.0)
        );

        if let Projection::Orthographic(orthographic_projection) = &bundle.projection {
            match &orthographic_projection.scaling_mode {
                ScalingMode::AutoMin {
                    min_width,
                    min_height,
                } => {
                    assert_eq!(*min_width, WINDOW_WIDTH);
                    assert_eq!(*min_height, WINDOW_HEIGHT);
                }
                _ => panic!("Expected AutoMin scaling mode"),
            }
        } else {
            panic!("Expected Orthographic projection");
        }
    }
}
