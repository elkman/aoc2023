use std::collections::HashSet;

pub fn sum_cards(lines: Vec<String>) -> u32 {
    let mut card_counts: Vec<u32> = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        let parts: Vec<&str> = line.split(&[':', '|']).map(|s| s.trim()).collect();
        //let card = parts[0];
        let left = parts[1];
        let right = parts[2];

        let left_nos: HashSet<i32> = left
            .split_whitespace()
            .filter_map(|token| token.parse().ok())
            .collect();

        let matches = right
            .split_whitespace()
            .filter_map(|token| token.parse().ok())
            .filter(|no| left_nos.contains(no))
            .count() as u32;

        // add original
        add_cards(&mut card_counts, i, 1);

        // add next matches
        for next_card in 0..matches as usize {
            let factor = card_counts[i];
            add_cards(&mut card_counts, i + next_card + 1, factor);
        }
    }
    card_counts.iter().sum()
}

fn add_cards(card_counts: &mut Vec<u32>, idx: usize, val: u32) {
    if card_counts.len() == idx {
        card_counts.push(val);
    } else {
        card_counts[idx] += val;
    }
}
