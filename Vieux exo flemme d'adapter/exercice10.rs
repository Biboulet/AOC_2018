use itertools::Itertools;
use std::{
    cmp::max,
    cmp::{self, min},
    collections::{HashMap, HashSet},
};

pub fn compute_results(input: Vec<String>) -> (u32, u32) {
    let instructions = parse_input(input);
    return (solve(&instructions, 10), 0);
}

fn solve(instructions: &Vec<((i32, i32), (i32, i32))>, max_sec: u32) -> u32 {
    let mut all_stars = instructions.clone();
    for second in 0..u32::MAX {
        Move_stars(&mut all_stars);
        let min_y = all_stars.iter().map(|a| a.0 .1).min().unwrap();
        let max_y = all_stars.iter().map(|a| a.0 .1).max().unwrap();
        if looks_observable(&min_y, &max_y) {
            print_stars(&all_stars, &min_y, &max_y, &second);
        }
    }

    return 0;
}

fn looks_observable(min: &i32, max: &i32) -> bool {
    return max - min < 12;
}

fn print_stars(all_stars: &[((i32, i32), (i32, i32))], min_y: &i32, max_y: &i32,second: &u32) {
    let max_x = all_stars.iter().map(|a| a.0 .0).max().unwrap();

    let min_x = all_stars.iter().map(|a| a.0 .0).min().unwrap();
    for y in *min_y..*max_y +1{
        let mut line = String::new();
        for x in min_x..max_x +1{
            let curr_coord = (x, y);
            if all_stars.iter().map(|a| a.0).contains(&curr_coord) {
                line.push('#');
            } else {
                line.push('.')
            }
        }
        println!("{}", line);
    }
}

fn Move_stars(all_stars: &mut Vec<((i32, i32), (i32, i32))>) {
    *all_stars = all_stars
        .iter()
        .map(|curr_star| {
            return (
                (
                    (curr_star.0 .0 + curr_star.1 .0),
                    (curr_star.0 .1 + curr_star.1 .1),
                ),
                (curr_star.1 .0, curr_star.1 .1),
            );
        })
        .collect_vec();
}

fn parse_input(input: Vec<String>) -> Vec<((i32, i32), (i32, i32))> {
    return input
        .iter()
        .map(|line| {
            return (
                (
                    line[10..16].replace(" ", "").parse().unwrap(),
                    line[18..24].replace(" ", "").parse().unwrap(),
                ),
                (
                    line[36..38].replace(" ", "").parse().unwrap(),
                    line[40..42].replace(" ", "").parse().unwrap(),
                ),
            );
        })
        .collect_vec();
}
