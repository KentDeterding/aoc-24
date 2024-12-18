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

    // Part 2
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut safe_count = 0;

    for line in reader.lines() {
        let line = line?;
        let report: Vec<_> = line
            .split_whitespace()
            .filter_map(|i| i.parse::<i32>().ok())
            .collect();
        
        if check_safety(report.clone()) {
            safe_count += 1;
        } else {
            for i in 0..report.len() {
                let mut dampened_report = report.clone();
                dampened_report.remove(i);

                if check_safety(dampened_report) {
                    safe_count += 1;
                    break;
                }
            }
        }
    }

    println!("Part 2: {safe_count}");

    Ok(())
}

fn check_safety(report: Vec<i32>) -> bool {
    let temp = report[0] - report[report.len() - 1];
    if temp == 0 {
        return false
    }
    let sign = temp / temp.abs();
    let mut safety = true;

    for i in 0..(report.len() - 1) {
        let diff = report[i] - report[i+1];
        if diff * sign < 1 || 3 < diff * sign {
            safety = false;
        }
    }

    safety
}
