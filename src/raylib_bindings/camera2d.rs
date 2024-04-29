use super::vector2::Vector2;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Camera2D {
    offset: Vector2,
    pub target: Vector2,
    rotation: f32,
    pub zoom: f32,
}

impl Camera2D {
    pub fn new(offset: Vector2, target: Vector2, rotation: f32, zoom: f32) -> Camera2D {
        Camera2D {
            offset,
            target,
            rotation,
            zoom,
        }
    }
}
