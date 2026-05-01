use super::color::Color;
use math::vec_two::Vec2;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub pos: Vec2,
    pub color: Color,
}

impl Vertex {
    pub fn new(x: f32, y: f32, color: Color) -> Self {
        Self {
            pos: Vec2::new(x, y),
            color,
        }
    }
    pub fn normalize(&self, window_width: f32, window_height: f32) -> Vertex {
        let x = ((self.pos.x * 2.0) / window_width) - 1.0;
        let y = 1.0 - ((self.pos.y * 2.0) / window_height);

        Vertex {
            pos: Vec2::new(x, y),
            color: self.color,
        }
    }
}
