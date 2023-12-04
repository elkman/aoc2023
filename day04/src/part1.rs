use std::collections::HashSet;

pub fn sum_points(lines: Vec<String>) -> u32 {
    let mut sum = 0;
    for line in lines {
        let parts: Vec<&str> = line.split(&[':', '|']).map(|s| s.trim()).collect();
        //let card = parts[0];
        let left = parts[1];
        let right = parts[2];

        let left_nos: HashSet<i32> = left
            .split_whitespace()
            .filter_map(|token| token.parse().ok())
            .collect();

        let matches = right
            .split_whitespace()
            .filter_map(|token| token.parse().ok())
            .filter(|no| left_nos.contains(no))
            .count() as u32;

        let points: u32 = if matches < 2 {
            matches
        } else {
            2u32.pow(matches - 1)
        };
        sum += points;
    }
    sum
}
