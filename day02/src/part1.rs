use std::collections::HashMap;

pub fn sum_game_ids(games_input: Vec<String>) -> i32 {
    // Define the cube counts
    let target_counts = [("red", 12), ("green", 13), ("blue", 14)]
        .iter()
        .cloned()
        .collect::<HashMap<_, _>>();

    // Initialize a variable to store the sum of possible game IDs
    let mut possible_game_sum = 0;

    // Iterate through each game
    for game_input in games_input {
        // Split the input into game ID and subsets of cubes
        let parts: Vec<&str> = game_input.split(":").collect();
        let game_id: i32 = parts[0].split_whitespace().last().unwrap().parse().unwrap();

        // Split subsets into individual cube counts
        let subsets: Vec<Vec<&str>> = parts[1]
            .split(';')
            .map(|subset| {
                subset
                    .trim()
                    .split(',')
                    .flat_map(|col| col.trim().split(' '))
                    .collect()
            })
            .collect();

        // Initialize a variable to check if the game is possible
        let mut is_possible = true;

        // Check each subset for cube counts
        'out: for subset in subsets {
            for i in (0..subset.len()).step_by(2) {
                let count: i32 = subset[i].parse().unwrap(); // Extract the cube count
                let color = subset[i + 1]; // Extract the color (red, green, or blue)

                // Check if the count is consistent with the target counts
                if let Some(&target_count) = target_counts.get(color) {
                    if target_count < count {
                        is_possible = false;
                        break 'out;
                    }
                } else {
                    // Color not found in target_counts
                    is_possible = false;
                    break 'out;
                }
            }
        }

        // If the game is possible, add its ID to the sum
        if is_possible {
            possible_game_sum += game_id;
        }
    }

    possible_game_sum
}
