use super::color::Color;
use super::vertex::Vertex;
use super::Primitive;
use math::vec_two::Vec2;

use windows::Win32::Graphics::Direct3D::*;

const TOPOLOGY: D3D_PRIMITIVE_TOPOLOGY = D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST;

#[derive(Debug, Clone)]
pub struct Triangle {
    pub vertices: [Vertex; 3],
    pub center: Vec2,
    pub width: f32,
    pub height: f32,
}

impl Triangle {
    // must go bottom left top mid bottom right
    pub fn from_vertices(vertices: [Vertex; 3]) -> Self {
        let width = vertices[2].pos.x - vertices[0].pos.x;
        let height = vertices[2].pos.y - vertices[1].pos.y;
        let center = Vec2::new(vertices[1].pos.x, vertices[1].pos.y + height / 2.0);

        Self {
            vertices,
            center,
            width,
            height,
        }
    }
    pub fn from_width_height(center: Vec2, width: f32, height: f32, color: Color) -> Self {
        let vertices = [
            Vertex::new(center.x - width / 2.0, center.y + height / 2.0, color),
            Vertex::new(center.x, center.y - height / 2.0, color),
            Vertex::new(center.x + width / 2.0, center.y + height / 2.0, color),
        ];

        Self {
            vertices,
            center,
            width,
            height,
        }
    }
}

impl Primitive for Triangle {
    fn get_vertices(&self) -> &[Vertex] {
        &self.vertices
    }
    fn get_topology(&self) -> D3D_PRIMITIVE_TOPOLOGY {
        TOPOLOGY
    }
    // TODO implement bounds for triangles
    fn in_bounds(&self, _pos: Vec2) -> bool {
        false
    }
}
