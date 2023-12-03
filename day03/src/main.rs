use shared::read_lines_from_file;

mod part1;
mod part2;

fn main() {
    let input1 = "src/input.txt"; // -> 535078
    let input2 = input1; // -> 75312571

    //let input1 = "src/input-test.txt"; // -> 4361
    //let input2 = input1; // -> 467835

    match read_lines_from_file(input1) {
        Ok(lines) => {
            println!(
                "aoc2023 - day 03 - part 1: part numbers sum={} (=535078)",
                part1::sum_part_numbers(lines)
            );
        }
        Err(err) => eprintln!("Error reading file: {:?}", err),
    }

    match read_lines_from_file(input2) {
        Ok(lines) => {
            println!(
                "aoc2023 - day 03 - part 2: gear ratios sum={} (=75312571)",
                part2::sum_gear_ratios(lines)
            );
        }
        Err(err) => eprintln!("Error reading file: {:?}", err),
    }
}
