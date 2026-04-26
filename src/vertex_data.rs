use crate::objects::vertex::Vertex;
use crate::objects::*;
use std::collections::BTreeMap;
use std::vec::Vec;

use windows::Win32::Graphics::Direct3D::*;

#[derive(Debug)]
pub struct ObjectData {
    pub index: u32,
    pub vertex_count: u32,
    pub topology: D3D_PRIMITIVE_TOPOLOGY,
}

#[derive(Debug)]
pub struct VertexData {
    pub data: Vec<Vertex>,
    pub object_data: Vec<ObjectData>,
    pub stride: u32,
}

pub fn stride() -> u32 {
    std::mem::size_of::<Vertex>() as u32
}

impl VertexData {
    pub fn new() -> Self {
        Self {
            data: vec![],
            object_data: vec![],
            stride: stride(),
        }
    }
    pub fn count(&self) -> u32 {
        self.data.len() as u32
    }
    pub fn push(&mut self, obj: Object, window_width: f32, window_height: f32) {
        let vertices = obj.normalize(window_width, window_height);
        let topology = obj.get_topology();
        let current_index = self.data.len();

        self.data.extend(&vertices);

        self.object_data.push(ObjectData {
            index: current_index as u32,
            vertex_count: vertices.len() as u32,
            topology: topology,
        })
    }
    pub fn clean(&mut self) {
        self.data.clear();
        self.object_data.clear();
    }
}
