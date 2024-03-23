use std::collections::HashMap;

pub fn compute_results(input: Vec<String>) -> (u32, String) {
    return (solve(&input), solve_part2(&input));
}

fn solve_part2(instructions: &Vec<String>) -> String {
    for box1 in instructions {
        for box2 in instructions {
            if boxes_are_prototypes(&box1, &box2) {
                return format!("{} + {}", box1, box2);
            }
        }
    }
    return "fonction not working".to_string();
}

fn boxes_are_prototypes(box1: &str, box2: &str) -> bool {
    return box1
        .chars()
        .zip(box2.chars())
        .fold(0, |num_of_diffrence, curr_chars| {
            num_of_diffrence + ((curr_chars.0 != curr_chars.1) as u32)
        })
        == 1;
}

fn solve(instructions: &Vec<String>) -> u32 {
    let occurences: (u32, u32) =
        instructions
            .iter()
            .fold((0, 0), |occurences: (u32, u32), current_box| {
                (
                    occurences.0 + box_is_two_occurence(current_box),
                    occurences.1 + box_is_three_occurence(current_box),
                )
            });
    return occurences.0 * occurences.1;
}

fn box_is_three_occurence(current_box: &str) -> u32 {
    let letters_map = get_letter_map(current_box);
    return letters_map.iter().any(|a| *a.1 == 2) as u32;
}
fn box_is_two_occurence(current_box: &str) -> u32 {
    let letters_map = get_letter_map(current_box);
    return letters_map.iter().any(|a| *a.1 == 3) as u32;
}

fn get_letter_map(current_box: &str) -> HashMap<String, u32> {
    let letters_map: HashMap<String, u32> =
        current_box
            .chars()
            .fold(HashMap::new(), |mut letters_map, current_char| {
                if letters_map.contains_key(&current_char.to_string()) {
                    letters_map.insert(
                        current_char.to_string(),
                        letters_map[&current_char.to_string()] + 1,
                    );
                } else {
                    letters_map.insert(current_char.to_string(), 1);
                }
                return letters_map;
            });
    letters_map
}
