use std::fs::File;
use std::io::{self, BufReader, BufRead};

mod part1;
mod part2;

fn main() {
    let input1 = "src/input.txt"; // -> 54601
    let input2= input1; // -> 54078
    // let input1 = "src/input-test1.txt"; // -> 1: 142
    // let input2 = "src/input-test2.txt"; // -> 2: 281

    match read_lines_from_file(input1) {
        Ok(lines) => {
            println!("aoc2023 - day 01 - part 1: total calibration sum={} (=54601)",
                     part1::extract_calibration_values(lines));
        }
        Err(err) => eprintln!("Error reading file: {:?}", err),
    }

    match read_lines_from_file(input2) {
        Ok(lines) => {
            println!("aoc2023 - day 01 - part 2: total calibration sum={} (=54078)",
                     part2::extract_calibration_values(lines));
        }
        Err(err) => eprintln!("Error reading file: {:?}", err),
    }
}

fn read_lines_from_file(file_path: &str) -> io::Result<Vec<String>> {
    // Open the file
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().filter_map(|line| line.ok()).collect();
    Ok(lines)
}