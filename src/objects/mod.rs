pub mod color;
pub mod rectangle;
pub mod triangle;
pub mod vertex;

use rectangle::Rectangle;
use triangle::Triangle;
use vertex::Vertex;

#[derive(Debug)]
pub enum Object {
    Rectangle(Rectangle),
    Triangle(Triangle),
}

impl Object {
    pub fn get_vertices(&self) -> &[Vertex] {
        match self {
            Object::Rectangle(rect) => &rect.vertices,
            Object::Triangle(triangle) => &triangle.vertices,
        }
    }
}
