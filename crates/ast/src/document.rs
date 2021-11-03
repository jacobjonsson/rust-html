use crate::node::Node;
use serde::Serialize;
use std::{cell::RefCell, ops::Deref, rc::Rc};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct Document(Rc<RefCell<DocumentData>>);

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct DocumentData {
    children: Vec<Node>,
}

impl Document {
    pub fn new(children: Vec<Node>) -> Document {
        Document(Rc::new(RefCell::new(DocumentData { children })))
    }
}

impl Deref for Document {
    type Target = RefCell<DocumentData>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Document {
    pub fn append_child(&mut self, child: Node) {
        self.0.deref().borrow_mut().children.push(child);
    }
}
