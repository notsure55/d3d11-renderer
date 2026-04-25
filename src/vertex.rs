use super::color::Color;

#[repr(C)]
pub struct Vertex {
    pos: Vec2,
    color: Color,
}

impl Vertex {
    pub fn new(pos: Vec2, color: Color) -> Self {
        Self { pos, color }
    }
}

#[repr(C)]
pub struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2 {
    pub fn new(pos: [f32; 2]) -> Self {
        Self {
            x: pos[0],
            y: pos[1],
        }
    }
}

#[repr(C)]
pub struct Triangle {
    vertices: [Vertex; 3],
}

impl Triangle {
    pub fn new(vertices: [Vertex; 3]) -> Self {
        Self { vertices }
    }
    pub fn stride() -> u32 {
        std::mem::size_of::<Vertex>() as u32
    }
    pub fn vertice_count() -> u32 {
        3
    }
    pub fn size() -> u32 {
        std::mem::size_of::<Triangle>() as u32
    }
}
