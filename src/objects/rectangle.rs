use super::color::Color;
use super::vertex::Vertex;
use crate::objects::Primitive;
use math::vec_two::Vec2;

use std::cell::Cell;
use windows::Win32::Graphics::Direct3D::*;

#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    //pub vertices: [Vertex; 5],
    pub topology: D3D_PRIMITIVE_TOPOLOGY,
    pub top_left: Vec2,
    pub width: f32,
    pub height: f32,
    pub color: Color,
}

impl Rectangle {
    pub fn new(
        pos: Vec2,
        width: f32,
        height: f32,
        color: Color,
        topology: D3D_PRIMITIVE_TOPOLOGY,
    ) -> Self {
        Self {
            topology,
            top_left: pos,
            width,
            height,
            color,
        }
    }

    pub fn new_filled(pos: Vec2, width: f32, height: f32, color: Color) -> Self {
        Self {
            topology: D3D11_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP,
            top_left: pos,
            width,
            height,
            color,
        }
    }
}

impl Primitive for Rectangle {
    fn get_vertices(&self) -> Vec<Vertex> {
        vec![
            // top left
            Vertex::new(self.top_left.x, self.top_left.y, self.color),
            // bottom left
            Vertex::new(self.top_left.x, self.top_left.y + self.height, self.color),
            // bottom right
            Vertex::new(
                self.top_left.x + self.width,
                self.top_left.y + self.height,
                self.color,
            ),
            // top right
            Vertex::new(self.top_left.x + self.width, self.top_left.y, self.color),
            // top left
            Vertex::new(self.top_left.x, self.top_left.y, self.color),
        ]
    }
    fn get_topology(&self) -> D3D_PRIMITIVE_TOPOLOGY {
        self.topology
    }
    fn in_bounds(&self, pos: Vec2) -> bool {
        if pos.x < self.top_left.x + self.width
            && pos.x > self.top_left.x
            && pos.y < self.top_left.y + self.height
            && pos.y > self.top_left.y
        {
            true
        } else {
            false
        }
    }
    fn pos(&mut self) -> &mut Vec2 {
        &mut self.top_left
    }
}
