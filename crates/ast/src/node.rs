use std::borrow::BorrowMut;
use std::ops::{Deref, DerefMut};

use crate::document::Document;
use crate::element::Element;
use crate::text::Text;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub enum Node {
    Document(Document),
    Element(Element),
    Text(Text),
}

impl Node {
    pub fn append_child(&mut self, child: Node) {
        match self {
            Node::Document(doc) => doc.append_child(child),
            Node::Element(el) => el.borrow_mut().append_child(child),
            _ => unreachable!(),
        }
    }
}
