#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Attribute {
    pub name: String,
    pub value: String,
}

impl Attribute {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            value: String::new(),
        }
    }

    pub fn from_name_value(name: String, value: String) -> Self {
        Self { name, value }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Token {
    Doctype,
    Tag {
        tag_name: String,
        self_closing: bool,
        attributes: Vec<Attribute>,
        is_end_tag: bool,
    },
    Character(char),
    EOF,
}

impl Token {
    pub fn new_start_tag() -> Token {
        Token::Tag {
            tag_name: String::new(),
            self_closing: false,
            attributes: Vec::new(),
            is_end_tag: false,
        }
    }

    pub fn new_end_tag() -> Token {
        Token::Tag {
            tag_name: String::new(),
            self_closing: false,
            attributes: Vec::new(),
            is_end_tag: true,
        }
    }

    pub fn is_tag(&self) -> bool {
        match self {
            Token::Tag { .. } => true,
            _ => false,
        }
    }

    pub fn is_start_tag(&self) -> bool {
        if let Token::Tag { is_end_tag, .. } = self {
            !*is_end_tag
        } else {
            false
        }
    }

    pub fn is_end_tag(&self) -> bool {
        if let Token::Tag { is_end_tag, .. } = self {
            *is_end_tag
        } else {
            false
        }
    }

    pub fn tag_name(&self) -> &String {
        if let Token::Tag { tag_name, .. } = self {
            tag_name
        } else {
            panic!("Token is not a tag");
        }
    }

    pub fn attributes(&self) -> &Vec<Attribute> {
        if let Token::Tag { attributes, .. } = self {
            &attributes
        } else {
            panic!("Token is not a tag");
        }
    }

    pub fn self_closing(&self) -> bool {
        if let Token::Tag { self_closing, .. } = self {
            *self_closing
        } else {
            panic!("Token is not a tag");
        }
    }

    pub fn set_tag_name(&mut self, new_name: &str) {
        if let Token::Tag {
            ref mut tag_name, ..
        } = self
        {
            *tag_name = new_name.to_owned();
        } else {
            panic!("Token is not a tag");
        }
    }

    pub fn character(&self) -> char {
        if let Token::Character(ch) = self {
            *ch
        } else {
            panic!("Token is not a character");
        }
    }
}
