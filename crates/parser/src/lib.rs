use ast::{Attribute, Component, Element, Node, NodeRef, Script, Style};
use lexer::{HtmlLexer, Token};

pub struct HtmlParser {
    lexer: HtmlLexer,
    fragment: NodeRef,
    stack: Vec<NodeRef>,
    script: Vec<Script>,
    style: Vec<Style>,
    token: Token,
}

impl HtmlParser {
    pub fn new(source: &str) -> HtmlParser {
        let fragment = NodeRef::new(Node::Element(Element {
            name: "fragment".into(),
            attributes: Vec::new(),
            children: Vec::new(),
        }));

        HtmlParser {
            lexer: HtmlLexer::new(source),
            stack: vec![fragment.clone()],
            script: Vec::new(),
            style: Vec::new(),
            fragment,
            token: Token::EOF,
        }
    }

    pub fn parse(mut self) -> Component {
        self.next();
        while self.token != Token::EOF {
            self.handle_token();
        }

        Component {
            html: self.fragment,
            script: self.script,
            style: self.style,
        }
    }

    fn current(&mut self) -> NodeRef {
        self.stack.last().unwrap().clone()
    }

    fn next(&mut self) -> &Token {
        self.token = self.lexer.next();
        &self.token
    }

    fn handle_token(&mut self) {
        match self.token {
            Token::Character(ch) if ch.is_whitespace() => {
                self.next();
            }
            Token::Character(_) => self.handle_text(),
            Token::Doctype => panic!("Does not support doctype"),
            Token::Tag { .. } => self.handle_tag(),
            Token::EOF => return,
        };
    }

    fn handle_tag(&mut self) {
        let parent = self.current();
        let name = self.token.tag_name();

        if self.token.is_end_tag() {
            self.stack.pop();
            self.next();
            return;
        }

        if self.token.tag_name().as_str() == "script" {
            let mut source = String::new();
            while !self.next().is_tag() {
                source.push(self.token.character());
            }
            let mut script = Script {
                attributes: Vec::new(),
                source,
            };
            for attribute in self.token.attributes() {
                script.attributes.push(Attribute {
                    name: attribute.name.clone(),
                    value: attribute.value.clone(),
                });
            }
            self.script.push(script);
            return;
        }

        if self.token.tag_name().as_str() == "style" {
            let mut source = String::new();
            while !self.next().is_tag() {
                source.push(self.token.character());
            }
            let mut style = Style {
                attributes: Vec::new(),
                source,
            };
            for attribute in self.token.attributes() {
                style.attributes.push(Attribute {
                    name: attribute.name.clone(),
                    value: attribute.value.clone(),
                });
            }
            self.style.push(style);
            return;
        }

        let mut element = Node::new_element(name.clone());
        for attribute in self.token.attributes() {
            element.set_element_attribute(attribute.name.clone(), attribute.value.clone());
        }

        let element_ref = NodeRef::new(element);

        if self.token.self_closing() {
            // We don't push it onto the stack if it's self closing.
            parent.borrow_mut().append_child(element_ref);
        } else {
            parent.borrow_mut().append_child(element_ref.clone());
            self.stack.push(element_ref.clone());
        }
        self.next();
    }

    fn handle_text(&mut self) {
        let mut text = Node::new_text();
        text.append_text(self.token.character());

        while let Token::Character(ch) = self.next() {
            text.append_text(*ch);
        }

        self.current().borrow_mut().append_child(NodeRef::new(text));
    }
}
