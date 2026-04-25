use super::vertex::Vertex;

#[repr(C)]
#[derive(Debug)]
pub struct Triangle {
    pub vertices: [Vertex; 3],
}

impl Triangle {
    pub fn new(vertices: [Vertex; 3]) -> Self {
        Self { vertices }
    }
    pub fn vertice_count() -> u32 {
        3
    }
    pub fn size() -> u32 {
        std::mem::size_of::<Triangle>() as u32
    }
}
