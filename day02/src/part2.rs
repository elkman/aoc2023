use std::cmp::max;

pub fn sum_game_powers(games_input: Vec<String>) -> i32 {
    // Initialize a variable to store the sum of possible game IDs
    let mut game_powers_sum = 0;

    // Iterate through each game
    for game_input in games_input {
        // Split the input into game ID and subsets of cubes
        let parts: Vec<&str> = game_input.split(":").collect();
        // game_id in parts[0] is not needed

        // Split subsets into individual cube counts
        let subsets: Vec<Vec<&str>> = parts[1]
            .split(';')
            .map(|subset| subset.trim().split(',')
                .flat_map(|col| col.trim().split(' '))
                .collect())
            .collect();

        // Initialize a variable to check if the game is possible
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        // Check each subset for cube counts
        for subset in subsets {
            for i in (0..subset.len()).step_by(2) {
                let count: i32 = subset[i].parse().unwrap(); // Extract the cube count
                let color = subset[i + 1]; // Extract the color (red, green, or blue)

                if color.eq("red") {
                    red = max(red, count);
                } else if color.eq("green") {
                    green = max(green, count);
                } else if color.eq("blue") {
                    blue = max(blue, count);
                } else { panic!("unknown color {}", color) }
            }
        }

        game_powers_sum += red * green * blue;
    }

    game_powers_sum
}
