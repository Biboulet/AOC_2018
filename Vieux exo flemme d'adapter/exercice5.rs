use std::cmp::min;

pub fn compute_results(input: Vec<String>) -> (u32, u32) {
    return (solve(&input[0]), solve_part2(&input[0]));
}

fn solve_part2(input: &str) -> u32 {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let final_length: Vec<u32> = alphabet
        .chars()
        .map(|current_char| {
            let local_length = solve(
                input
                    .replace(current_char, "")
                    .replace(current_char.to_ascii_uppercase(), "").as_str(),
            );
            dbg!(local_length);
            return local_length;
        })
        .collect::<Vec<u32>>();
    return final_length.iter().fold(u32::max_value(), |minimum_length, current_val| min(minimum_length, *current_val));
}

fn solve(input: &str) -> u32 {
    let mut is_finished = false;
    let mut current_polymer = input.to_string();
    while !is_finished {
        let next_polymer = reduce_polymer(current_polymer.clone());
        is_finished = next_polymer == current_polymer;
        current_polymer = next_polymer.clone();
    }
    return current_polymer.len() as u32;
}

fn reduce_polymer(previous_polymer: String) -> String {
    let new_polymer: Option<String> = previous_polymer
        .chars()
        .zip(previous_polymer[1..].chars())
        .enumerate()
        .find_map(|args| {
            if polymer_compatible(args.1 .0, args.1 .1) {
                return Some(
                    previous_polymer
                        .get(..args.0)
                        .expect("index higher than string length")
                        .to_string()
                        + previous_polymer
                            .get((args.0 + 2)..)
                            .expect("index higher than string length"),
                );
            } else {
                return None;
            }
        });
    if new_polymer.is_none() {
        return previous_polymer;
    } else {
        return new_polymer.unwrap();
    }
}

fn polymer_compatible(char1: char, char2: char) -> bool {
    return (char1.is_uppercase() ^ char2.is_uppercase())
        & (char1.to_ascii_lowercase() == char2.to_ascii_lowercase());
}
