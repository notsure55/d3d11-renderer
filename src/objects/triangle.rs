use super::vertex::Vertex;
use crate::objects::Primitive;

use windows::Win32::Graphics::Direct3D::*;

const TOPOLOGY: D3D_PRIMITIVE_TOPOLOGY = D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST;

#[repr(C)]
#[derive(Debug)]
pub struct Triangle {
    pub vertices: [Vertex; 3],
}

impl Triangle {
    pub fn new(vertices: [Vertex; 3]) -> Self {
        Self { vertices }
    }
}

impl Primitive for Triangle {
    fn get_vertices(&self) -> &[Vertex] {
        &self.vertices
    }
    fn get_topology(&self) -> D3D_PRIMITIVE_TOPOLOGY {
        TOPOLOGY
    }
}
