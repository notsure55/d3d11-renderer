pub mod color;
pub mod rectangle;
pub mod triangle;
pub mod vertex;

use rectangle::Rectangle;
use triangle::Triangle;
use vertex::Vertex;

use windows::Win32::Graphics::Direct3D::*;

#[derive(Debug)]
pub enum Object {
    Rectangle(Rectangle),
    Triangle(Triangle),
}

impl Primitive for Object {
    fn get_vertices(&self) -> &[Vertex] {
        match self {
            Object::Rectangle(rect) => rect.get_vertices(),
            Object::Triangle(triangle) => triangle.get_vertices(),
        }
    }
    fn get_topology(&self) -> D3D_PRIMITIVE_TOPOLOGY {
        match self {
            Object::Rectangle(rect) => rect.get_topology(),
            Object::Triangle(tri) => tri.get_topology(),
        }
    }
    fn normalize(&self, window_width: f32, window_height: f32) -> Vec<Vertex> {
        match self {
            Object::Rectangle(rect) => rect.normalize(window_width, window_height),
            Object::Triangle(tri) => tri.normalize(window_width, window_height),
        }
    }
}

pub trait Primitive {
    fn get_vertices(&self) -> &[Vertex];

    fn get_topology(&self) -> D3D_PRIMITIVE_TOPOLOGY;

    fn normalize(&self, window_width: f32, window_height: f32) -> Vec<Vertex> {
        self.get_vertices()
            .iter()
            .map(|vert| vert.normalize(window_width, window_height))
            .collect()
    }
}
