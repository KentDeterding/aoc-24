use std::fs::File;
use std::io::{BufReader, BufRead};
use anyhow::Result;

fn main() -> Result<()> {

    let file_path = "data-d01.txt";

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut left_list = Vec::<i32>::new();
    let mut right_list = Vec::<i32>::new();

    for line in reader.lines() {
        let line = line?;

        let parts: Vec<_> = line.split_whitespace().collect();
        left_list.push(parts[0].parse()?);
        right_list.push(parts[1].parse()?);
    }

    left_list.sort();
    right_list.sort();

    let mut difference = 0;

    for i in 0..(left_list.len()) {
        difference += (left_list[i] - right_list[i]).abs();
    }

    println!("Part 1: {difference}");

    let mut similarity = 0;

    for i in left_list {
        let mut count = 0;
        for j in &right_list {
            if i == *j {
                count += 1;
            }
        }
        similarity += i * count;
    }

    println!("Part 2: {similarity}");

    Ok(())
}
