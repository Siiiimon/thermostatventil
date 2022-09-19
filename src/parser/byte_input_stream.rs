// TODO: §13.2.3

use std::io::{BufReader, Error, Read};

#[derive(Debug)]
pub enum Encoding {
    Utf8,
}

#[derive(Debug)]
pub enum Confidence {
    Certain,
    Tentative,
    Irrelevant,
}

pub struct ByteStreamDecoder<I: Read> {
    input: BufReader<I>,
    encoding: Encoding,
    confidence: Confidence,
    // TODO: insertion_point,
}

impl<I: Read> ByteStreamDecoder<I> {
    pub fn initialize(in_stream: I) -> Self {
        let input = BufReader::new(in_stream);
        let (encoding, confidence) = Self::sniff(input);
        println!(
            "sniffed {:?} encoding with {:?} confidence",
            encoding, confidence
        );

        Self {
            input,
            encoding,
            confidence,
        }
    }

    pub fn decode(&mut self) -> Option<char> {
        // decode byte to codepoint using `encoding`
        // pass resulting codepoint through pre-processor (becoming a character)
        let mut buf = [0];
        if let Ok(n) = self.input.read(&mut buf) {
            if n > 0 {
                return Some(buf[0] as char);
            }
        }
        None
    }

    fn sniff(input: BufReader<I>) -> (Encoding, Confidence) {
        // TODO: implement BOM sniffing (ref: §13.2.3.2)
        (Encoding::Utf8, Confidence::Certain)
    }
}
