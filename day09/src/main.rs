use shared::read_lines_from_file;

mod part1;
mod part2;

fn main() {
    let input1 = "day09/src/input.txt"; //
    let input2 = input1; // ->

    //let input1 = "day09/src/input-test.txt"; // -> 114
    //let input2 = "day09/src/input-test.txt"; // -> 2

    match read_lines_from_file(input1) {
        Ok(lines) => {
            println!(
                "aoc2023 - day 09 - part 1: sum={} (=1868368343)",
                part1::sum(lines)
            );
        }
        Err(err) => eprintln!("Error reading file: {:?}", err),
    }

    match read_lines_from_file(input2) {
        Ok(lines) => {
            println!(
                "aoc2023 - day 09 - part 2: sum={} (=1022)",
                part2::sum(lines)
            );
        }
        Err(err) => eprintln!("Error reading file: {:?}", err),
    }
}
