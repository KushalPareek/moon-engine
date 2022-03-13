use crate::transform::Transform;
use nalgebra::Matrix4;
use nalgebra::Orthographic3;
use nalgebra::Vector3;

/// The 'X' component at the left and right edges of the screen
const FIXED_WIDTH: f32 = 20.0;
/// Calculate the height from the `FIXED_WIDTH` to maintain 16:9 Aspect ratio
const HEIGHT: f32 = FIXED_WIDTH / 1.77;

/// A `Camera` represents a Virtual Camera, that has a view and Orthographic projection matrices
#[allow(dead_code)]
#[derive(Debug)]
pub struct Camera {
    pub transform: Transform,
    pub orthographic: Orthographic3<f32>,
    pub width: f32,
    pub height: f32,
    znear: f32,
    zfar: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            transform: Transform::new(),
            width: 1920.0,
            height: 1080.0,
            znear: 0.0f32,
            zfar: 1000.0f32,
            orthographic: Orthographic3::new(
                -FIXED_WIDTH / 2.0,
                FIXED_WIDTH / 2.0,
                HEIGHT / 2.0,
                -HEIGHT / 2.0,
                0f32,
                1000.0f32,
            ),
        }
    }
}

#[allow(dead_code)]
impl Camera {
    /// Create a new `Camera` with default values.
    pub fn new() -> Self {
        Default::default()
    }
    /// Create a new `Camera` with an initial position.
    pub fn with_position(position: Vector3<f32>) -> Self {
        Self {
            transform: Transform::new_with_position(position),
            ..Default::default()
        }
    }
    /// Create a new `Camera` with an initial transform.
    pub fn with_transform(transform: Transform) -> Self {
        Self {
            transform,
            ..Default::default()
        }
    }
    /// Create a new `Camera` with an initial width and height.
    pub fn with_width_and_height(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            orthographic: Orthographic3::new(
                -width / 2.0,
                width / 2.0,
                height / 2.0,
                -height / 2.0,
                0f32,
                1000.0f32,
            ),
            ..Default::default()
        }
    }

    /// Set the width and height of the camera plane, and update the Projection Matrix to match.
    pub fn set_width_and_height(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }

    /// Return the Projection Matrix of the `Camera` as a slice of `f32` so it can be used by WebGL.
    pub fn projection(&self) -> &[f32] {
        self.orthographic.as_matrix().as_slice()
    }

    pub fn view_projection_matrix(&self) -> Matrix4<f32> {
        self.transform.matrix * self.orthographic.as_matrix()
    }

    pub fn screen_to_world_coordinates(&self, screen_x: f32, screen_y: f32) -> (f32, f32) {
        let clipped_x = (screen_x / self.width - 0.5) * 2.0;
        let clipped_y = (screen_y / self.height - 0.5) * 2.0;

        (clipped_x * FIXED_WIDTH, clipped_y * HEIGHT)
    }
}
