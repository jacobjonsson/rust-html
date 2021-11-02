mod token;

pub use token::*;

#[derive(Debug)]
pub enum State {
    Data,
    PlainText,
    TagOpen,
    TagName,
    EndTagOpen,
    BeforeAttributeName,
    AttributeName,
    AfterAttributeName,
    BeforeAttributeValue,
    AttributeValueDoubleQuoted,
    AttributeValueSingleQuoted,
    AttributeValueUnquoted,
    AfterAttributeValueQuoted,
    SelfClosingStartTag,
}

pub const EOF_CHAR: char = '\0';

pub struct HtmlLexer {
    // The list of characters
    chars: Vec<(usize, char)>,

    // The current position
    index: usize,

    // The current character
    current_char: char,

    // The current tokenizing state
    state: State,

    // The current token
    current_token: Option<Token>,

    reconsume_char: bool,
}

impl HtmlLexer {
    pub fn new(source: &str) -> HtmlLexer {
        HtmlLexer {
            chars: source.char_indices().collect(),
            index: 0,
            current_char: '\0',
            current_token: None,
            state: State::Data,
            reconsume_char: false,
        }
    }

    pub fn next(&mut self) -> Token {
        loop {
            match self.state {
                State::Data => {
                    let ch = self.consume_next();
                    match ch {
                        '<' => self.switch_to(State::TagOpen),
                        EOF_CHAR => return self.emit_eof(),
                        _ => return self.emit_char(),
                    }
                }
                State::PlainText => {
                    let ch = self.consume_next();
                    match ch {
                        EOF_CHAR => return self.emit_eof(),
                        _ => return self.emit_char(),
                    }
                }
                State::TagOpen => {
                    let ch = self.consume_next();
                    match ch {
                        '!' => todo!(),
                        '?' => todo!(),
                        '/' => self.switch_to(State::EndTagOpen),
                        c if c.is_alphabetic() => {
                            self.current_token = Some(Token::new_start_tag());
                            self.reconsume_in(State::TagName);
                        }
                        EOF_CHAR => panic!("Invalid tag name"),
                        _ => panic!("Invalid tag name"),
                    }
                }
                State::EndTagOpen => {
                    let ch = self.consume_next();
                    match ch {
                        c if c.is_alphabetic() => {
                            self.current_token = Some(Token::new_end_tag());
                            self.reconsume_in(State::TagName);
                        }
                        '>' => panic!("Missing end tag name"),
                        EOF_CHAR => panic!("Invalid EOF before tag name"),
                        _ => panic!("Invalid first character of tag name"),
                    }
                }
                State::TagName => {
                    let ch = self.consume_next();
                    match ch {
                        c if c.is_whitespace() => {
                            self.switch_to(State::BeforeAttributeName);
                        }
                        '/' => self.switch_to(State::SelfClosingStartTag),
                        '>' => {
                            self.switch_to(State::Data);
                            return self.emit_current_token();
                        }
                        c if c.is_ascii_uppercase() => {
                            self.append_character_to_tag_name(c.to_ascii_lowercase());
                        }
                        EOF_CHAR => panic!("EOF in tag"),
                        c => self.append_character_to_tag_name(c),
                    }
                }
                State::BeforeAttributeName => {
                    let ch = self.consume_next();
                    match ch {
                        c if c.is_whitespace() => {}
                        '/' => self.reconsume_in(State::AfterAttributeName),
                        '=' => panic!("unexpected-equals-sign-before-attribute-name"),
                        EOF_CHAR => self.reconsume_in(State::AfterAttributeName),
                        _ => {
                            let attribute = Attribute::new();
                            self.new_attribute(attribute);
                            self.reconsume_in(State::AttributeName);
                        }
                    }
                }
                State::AttributeName => {
                    let ch = self.consume_next();
                    match ch {
                        c if c.is_whitespace() => self.reconsume_in(State::AfterAttributeName),
                        '/' | '>' => self.reconsume_in(State::AfterAttributeName),
                        '=' => self.switch_to(State::BeforeAttributeValue),
                        '"' | '\'' | '<' => panic!("unexpected-character-in-attribute-name"),
                        c => self.append_character_to_attribute_name(c),
                    }
                }
                State::BeforeAttributeValue => {
                    let ch = self.consume_next();
                    match ch {
                        c if c.is_whitespace() => {}
                        '"' => self.switch_to(State::AttributeValueDoubleQuoted),
                        '\'' => self.switch_to(State::AttributeValueSingleQuoted),
                        '>' => panic!("missing-attribute-value"),
                        _ => self.reconsume_in(State::AttributeValueUnquoted),
                    }
                }

                State::AfterAttributeName => {
                    let ch = self.consume_next();
                    match ch {
                        c if c.is_whitespace() => {}
                        '/' => self.switch_to(State::SelfClosingStartTag),
                        '=' => self.switch_to(State::BeforeAttributeValue),
                        '>' => {
                            self.switch_to(State::Data);
                            return self.emit_current_token();
                        }
                        EOF_CHAR => panic!("eof-in-tag"),
                        _ => {
                            let attribute = Attribute::new();
                            self.new_attribute(attribute);
                            self.reconsume_in(State::AttributeName);
                        }
                    }
                }

                State::SelfClosingStartTag => {
                    let ch = self.consume_next();
                    match ch {
                        '>' => {
                            let tag = self.current_token.as_mut().unwrap();
                            if let Token::Tag {
                                ref mut self_closing,
                                ..
                            } = tag
                            {
                                *self_closing = true;
                            }
                            self.switch_to(State::Data);
                            return self.emit_current_token();
                        }
                        EOF_CHAR => panic!("eof-in-tag"),
                        _ => panic!("unexpected-solidus-in-tag"),
                    }
                }
                State::AttributeValueDoubleQuoted => {
                    let ch = self.consume_next();
                    match ch {
                        '"' => self.switch_to(State::AfterAttributeValueQuoted),
                        '&' => todo!(),
                        EOF_CHAR => panic!("eof-in-tag"),
                        c => self.append_character_to_attribute_value(c),
                    }
                }
                State::AttributeValueSingleQuoted => {
                    let ch = self.consume_next();
                    match ch {
                        '\'' => self.switch_to(State::AfterAttributeValueQuoted),
                        '&' => todo!(),
                        EOF_CHAR => panic!("eof-in-tag"),
                        c => self.append_character_to_attribute_value(c),
                    }
                }
                State::AttributeValueUnquoted => {
                    let ch = self.consume_next();
                    match ch {
                        c if c.is_whitespace() => self.switch_to(State::BeforeAttributeName),
                        '>' => {
                            self.switch_to(State::Data);
                            return self.emit_current_token();
                        }
                        '"' | '\'' | '<' | '=' | '`' => {
                            panic!("unexpected-character-in-unquoted-attribute-value");
                        }
                        EOF_CHAR => panic!("eof-in-tag"),
                        c => self.append_character_to_attribute_value(c),
                    }
                }
                State::AfterAttributeValueQuoted => {
                    let ch = self.consume_next();
                    match ch {
                        c if c.is_whitespace() => self.switch_to(State::BeforeAttributeName),
                        '/' => self.switch_to(State::SelfClosingStartTag),
                        '>' => {
                            self.switch_to(State::Data);
                            return self.emit_current_token();
                        }
                        EOF_CHAR => panic!("eof-in-tag"),
                        _ => panic!("missing-whitespace-between-attributes"),
                    }
                }
            }
        }
    }

    fn new_attribute(&mut self, attribute: Attribute) {
        let token = self.current_token.as_mut().unwrap();
        if let Token::Tag {
            ref mut attributes, ..
        } = token
        {
            attributes.push(attribute)
        }
    }

    fn append_character_to_attribute_value(&mut self, ch: char) {
        let current_tag = self.current_token.as_mut().unwrap();
        if let Token::Tag {
            ref mut attributes, ..
        } = current_tag
        {
            let attribute = attributes.last_mut().unwrap();
            attribute.value.push(ch);
        }
    }

    fn append_character_to_attribute_name(&mut self, ch: char) {
        let current_tag = self.current_token.as_mut().unwrap();
        if let Token::Tag {
            ref mut attributes, ..
        } = current_tag
        {
            let attribute = attributes.last_mut().unwrap();
            attribute.name.push(ch);
        }
    }

    fn append_character_to_tag_name(&mut self, ch: char) {
        let current_tag = self.current_token.as_mut().unwrap();
        if let Token::Tag { tag_name, .. } = current_tag {
            tag_name.push(ch);
        } else {
            panic!("Attempted to push ch to a non-existent tag");
        }
    }

    fn emit_current_token(&self) -> Token {
        self.current_token.clone().unwrap()
    }

    fn reconsume_in(&mut self, state: State) {
        self.reconsume_char = true;
        self.switch_to(state);
    }

    fn switch_to(&mut self, state: State) {
        self.state = state;
    }

    fn emit_char(&self) -> Token {
        Token::Character(self.current_char)
    }

    fn emit_eof(&self) -> Token {
        Token::EOF
    }

    fn consume_next(&mut self) -> char {
        let ch = if self.reconsume_char {
            self.reconsume_char = false;
            self.current_char
        } else {
            let ch = self
                .chars
                .get(self.index)
                .map(|(_, ch)| *ch)
                .unwrap_or(EOF_CHAR);
            self.index += 1;
            ch
        };

        self.current_char = ch;
        ch
    }
}
