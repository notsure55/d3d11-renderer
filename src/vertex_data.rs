use crate::objects::vertex::Vertex;
use crate::objects::*;
use std::collections::BTreeMap;
use std::vec::Vec;

#[derive(Debug)]
pub struct VertexData {
    pub data: Vec<Vertex>,
    pub index: BTreeMap<u32, u32>,
    pub stride: u32,
}

pub fn stride() -> u32 {
    std::mem::size_of::<Vertex>() as u32
}

impl VertexData {
    pub fn new() -> Self {
        Self {
            data: vec![],
            index: BTreeMap::new(),
            stride: stride(),
        }
    }
    pub fn count(&self) -> u32 {
        self.data.len() as u32
    }
    pub fn push(&mut self, obj: Object) {
        let vertices = obj.get_vertices();
        let current_index = self.data.len();

        self.data.extend_from_slice(vertices);
        self.index
            .insert(current_index as u32, vertices.len() as u32);
    }
    pub fn clean(&mut self) {
        self.data = vec![];
        self.index = BTreeMap::new();
    }
}
