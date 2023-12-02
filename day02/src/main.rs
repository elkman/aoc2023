use shared::read_lines_from_file;

mod part1;
mod part2;

fn main() {
    let input1 = "src/input.txt"; // -> 2476
    let input2= input1; // -> 54078
    // let input1 = "src/input-test.txt"; // -> 1: 8
    // let input2 = input1; // -> 2286

    match read_lines_from_file(input1) {
        Ok(lines) => {
            println!("aoc2023 - day 02 - part 1: game ids sum={} (=2476)",
                     part1::sum_game_ids(lines));
        }
        Err(err) => eprintln!("Error reading file: {:?}", err),
    }

    match read_lines_from_file(input2) {
        Ok(lines) => {
            println!("aoc2023 - day 01 - part 2: game powers sum={} (=54911)",
                     part2::sum_game_powers(lines));
        }
        Err(err) => eprintln!("Error reading file: {:?}", err),
    }
}
