use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use anyhow::Result;

fn main() -> Result<()> {

    let file_path = "data-d03.txt";
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut str = String::new();
    reader.read_to_string(&mut str).expect("Cannot read string");

    let mut sum = 0;

    let mut chars = str.chars().peekable();

    while chars.peek().is_some() {
        if chars.next() != Some('m') {
            continue;
        }
        if chars.next() != Some('u') {
            continue;
        }
        if chars.next() != Some('l') {
            continue;
        }
        if chars.next() != Some('(') {
            continue;
        }

        // find first number
        
        if chars.next() != Some(',') {
            continue;
        }

        // find second number
        // can't use next() here, gotta change it to use peek()
        if chars.next().unwrap().is_numeric() {

        }
        
        if chars.next() != Some(')') {
            continue;
        }
    }

    Ok(())
}
