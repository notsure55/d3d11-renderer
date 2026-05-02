use super::color::Color;
use super::vertex::Vertex;
use super::Primitive;
use math::vec_two::Vec2;

use windows::Win32::Graphics::Direct3D::*;

const TOPOLOGY: D3D_PRIMITIVE_TOPOLOGY = D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST;

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub center: Vec2,
    pub width: f32,
    pub height: f32,
    pub color: Color,
}

impl Triangle {
    // must go bottom left top mid bottom right
    pub fn from_vertices(vertices: [Vertex; 3]) -> Self {
        let width = vertices[2].pos.x - vertices[0].pos.x;
        let height = vertices[2].pos.y - vertices[1].pos.y;
        let center = Vec2::new(vertices[1].pos.x, vertices[1].pos.y + height / 2.0);

        Self {
            center,
            width,
            height,
            color: vertices[0].color,
        }
    }
    pub fn from_width_height(center: Vec2, width: f32, height: f32, color: Color) -> Self {
        Self {
            center,
            width,
            height,
            color,
        }
    }
}

impl Primitive for Triangle {
    fn get_vertices(&self) -> Vec<Vertex> {
        let vertices = vec![
            Vertex::new(
                self.center.x - self.width / 2.0,
                self.center.y + self.height / 2.0,
                self.color,
            ),
            Vertex::new(self.center.x, self.center.y - self.height / 2.0, self.color),
            Vertex::new(
                self.center.x + self.width / 2.0,
                self.center.y + self.height / 2.0,
                self.color,
            ),
        ];

        vertices
    }
    fn get_topology(&self) -> D3D_PRIMITIVE_TOPOLOGY {
        TOPOLOGY
    }
    // TODO implement bounds for triangles
    fn in_bounds(&self, _pos: Vec2) -> bool {
        false
    }
    fn pos(&mut self) -> &mut Vec2 {
        &mut self.center
    }
}
