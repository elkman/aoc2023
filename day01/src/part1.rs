pub fn extract_calibration_values(input_lines: Vec<String>) -> u32 {
    let mut sum = 0;

    for line in input_lines {
        let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();

        if let (Some(first_digit), Some(last_digit)) =
            (digits.first().cloned(), digits.last().cloned())
        {
            //println!("1={}, 2={} ({})", first_digit, last_digit, line);
            let calibration_value = first_digit * 10 + last_digit;
            sum += calibration_value;
        }
    }

    sum
}
