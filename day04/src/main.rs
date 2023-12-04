use shared::read_lines_from_file;

mod part1;
mod part2;

fn main() {
    let input1 = "src/input.txt"; // -> 27454
    let input2 = input1; // -> 6857330

    //let input1 = "src/input-test.txt"; // -> 13
    //let input2 = "src/input-test.txt"; // -> 30

    match read_lines_from_file(input1) {
        Ok(lines) => {
            println!(
                "aoc2023 - day 04 - part 1: points sum={} (=27454)",
                part1::sum_points(lines)
            );
        }
        Err(err) => eprintln!("Error reading file: {:?}", err),
    }

    match read_lines_from_file(input2) {
        Ok(lines) => {
            println!(
                "aoc2023 - day 04 - part 2: cards sum={} (=6857330)",
                part2::sum_cards(lines)
            );
        }
        Err(err) => eprintln!("Error reading file: {:?}", err),
    }
}
