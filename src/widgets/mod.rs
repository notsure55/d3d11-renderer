use crate::objects::*;
use math::vec_two::Vec2;

pub mod checkbox;

#[derive(Debug, Clone)]
pub enum Widget {
    CheckBox(checkbox::CheckBox),
}

impl MenuWidget for Widget {
    fn primitive(&self) -> &Object {
        match self {
            Widget::CheckBox(check_box) => check_box.primitive(),
        }
    }
    fn clicked(&self, mouse_pos: Vec2, mouse_clicked: bool) {
        match self {
            Widget::CheckBox(check_box) => check_box.clicked(mouse_pos, mouse_clicked),
        }
    }
    fn additional_prims(&self) -> Option<&[Object]> {
        match self {
            Widget::CheckBox(check_box) => check_box.additional_prims(),
        }
    }
}

pub trait MenuWidget {
    fn additional_prims(&self) -> Option<&[Object]>;
    fn primitive(&self) -> &Object;
    fn hovering(&self, mouse_pos: Vec2) -> bool {
        let object = self.primitive();
        object.in_bounds(mouse_pos)
    }
    fn clicked(&self, mouse_pos: Vec2, mouse_clicked: bool);
}
