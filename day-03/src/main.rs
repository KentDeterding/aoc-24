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

    // Part 1

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
        let mut num_str = String::new();
        while chars.peek().unwrap().is_numeric() {
            num_str.push(chars.next().unwrap());
        }
        let mut num_1 = 0;
        if num_str.len() > 0 && num_str.len() <= 3 {
            num_1 = num_str.parse::<i32>()?;
        }
        
        if chars.next() != Some(',') {
            continue;
        }

        // find second number
        let mut num_str = String::new();
        while chars.peek().unwrap().is_numeric() {
            num_str.push(chars.next().unwrap());
        }
        let mut num_2 = 0;
        if num_str.len() > 0 && num_str.len() <= 3 {
            num_2 = num_str.parse::<i32>()?;
        }
        
        if chars.next() != Some(')') {
            continue;
        }

        sum += num_1 * num_2;
    }

    println!("Part 1: {sum}");


    // Part 2

    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut str = String::new();
    reader.read_to_string(&mut str).expect("Cannot read string");
    let mut chars = str.chars().peekable();

    sum = 0;
    let mut enabled = true;

    while chars.peek().is_some() {
        match chars.next() {
            Some('m') => {
                if !enabled {
                    continue
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
                let mut num_str = String::new();
                while chars.peek().unwrap().is_numeric() {
                    num_str.push(chars.next().unwrap());
                }
                let mut num_1 = 0;
                if num_str.len() > 0 && num_str.len() <= 3 {
                    num_1 = num_str.parse::<i32>()?;
                }
                
                if chars.next() != Some(',') {
                    continue;
                }

                // find second number
                let mut num_str = String::new();
                while chars.peek().unwrap().is_numeric() {
                    num_str.push(chars.next().unwrap());
                }
                let mut num_2 = 0;
                if num_str.len() > 0 && num_str.len() <= 3 {
                    num_2 = num_str.parse::<i32>()?;
                }
                
                if chars.next() != Some(')') {
                    continue;
                }

                sum += num_1 * num_2;
            },
            Some('d') => {
                if chars.next().unwrap() != 'o' {
                    continue;
                }

                match chars.next().unwrap() {
                    '(' => {
                        if chars.next().unwrap() == ')' {
                            enabled = true;
                        }
                    },
                    'n' => {
                        if chars.next().unwrap() != '\'' {
                            continue
                        }
                        if chars.next().unwrap() != 't' {
                            continue
                        }
                        if chars.next().unwrap() != '(' {
                            continue
                        }
                        if chars.next().unwrap() != ')' {
                            continue
                        }
                        enabled = false;
                    },
                    _ => continue
                }
            },
            _ => continue
        }
    }

    println!("Part 2: {sum}");

    Ok(())
}
