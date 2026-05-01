use super::{Clickable, HasPrimitive, Hoverable};
use crate::objects::rectangle::Rectangle;
use crate::objects::Object;
use math::vec_two::Vec2;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug, Clone)]
pub struct CheckBox {
    pub rect: Object,
    pub toggle: &'static AtomicBool,
}

impl CheckBox {
    pub fn new(rect: Rectangle, toggle: &'static AtomicBool) -> Self {
        Self {
            rect: Object::Rectangle(rect),
            toggle,
        }
    }
}

impl HasPrimitive for CheckBox {
    fn primitive(&self) -> &Object {
        &self.rect
    }
}

impl Hoverable for CheckBox {}
impl Clickable for CheckBox {
    fn clicked(&self, mouse_pos: Vec2, mouse_clicked: bool) -> bool {
        if self.hovering(mouse_pos) && mouse_clicked {
            self.toggle.fetch_xor(true, Ordering::SeqCst);
            true
        } else {
            false
        }
    }
}
