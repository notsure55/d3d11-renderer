use super::MenuWidget;
use crate::objects::Object;
use crate::objects::{color::Color, cross::Cross, rectangle::Rectangle};
use math::vec_two::Vec2;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug, Clone)]
pub struct CheckBox {
    pub rect: Object,
    pub objects: Vec<Object>,
    pub toggle: &'static AtomicBool,
}

impl CheckBox {
    pub fn new(rect: Rectangle, toggle: &'static AtomicBool) -> Self {
        let cross = Cross::new_from_rect(&rect, 2.0, Color::new([0.0, 0.5, 0.5, 1.0]));

        Self {
            rect: Object::Rectangle(rect),
            objects: vec![Object::Cross(cross)],
            toggle,
        }
    }
}

impl MenuWidget for CheckBox {
    fn primitive(&self) -> &Object {
        &self.rect
    }
    fn clicked(&self, mouse_pos: Vec2, mouse_clicked: bool) {
        if self.hovering(mouse_pos) && mouse_clicked {
            self.toggle.fetch_xor(true, Ordering::SeqCst);
        }
    }
    fn additional_prims(&self) -> Option<&[Object]> {
        if self.toggle.load(Ordering::Acquire) == true {
            Some(&self.objects)
        } else {
            None
        }
    }
}
