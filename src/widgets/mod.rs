use crate::objects::*;
use math::vec_two::Vec2;

pub mod checkbox;

#[derive(Debug, Clone)]
pub enum Widget {
    CheckBox(checkbox::CheckBox),
}

impl HasPrimitive for Widget {
    fn primitive(&self) -> &Object {
        match self {
            Widget::CheckBox(check_box) => check_box.primitive(),
        }
    }
}

impl Hoverable for Widget {}
impl Clickable for Widget {}

pub trait HasPrimitive {
    fn primitive(&self) -> &Object;
}

pub trait Hoverable: HasPrimitive {
    fn hovering(&self, mouse_pos: Vec2) -> bool {
        let object = self.primitive();
        object.in_bounds(mouse_pos)
    }
}

pub trait Clickable: Hoverable {
    fn clicked(&self, mouse_pos: Vec2, mouse_clicked: bool) -> bool {
        if self.hovering(mouse_pos) && mouse_clicked {
            true
        } else {
            false
        }
    }
}
