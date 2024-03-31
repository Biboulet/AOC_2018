use std::cmp::max;

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<String> {
    let input_answer_offset = input.parse::<u32>().unwrap();
    Some(solve(input_answer_offset, vec![3, 7]))
}
pub fn part_two(input: &str) -> Option<u32> {
    let input_answer_offset = input.parse::<u32>().unwrap();
    Some(solve2(input_answer_offset, vec![3, 7]))
}

fn solve2(input: u32, starting_state: Vec<u8>) -> u32 {
    let len_of_input = input.to_string().len();
    let mut queue: Vec<u8> = starting_state.clone();
    let mut p1: usize = 0;
    let mut p2: usize = 1;
    let mut index_tested = 0;
    loop {
        let new_recipe = queue[p1] + queue[p2];
        if new_recipe >= 10 {
            queue.push(1);
            queue.push(new_recipe % 10);
        } else {
            queue.push(new_recipe);
        }

        p1 = (p1 + 1 + queue[p1] as usize) % queue.len();
        p2 = (p2 + 1 + queue[p2] as usize) % queue.len();

        for starting_index_pattern in
            index_tested..max(0, queue.len() as i32 - len_of_input as i32) as usize
        {
            let pattern: u32 = (0..len_of_input)
                .map(|i| {
                    queue[i + starting_index_pattern] as u32 * 10_u32.pow((len_of_input - i - 1) as u32)
                })
                .sum();
            if pattern == input {
                return starting_index_pattern as u32;
            }
        }
        index_tested = max(0, queue.len() as i32 - len_of_input as i32) as usize;
    }
}

fn solve(input_answer_offset: u32, starting_state: Vec<u8>) -> String {
    let mut queue: Vec<u8> = starting_state.clone();
    let mut p1: usize = 0;
    let mut p2: usize = 1;
    while queue.len() < (input_answer_offset + 10) as usize {
        let new_recipe = queue[p1] + queue[p2];
        if new_recipe >= 10 {
            queue.push(1);
            queue.push(new_recipe % 10);
        } else {
            queue.push(new_recipe);
        }

        p1 = (p1 + 1 + queue[p1] as usize) % queue.len();
        p2 = (p2 + 1 + queue[p2] as usize) % queue.len();
    }

    return (input_answer_offset..input_answer_offset + 10)
        .map(|a| queue[a as usize].to_string())
        .collect::<Vec<String>>()
        .join("");
}
