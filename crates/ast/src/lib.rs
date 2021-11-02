use serde::Serialize;
use std::cell::{Ref, RefCell, RefMut};
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Serialize)]
pub struct Component {
    pub html: NodeRef,
    pub script: Vec<Script>,
    pub style: Vec<Style>,
}

/// A reference to a node.
#[derive(Serialize)]
pub struct NodeRef(Rc<RefCell<Node>>);

impl Clone for NodeRef {
    fn clone(&self) -> Self {
        NodeRef(Rc::clone(&self.0))
    }
}

impl PartialEq for NodeRef {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl fmt::Debug for NodeRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&*self.borrow(), f)
    }
}

impl NodeRef {
    pub fn new(data: Node) -> NodeRef {
        NodeRef(Rc::new(RefCell::new(data)))
    }

    pub fn borrow(&self) -> Ref<Node> {
        self.0.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<Node> {
        self.0.borrow_mut()
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub enum Node {
    Element(Element),
    Script(Script),
    Style(Style),
    Text(String),
}

impl Node {
    pub fn new_text() -> Node {
        Node::Text(String::new())
    }

    pub fn new_element(name: String) -> Node {
        Node::Element(Element {
            name,
            attributes: Vec::new(),
            children: Vec::new(),
        })
    }

    pub fn append_child(&mut self, child: NodeRef) {
        match self {
            Node::Element(Element {
                ref mut children, ..
            }) => children.push(child),
            Node::Script(_) => unreachable!("Node does not support children"),
            Node::Style(_) => unreachable!("Node does not support children"),
            Node::Text(_) => unreachable!("Node does not support children"),
        }
    }

    pub fn set_element_name(&mut self, new_name: String) {
        match self {
            Node::Element(Element { ref mut name, .. }) => *name = new_name,
            Node::Script(_) => unreachable!("Node is not an element node"),
            Node::Style(_) => unreachable!("Node is not an element node"),
            Node::Text(_) => unreachable!("Node is not an element node"),
        }
    }

    pub fn set_element_attribute(&mut self, name: String, value: String) {
        match self {
            Node::Element(Element {
                ref mut attributes, ..
            }) => attributes.push(Attribute { name, value }),
            Node::Script(Script {
                ref mut attributes, ..
            }) => attributes.push(Attribute { name, value }),
            Node::Style(Style {
                ref mut attributes, ..
            }) => attributes.push(Attribute { name, value }),
            Node::Text(_) => unreachable!("Node is not an element node"),
        }
    }

    pub fn append_text(&mut self, ch: char) {
        match self {
            Node::Element(_) => unreachable!("Node is not a text node"),
            Node::Script(_) => unreachable!("Node is not a text node"),
            Node::Style(_) => unreachable!("Node is not a text node"),
            Node::Text(ref mut text) => text.push(ch),
        }
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Element {
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub children: Vec<NodeRef>,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Attribute {
    pub name: String,
    pub value: String,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Script {
    pub attributes: Vec<Attribute>,
    pub source: String,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Style {
    pub attributes: Vec<Attribute>,
    pub source: String,
}
