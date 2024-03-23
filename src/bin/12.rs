use std::collections::HashSet;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<i64> {
    let (initial_state, instructions) = parse_input(input);
    Some(simulate(initial_state, instructions, 20))
}
pub fn part_two(input: &str) -> Option<i64> {
    let (initial_state, instructions) = parse_input(input);
    Some(simulate(initial_state, instructions, 50000000000))
}

fn simulate(initial_state: HashSet<i32>, instructions: Vec<(Vec<bool>, bool)>, cycle: i64) -> i64 {
    let mut current_state = initial_state.clone();
    let mut previous_score_growth_rate = current_state.iter().sum::<i32>();
    for k in 0..cycle {
        let mut new_state = HashSet::new();
        for plant_index in
            *current_state.iter().min().unwrap() - 2..=*current_state.iter().max().unwrap() + 2
        {
            let situation: Vec<bool> = (plant_index - 2..=plant_index + 2)
                .map(|i| current_state.contains(&i))
                .collect();

            if let Some(valid_instr) = instructions.iter().find(|(vec, _)| vec.eq(&situation)) {
                if valid_instr.1 {
                    new_state.insert(plant_index);
                } else {
                    new_state.remove(&(plant_index));
                }
            }
        }

        let new_score_growth_rate =
            new_state.iter().sum::<i32>() - current_state.iter().sum::<i32>();
        //growth rate is stable
        if new_score_growth_rate == previous_score_growth_rate {
            return new_state.iter().sum::<i32>() as i64
                + new_score_growth_rate as i64 * (cycle - k - 1);
        }
        previous_score_growth_rate = new_score_growth_rate;
        current_state = new_state;
    }
    return current_state.iter().sum::<i32>() as i64;
}

fn parse_input(input: &str) -> (HashSet<i32>, Vec<(Vec<bool>, bool)>) {
    let initial_state: HashSet<i32> = input
        .split("\r\n")
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .chars()
        .enumerate()
        .fold(HashSet::new(), |mut set, (i, char)| {
            if char == '#' {
                set.insert(i as i32);
            }
            set
        });

    let instructions: Vec<(Vec<bool>, bool)> = input
        .split("\r\n\r\n")
        .nth(1)
        .unwrap()
        .split('\n')
        .map(parse_line)
        .collect();
    (initial_state, instructions.clone())
}

fn parse_line(line: &str) -> (Vec<bool>, bool) {
    let args: Vec<&str> = line.trim().split(" => ").collect();
    return (
        args[0].trim().chars().map(|char| char == '#').collect(),
        args[1] == "#",
    );
}
