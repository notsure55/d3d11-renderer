use super::color::Color;
use super::vertex::{Vec2, Vertex};

#[repr(C)]
#[derive(Debug)]
pub struct Rectangle {
    pub vertices: [Vertex; 5],
}

impl Rectangle {
    pub fn new(pos: Vec2, width: f32, height: f32, color: Color) -> Self {
        let vertices = [
            // top left
            Vertex::new(pos.x, pos.y, color),
            // bottom left
            Vertex::new(pos.x, pos.y + height, color),
            // bottom right
            Vertex::new(pos.x + width, pos.y + height, color),
            // top right
            Vertex::new(pos.x + width, pos.y, color),
            // top left
            Vertex::new(pos.x, pos.y, color),
        ];

        Self { vertices }
    }
    pub fn normalize(&mut self, window_width: f32, window_height: f32) {
        for vertice in &mut self.vertices {
            vertice.normalize(window_width, window_height);
        }
    }
}
