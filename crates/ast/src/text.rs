use serde::Serialize;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct Text(Rc<RefCell<TextData>>);

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct TextData {
    value: String,
}

impl Text {
    pub fn new(value: String) -> Self {
        Text(Rc::new(RefCell::new(TextData { value })))
    }

    pub fn push(&mut self, ch: char) {
        self.0.deref().borrow_mut().value.push(ch);
    }

    pub fn push_str(&mut self, string: &str) {
        self.0.deref().borrow_mut().value.push_str(string);
    }
}
