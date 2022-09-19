use crate::parser::tokenizer::Tokenizer;

use self::byte_input_stream::ByteStreamDecoder;
use std::{collections::VecDeque, io::Read};

pub mod byte_input_stream;
pub mod tokenizer;
pub mod tokens;

pub struct HTMLParser {}

impl HTMLParser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse<I: Read>(input: I) {
        println!("initializing decoder");
        let mut decoder = ByteStreamDecoder::initialize(input);
        // TODO: check for user-specified encoding overwrites

        // this can probably be done in a seperate thread
        // or at least in parallel to tokenization
        // ideally we'd use something like Go's channels for this
        let char_queue: VecDeque<char> = VecDeque::new();
        loop {
            let c = decoder.decode();
            match c {
                Some(c) => char_queue.push_back(c),
                None => break,
            }
        }
        println!("done decoding, starting tokenizer");
        let tokenizer = Tokenizer::new(char_queue);
        tokenizer.run();
    }
}
