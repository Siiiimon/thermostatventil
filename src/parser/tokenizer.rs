use std::collections::VecDeque;

use super::tokens::{self, Token};

pub struct Tokenizer {
    state: State,
    input: VecDeque<char>,
    current_char: char,
    next_char: char,
}

#[derive(Debug)]
enum State {
    Data,
    TagOpen,
    MarkupDeclarationOpen,
    EndTagOpen,
    TagName,
}

#[derive(Debug)]
enum ParseError {
    EndOfFile,
    UnexpectedNull,
}

impl Tokenizer {
    pub fn new(input: VecDeque<char>) -> Self {
        Self {
            state: State::Data,
            input,
            current_char: '\0',
            next_char: '\0',
        }
    }

    pub fn run(&mut self) -> VecDeque<Box<dyn Token>> {
        let mut output: VecDeque<Box<dyn Token>> = VecDeque::new();

        loop {
            let mut token = self.parse_next();
            match token {
                Ok(t) => output.push_back(t),
                Err(ParseError::EndOfFile) => break,
                Err(e) => panic!("{:?}", e),
            }
        }
        output
    }

    fn parse_next(&mut self) -> Result<Box<dyn Token>, ParseError> {
        let mut temp_token: Box<dyn Token>;
        loop {
            match self.state {
                State::Data => {
                    self.consume_next();
                    match self.current_char {
                        '&' => panic!("unimplemented"),
                        '<' => {
                            self.state = State::TagOpen;
                            continue;
                        }
                        '\0' => return Err(ParseError::UnexpectedNull),
                        _ => {
                            return Ok(Box::new(tokens::Character {
                                character: self.current_char,
                            }));
                        }
                    }
                }

                State::TagOpen => {
                    self.consume_next();
                    match self.current_char {
                        '!' => panic!("unimplemented"),
                        '/' => self.state = State::EndTagOpen,
                        '?' => panic!("unimplemented"),
                        _ => {
                            if !self.current_char.is_ascii_alphabetic() {
                                panic!("non-ascii-alpha tag name")
                            }

                            temp_token = Box::new(tokens::StartTag::new());
                            self.state = State::TagName;
                            self.reconsume();
                        }
                    }
                }

                State::TagName => {
                    self.consume_next();
                    match self.current_char {
                        // FIXME: add LF and FF
                        '\t' | ' ' => {
                            panic!("unimplemented")
                        }
                        '/' => panic!("unimplemented"),
                        '>' => {
                            self.state = State::Data;
                            return Ok(temp_token);
                        }
                        '\0' => {
                            return Err(ParseError::UnexpectedNull);
                        }
                        _ => match temp_token.as_any().downcast_mut::<tokens::StartTag>() {
                            Some(t) => t.name.push(self.current_char),
                            None => panic!("invalid temp token"),
                        },
                    }
                }

                _ => panic!("unimplemented state: {:?}", self.state),
            }
        }
    }

    fn consume_next(&mut self) {
        if let Some(c) = self.input.pop_front() {
            self.current_char = self.next_char;
            self.next_char = c;
        } else {
            panic!("prob at the end of characters rn");
        }
    }

    fn reconsume(&mut self) {
        self.input.push_front(self.next_char);
        self.next_char = self.current_char;
    }
}
