use ast::{Attribute, Document, Element, Node, Text};
use lexer::{HtmlLexer, Token};

pub fn parse_html(source: &str) -> Document {
    HtmlParser::new(source).parse()
}

struct HtmlParser {
    lexer: HtmlLexer,
    document: Document,
    stack: Vec<Node>,
    token: Token,
}

impl HtmlParser {
    fn new(source: &str) -> HtmlParser {
        let document = Document::new(Vec::new());
        let fragment = Node::Document(document.clone());

        HtmlParser {
            lexer: HtmlLexer::new(source),
            stack: vec![fragment],
            document,
            token: Token::EOF,
        }
    }

    fn parse(mut self) -> Document {
        self.next();
        while self.token != Token::EOF {
            self.handle_token();
        }

        self.document.clone()
    }

    fn current(&mut self) -> Node {
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
        let name = self.token.tag_name();

        if self.token.is_end_tag() {
            self.stack.pop();
            self.next();
            return;
        }

        let mut attributes = Vec::new();
        for lexer::Attribute { name, value } in self.token.attributes() {
            attributes.push(Attribute::new(name.clone(), value.clone()));
        }

        let element = Element::new(name.clone(), attributes, Vec::new());

        if self.token.self_closing() {
            // We don't push it onto the stack if it's self closing.
            self.current().append_child(Node::Element(element))
        } else {
            self.current().append_child(Node::Element(element.clone()));
            self.stack.push(Node::Element(element.clone()));
        }
        self.next();
    }

    fn handle_text(&mut self) {
        let mut text = Text::new(String::new());
        text.push(self.token.character());

        while let Token::Character(ch) = self.next() {
            text.push(*ch);
        }

        self.current().append_child(Node::Text(text.clone()));
    }
}
