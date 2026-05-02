use super::color::Color;
use super::rectangle::Rectangle;
use super::vertex::Vertex;
use super::Primitive;
use math::vec_two::Vec2;

use windows::Win32::Graphics::Direct3D::*;

const TOPOLOGY: D3D_PRIMITIVE_TOPOLOGY = D3D11_PRIMITIVE_TOPOLOGY_LINELIST;

#[derive(Debug, Clone, Copy)]
pub struct Cross {
    center: Vec2,
    width: f32,
    height: f32,
    color: Color,
}

impl Cross {
    pub fn new(center: Vec2, width: f32, height: f32, color: Color) -> Self {
        Self {
            center,
            width,
            height,
            color,
        }
    }
    pub fn new_from_rect(rect: &Rectangle, offset_inside: f32, color: Color) -> Self {
        let center = Vec2::new(
            rect.top_left.x + rect.width / 2.0,
            rect.top_left.y + rect.height / 2.0,
        );

        Self::new(
            center,
            rect.width - offset_inside,
            rect.height - offset_inside,
            color,
        )
    }
}

impl Primitive for Cross {
    fn get_vertices(&self) -> Vec<Vertex> {
        vec![
            // top left
            Vertex::new(
                self.center.x - self.width / 2.0,
                self.center.y - self.height / 2.0,
                self.color,
            ),
            // bottom right
            Vertex::new(
                self.center.x + self.width / 2.0,
                self.center.y + self.height / 2.0,
                self.color,
            ),
            // top right
            Vertex::new(
                self.center.x + self.width / 2.0,
                self.center.y - self.height / 2.0,
                self.color,
            ),
            // bottom left
            Vertex::new(
                self.center.x - self.width / 2.0,
                self.center.y + self.height / 2.0,
                self.color,
            ),
        ]
    }
    fn get_topology(&self) -> D3D_PRIMITIVE_TOPOLOGY {
        TOPOLOGY
    }
    fn in_bounds(&self, _pos: Vec2) -> bool {
        false
    }
    fn pos(&mut self) -> &mut Vec2 {
        &mut self.center
    }
}
