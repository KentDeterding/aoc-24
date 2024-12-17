use std::fs::File;
use std::io::{BufReader, BufRead};
use anyhow::Result;

fn main() -> Result<()> {

    let file_path = "data-d02.txt";

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut safe_count = 0;

    'lines: for line in reader.lines() {
        let line = line?;

        let report: Vec<_> = line.split_whitespace().collect();

        let temp = report[0].parse::<i32>()? - report[1].parse::<i32>()?;
        if temp == 0 {
            continue;
        }
        let sign = temp / temp.abs();

        for i in 0..(report.len() - 1) {
            let diff = report[i].parse::<i32>()? - report[i+1].parse::<i32>()?;
            if diff * sign < 1 || 3 < diff * sign {
                continue 'lines;
            }
        }

        safe_count += 1;
    }

    println!("Part 1: {safe_count}");

    Ok(())
}
