use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn read_lines_from_file(file_path: &str) -> io::Result<Vec<String>> {
    // Open the file
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().filter_map(|line| line.ok()).collect();
    Ok(lines)
}
