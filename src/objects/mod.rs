pub mod color;
pub mod cross;
pub mod rectangle;
pub mod triangle;
pub mod vertex;

use cross::Cross;
use math::vec_two::Vec2;
use rectangle::Rectangle;
use triangle::Triangle;
use vertex::Vertex;

use windows::Win32::Graphics::Direct3D::*;

#[derive(Debug, Clone, Copy)]
pub enum Object {
    Rectangle(Rectangle),
    Triangle(Triangle),
    Cross(Cross),
}

impl Primitive for Object {
    fn get_vertices(&self) -> Vec<Vertex> {
        match self {
            Object::Rectangle(rect) => rect.get_vertices(),
            Object::Triangle(triangle) => triangle.get_vertices(),
            Object::Cross(cross) => cross.get_vertices(),
        }
    }
    fn get_topology(&self) -> D3D_PRIMITIVE_TOPOLOGY {
        match self {
            Object::Rectangle(rect) => rect.get_topology(),
            Object::Triangle(tri) => tri.get_topology(),
            Object::Cross(cross) => cross.get_topology(),
        }
    }
    fn normalize(&self, window_width: f32, window_height: f32) -> Vec<Vertex> {
        match self {
            Object::Rectangle(rect) => rect.normalize(window_width, window_height),
            Object::Triangle(tri) => tri.normalize(window_width, window_height),
            Object::Cross(cross) => cross.normalize(window_width, window_height),
        }
    }
    fn in_bounds(&self, pos: Vec2) -> bool {
        match self {
            Object::Rectangle(rect) => rect.in_bounds(pos),
            Object::Triangle(tri) => tri.in_bounds(pos),
            Object::Cross(cross) => cross.in_bounds(pos),
        }
    }
    fn pos(&mut self) -> &mut Vec2 {
        match self {
            Object::Rectangle(rect) => rect.pos(),
            Object::Triangle(tri) => tri.pos(),
            Object::Cross(cross) => cross.pos(),
        }
    }
}

pub trait Primitive {
    fn get_vertices(&self) -> Vec<Vertex>;

    fn get_topology(&self) -> D3D_PRIMITIVE_TOPOLOGY;

    fn normalize(&self, window_width: f32, window_height: f32) -> Vec<Vertex> {
        let mut vertices = self.get_vertices();
        vertices
            .iter_mut()
            .for_each(|vert| *vert = vert.normalize(window_width, window_height));

        vertices
    }

    fn in_bounds(&self, pos: Vec2) -> bool;

    fn pos(&mut self) -> &mut Vec2;

    fn move_primitive(&mut self, diff: Vec2) {
        let pos = self.pos();
        *pos = *pos + diff;
    }
}
