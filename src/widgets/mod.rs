use crate::objects::*;
use math::vec_two::Vec2;
use std::cell::UnsafeCell;

pub mod checkbox;

#[derive(Debug)]
pub enum Widget {
    CheckBox(checkbox::CheckBox),
}

impl MenuWidget for Widget {
    fn primitive(&self) -> Object {
        match self {
            Widget::CheckBox(check_box) => check_box.primitive(),
        }
    }
    fn clicked(&self, mouse_pos: Vec2, mouse_clicked: bool) -> bool {
        match self {
            Widget::CheckBox(check_box) => check_box.clicked(mouse_pos, mouse_clicked),
        }
    }
    fn additional_prims(&self) -> Option<&[UnsafeCell<Object>]> {
        match self {
            Widget::CheckBox(check_box) => check_box.additional_prims(),
        }
    }
    fn move_object(&self, mouse_diff: Vec2) {
        match self {
            Widget::CheckBox(check_box) => check_box.move_object(mouse_diff),
        }
    }
}

pub trait MenuWidget {
    fn additional_prims(&self) -> Option<&[UnsafeCell<Object>]>;
    fn primitive(&self) -> Object;
    fn hovering(&self, mouse_pos: Vec2) -> bool {
        let object = self.primitive();
        object.in_bounds(mouse_pos)
    }
    fn clicked(&self, mouse_pos: Vec2, mouse_clicked: bool) -> bool;
    fn drag(&self, cached_pos: Vec2, mouse_pos: Vec2, mouse_clicked: bool) {
        if self.hovering(mouse_pos) && mouse_clicked {
            let diff = mouse_pos - cached_pos;
            log::info!("Dragging to -> {diff:?}");
            self.move_object(diff);
        }
    }
    fn move_object(&self, mouse_diff: Vec2) {}
}
