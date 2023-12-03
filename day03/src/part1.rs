fn is_symbol(c: char) -> bool {
    '.' != c && !c.is_ascii_digit()
}

fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn detect_symbol(engine: &[Vec<char>], row: usize, col: usize) -> bool {
    let directions = [
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, 1),
        (0, -1),
        (1, 1),
        (1, 0),
        (1, -1),
    ];

    for &(dr, dc) in directions.iter() {
        let new_row = row as isize + dr;
        let new_col = col as isize + dc;

        if new_row >= 0
            && new_col >= 0
            && new_row < engine.len() as isize
            && new_col < engine[0].len() as isize
            && is_symbol(engine[new_row as usize][new_col as usize])
        {
            return true;
        }
    }
    false
}

pub fn sum_part_numbers(lines: Vec<String>) -> u32 {
    let engine: Vec<Vec<char>> = lines.iter().map(|s| s.chars().collect()).collect();
    let e = &engine;
    let mut total_sum = 0;

    for row in 0..engine.len() {
        let mut number: u32 = 0;
        let mut has_symbol = false;
        for col in 0..engine[row].len() {
            let cell = engine[row][col];
            if is_digit(cell) {
                number = number * 10 + cell.to_digit(10).unwrap();
                has_symbol |= detect_symbol(e, row, col);
            } else {
                if has_symbol && number > 0 {
                    total_sum += number;
                }
                has_symbol = false;
                number = 0;
            }
        }
        if has_symbol && number > 0 {
            total_sum += number;
        }
    }
    total_sum
}
