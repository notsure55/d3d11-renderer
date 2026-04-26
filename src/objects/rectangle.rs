use super::color::Color;
use super::vertex::{Vec2, Vertex};
use crate::objects::Primitive;

use windows::Win32::Graphics::Direct3D::*;

#[repr(C)]
#[derive(Debug)]
pub struct Rectangle {
    pub vertices: [Vertex; 5],
    pub topology: D3D_PRIMITIVE_TOPOLOGY,
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

        Self { vertices, topology }
    }
}

impl Primitive for Rectangle {
    fn get_vertices(&self) -> &[Vertex] {
        &self.vertices
    }
    fn get_topology(&self) -> D3D_PRIMITIVE_TOPOLOGY {
        self.topology
    }
}
