use std::fs;

fn main() {
    let input1 = "day06/src/input1.txt";
    //let input1 = "day06/src/input-test1.txt"; // -> 288

    let input2 = "day06/src/input2.txt";
    //let input2 = "day06/src/input-test2.txt"; // -> 71503

    println!(
        "aoc2023 - day 06 - part 1: result={} (=440000)",
        calc_product_of_ways(input1)
    );

    println!(
        "aoc2023 - day 06 - part 2: result={} (=26187338)",
        calc_product_of_ways(input2)
    );
}

pub fn calc_product_of_ways(file: &str) -> u64 {
    // Read input from a file named "input1.txt"
    let input = fs::read_to_string(file).expect("Error reading file");

    // Split input into lines
    let lines: Vec<&str> = input.lines().collect();

    // Extract times and distances separately
    let times: Vec<u64> = lines[0][10..]
        .split_whitespace()
        .map(|s| s.parse().expect("Error parsing time"))
        .collect();
    let distances: Vec<u64> = lines[1][10..]
        .split_whitespace()
        .map(|s| s.parse().expect("Error parsing distance"))
        .collect();

    // Initialize a variable to store the result
    let mut result = 1;

    // Iterate through each race
    for (&time, &distance) in times.iter().zip(distances.iter()) {
        let mut ways_to_win: u64 = 0;

        for t in 1..time {
            let d = t * (time - t);
            if d > distance {
                ways_to_win += 1;
            }
        }

        result *= ways_to_win;
    }

    result
}
