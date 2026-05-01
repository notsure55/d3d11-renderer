use super::color::Color;
use super::rectangle::Rectangle;
use super::vertex::Vertex;
use super::Primitive;
use math::vec_two::Vec2;

use windows::Win32::Graphics::Direct3D::*;

const TOPOLOGY: D3D_PRIMITIVE_TOPOLOGY = D3D11_PRIMITIVE_TOPOLOGY_LINELIST;

#[derive(Debug, Clone)]
pub struct Cross {
    vertices: [Vertex; 4],
    center: Vec2,
    width: f32,
    height: f32,
}

impl Cross {
    pub fn new(center: Vec2, width: f32, height: f32, color: Color) -> Self {
        let vertices = [
            // top left
            Vertex::new(center.x - width / 2.0, center.y - height / 2.0, color),
            // bottom right
            Vertex::new(center.x + width / 2.0, center.y + height / 2.0, color),
            // top right
            Vertex::new(center.x + width / 2.0, center.y - height / 2.0, color),
            // bottom left
            Vertex::new(center.x - width / 2.0, center.y + height / 2.0, color),
        ];

        Self {
            vertices,
            center,
            width,
            height,
        }
    }
    pub fn new_from_rect(rect: &Rectangle, offset_inside: f32, color: Color) -> Self {
        let center = Vec2::new(
            rect.top_left.x + rect.width / 2.0,
            rect.top_left.y + rect.height / 2.0,
        );

        Self::new(
            center,
            rect.width - offset_inside,
            rect.height - offset_inside,
            color,
        )
    }
}

impl Primitive for Cross {
    fn get_vertices(&self) -> &[Vertex] {
        &self.vertices
    }
    fn get_topology(&self) -> D3D_PRIMITIVE_TOPOLOGY {
        TOPOLOGY
    }
    fn in_bounds(&self, _pos: Vec2) -> bool {
        false
    }
}
