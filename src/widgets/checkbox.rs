use super::MenuWidget;
use crate::objects::Object;
use crate::objects::{color::Color, cross::Cross, rectangle::Rectangle, Primitive};
use math::vec_two::Vec2;
use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicBool, Ordering};

// TODO find out a way to replace these unsafe cells because im not sure if im using them safely.
#[derive(Debug)]
pub struct CheckBox {
    pub rect: UnsafeCell<Object>,
    pub objects: [UnsafeCell<Object>; 1],
    pub toggle: &'static AtomicBool,
}

unsafe impl Send for CheckBox {}
unsafe impl Sync for CheckBox {}

impl CheckBox {
    pub fn new(rect: Rectangle, toggle: &'static AtomicBool) -> Self {
        let cross = Cross::new_from_rect(&rect, 2.0, Color::new([0.0, 0.5, 0.5, 1.0]));

        Self {
            rect: UnsafeCell::new(Object::Rectangle(rect)),
            objects: [UnsafeCell::new(Object::Cross(cross))],
            toggle,
        }
    }
}

impl MenuWidget for CheckBox {
    fn primitive(&self) -> Object {
        unsafe { *self.rect.get() }
    }
    fn clicked(&self, mouse_pos: Vec2, mouse_clicked: bool) -> bool {
        if self.hovering(mouse_pos) && mouse_clicked {
            self.toggle.fetch_xor(true, Ordering::SeqCst);
            true
        } else {
            false
        }
    }
    fn additional_prims(&self) -> Option<&[UnsafeCell<Object>]> {
        if self.toggle.load(Ordering::Acquire) == true {
            Some(&self.objects)
        } else {
            None
        }
    }
    fn move_object(&self, mouse_diff: Vec2) {
        unsafe { &mut *self.rect.get() }.move_primitive(mouse_diff);
        for obj in self.objects.iter() {
            unsafe { &mut *obj.get() }.move_primitive(mouse_diff);
        }
    }
}
