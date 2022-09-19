use std::fs::File;
use std::io::prelude::*;

pub mod parser;

fn main() -> std::io::Result<()> {
    let file = File::open("assets/test.html")?;

    Ok(())
}
