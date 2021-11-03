use crate::node::Node;
use serde::Serialize;
use std::{cell::RefCell, ops::Deref, rc::Rc};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct Element(Rc<RefCell<ElementData>>);

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct ElementData {
    name: String,
    attributes: Vec<Attribute>,
    children: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct Attribute {
    name: String,
    value: String,
}

impl Element {
    pub fn new(name: String, attributes: Vec<Attribute>, children: Vec<Node>) -> Element {
        Element(Rc::new(RefCell::new(ElementData {
            name,
            attributes,
            children,
        })))
    }
}

impl Element {
    pub fn append_attribute(&mut self, attribute: Attribute) {
        self.0.deref().borrow_mut().attributes.push(attribute);
    }

    pub fn append_child(&mut self, child: Node) {
        self.0.deref().borrow_mut().children.push(child);
    }
}

impl Attribute {
    pub fn new(name: String, value: String) -> Attribute {
        Attribute { name, value }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}
