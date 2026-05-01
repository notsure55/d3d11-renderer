use super::color::Color;
use super::vertex::Vertex;
use crate::objects::Primitive;
use math::vec_two::Vec2;

use windows::Win32::Graphics::Direct3D::*;

#[derive(Debug, Clone)]
pub struct Rectangle {
    pub vertices: [Vertex; 5],
    pub topology: D3D_PRIMITIVE_TOPOLOGY,
    pub top_left: Vec2,
    pub width: f32,
    pub height: f32,
}

impl Rectangle {
    pub fn new(
        pos: Vec2,
        width: f32,
        height: f32,
        color: Color,
        topology: D3D_PRIMITIVE_TOPOLOGY,
    ) -> Self {
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

        Self {
            vertices,
            topology,
            top_left: pos,
            width,
            height,
        }
    }

    pub fn new_filled(pos: Vec2, width: f32, height: f32, color: Color) -> Self {
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

        Self {
            vertices,
            topology: D3D11_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP,
            top_left: pos,
            width,
            height,
        }
    }
}

impl Primitive for Rectangle {
    fn get_vertices(&self) -> &[Vertex] {
        &self.vertices
    }
    fn get_topology(&self) -> D3D_PRIMITIVE_TOPOLOGY {
        self.topology
    }
    fn in_bounds(&self, pos: Vec2) -> bool {
        if pos.x < self.top_left.x + self.width
            && pos.x > self.top_left.x
            && pos.y < self.top_left.y + self.height
            && pos.y > self.top_left.y
        {
            true
        } else {
            false
        }
    }
}
