fn is_star(c: char) -> bool {
    '*' == c
}

fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn detect_gear_ratio(engine: &[Vec<char>], row: usize, col: usize) -> u32 {
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

    let mut ratio: u32 = 1;
    let mut cnt: u32 = 0;
    let mut used: Vec<(isize, isize)> = Vec::new();

    for &(dr, dc) in directions.iter() {
        let new_row = row as isize + dr;
        let mut new_col = col as isize + dc;

        if new_row >= 0
            && new_col >= 0
            && new_row < engine.len() as isize
            && new_col < engine[0].len() as isize
            && is_digit(engine[new_row as usize][new_col as usize])
            && !used.contains(&(new_row, new_col))
        {
            cnt += 1;
            if cnt > 2 {
                panic!("found {} gears", cnt)
            }

            let mut c = new_col - 1;
            while c >= 0 && is_digit(engine[new_row as usize][c as usize]) {
                new_col = c;
                c -= 1;
            }
            let mut gear: u32 = 0;
            while new_col < engine[0].len() as isize
                && is_digit(engine[new_row as usize][new_col as usize])
            {
                gear = gear * 10
                    + engine[new_row as usize][new_col as usize]
                        .to_digit(10)
                        .unwrap();
                used.push((new_row, new_col));
                new_col += 1;
            }
            ratio *= gear;
        }
    }
    if cnt != 2 {
        ratio = 0
    }
    ratio
}

pub fn sum_gear_ratios(lines: Vec<String>) -> u32 {
    let engine: Vec<Vec<char>> = lines.iter().map(|s| s.chars().collect()).collect();
    let e = &engine;
    let mut total_sum = 0;

    for row in 0..engine.len() {
        for col in 0..engine[row].len() {
            if is_star(engine[row][col]) {
                total_sum += detect_gear_ratio(e, row, col);
            }
        }
    }
    total_sum
}