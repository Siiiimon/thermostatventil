use std::any::Any;

pub trait Token {
    fn as_any(&self) -> &dyn Any;
}

pub struct DocType {
    name: Option<String>,
    system_id: Option<u32>,
    public_id: Option<u32>,
    force_quirks: bool,
}

impl Token for DocType {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl DocType {
    pub fn new() -> Self {
        DocType {
            name: None,
            public_id: None,
            system_id: None,
            force_quirks: false,
        }
    }
}

pub struct StartTag {
    pub name: String,
    is_self_closing: bool,
    // FIXME: attributes
}

impl Token for StartTag {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl StartTag {
    pub fn new() -> Self {
        StartTag {
            name: String::new(),
            is_self_closing: false,
        }
    }
}

pub struct EndTag {
    name: String,
    is_self_closing: bool,
    // FIXME: attributes
}

impl Token for EndTag {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl EndTag {
    pub fn new() -> Self {
        EndTag {
            name: String::new(),
            is_self_closing: false,
        }
    }
}

pub struct Character {
    pub character: char,
}

impl Token for Character {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct Comment {
    pub comment: String,
}

impl Token for Comment {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
