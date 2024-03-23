use std::vec;

use itertools::Itertools;
const MIN: i32 = -10;
const MAX: i32 = 10;


pub fn compute_results(input: Vec<String>) -> (u32, u32) {
    let nanobots: Vec<((i32, i32, i32), usize)> = parse_input(input);
    return (solve_p1(&nanobots), solve_p2(&nanobots));
}

fn solve_p2(nanobots: &[((i32, i32, i32), usize)]) -> u32 {
    let mut best_position: ((i32, i32, i32), u32) = ((0, 0, 0), 0);
    for x in MIN..=MAX {
        for y in MIN..=MAX {
            for z in MIN..=MAX {
                let coords = (x, y, z);
                let score = get_num_of_nano_bots_in_range(nanobots, coords);
                if score < best_position.1 {
                    best_position = (coords, score);
                }
                else if score == best_position.1{
                    if man_dist((0, 0, 0), coords) < man_dist((0, 0, 0), best_position.0){
                        best_position = (coords, score);
                    }
                }
            }
        }
    }
    return man_dist((0, 0, 0), best_position.0);
}

fn get_num_of_nano_bots_in_range(nanobots: &[((i32, i32, i32), usize)], coords: (i32, i32, i32)) -> u32 {
    return nanobots.iter().fold(0, |score, curr_nanobot| {
        if is_at_range(curr_nanobot, &coords) {
            return score + 1;
        } else {
            return score;
        }
    });
}

fn solve_p1(nanobots: &[((i32, i32, i32), usize)]) -> u32 {
    let strongest_nanobot = nanobots
        .iter()
        .fold(nanobots[0], |best_nanobot, curr_nanobot| {
            if curr_nanobot.1 > best_nanobot.1 {
                return *curr_nanobot;
            } else {
                return best_nanobot;
            }
        });

    return nanobots.iter().fold(0, |score, curr_nanobot| {
        if is_at_range(&strongest_nanobot, &curr_nanobot.0) {
            return score + 1;
        } else {
            return score;
        }
    });
}

fn is_at_range(
    nanobot: &((i32, i32, i32), usize),
    sub_nanobot_pos: &(i32, i32, i32),
) -> bool {
    return man_dist(nanobot.0, *sub_nanobot_pos) <= nanobot.1 as u32;
}

fn man_dist(curr_nanobot: (i32, i32, i32), curr_sub_nanobot: (i32, i32, i32)) -> u32 {
    return curr_nanobot.0.abs_diff(curr_sub_nanobot.0)
        + curr_nanobot.1.abs_diff(curr_sub_nanobot.1)
        + curr_nanobot.2.abs_diff(curr_sub_nanobot.2);
}

fn parse_input(input: Vec<String>) -> Vec<((i32, i32, i32), usize)> {
    return input
        .iter()
        .map(|line| {
            let coords: Vec<i32> = line
                .split("pos=<")
                .nth(1)
                .unwrap()
                .split(">,")
                .nth(0)
                .unwrap()
                .split(",")
                .map(|a| a.parse::<i32>().unwrap())
                .collect_vec();

            let radius = line.split("r=").nth(1).unwrap().parse().unwrap();
            return ((coords[0], coords[1], coords[2]), radius);
        })
        .collect_vec();
}
