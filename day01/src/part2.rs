pub fn extract_calibration_values(input_lines: Vec<String>) -> u32 {
    let mut sum = 0;
    for line in input_lines {
        sum += extract_first_and_last_digit(&line);
    }
    sum
}

fn extract_first_and_last_digit(line: &str) -> u32 {
    let line_string = line.to_string();
    let mut first: u32 = 0;
    let mut last: u32 = 0;

    let digit_map = [
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    'search_from_left: for i in 0..line_string.len() {
        for &(word, digit) in digit_map.iter() {
            if line_string[i..].starts_with(word) {
                first = digit;
                break 'search_from_left;
            }
        }
    }

    'search_from_right: for i in (0..line_string.len()).rev() {
        for &(word, digit) in digit_map.iter() {
            if line_string[i..].starts_with(word) {
                last = digit;
                break 'search_from_right;
            }
        }
    }

    if first == 0 || last == 0 {
        panic!(
            "No digits found: line={}, first={}, last={}",
            line_string, first, last
        );
    }
    first * 10 + last
}
