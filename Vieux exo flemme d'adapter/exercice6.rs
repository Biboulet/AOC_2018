use itertools::Itertools;
use std::{
    cmp::max,
    collections::{HashMap, HashSet},
};

const MAX_SIZE_MAP: u32 = 400;
const MIN_SIZE_MAP: u32 = 0;
const MAX_DISTANCE: u32 = 10000;

pub fn compute_results(input: Vec<String>) -> (u32, u32) {
    let instructions = parse_input(input);
    return (solve(&instructions), solve_part2(&instructions));
}

fn solve_part2(instructions: &Vec<(u32, u32)>) -> u32 {
    let map: HashMap<(u32, u32), u32> = (0..=2)
        .map(|_| MIN_SIZE_MAP..=MAX_SIZE_MAP)
        .multi_cartesian_product()
        .map(|coords| {
            let coords_tuple = (coords[0] as u32, coords[1] as u32);
            (
                coords_tuple,
                sum_of_all_dist_from_instr_to_coord(&instructions, coords_tuple),
            )
        })
        .collect::<HashMap<(u32, u32), u32>>();

    return map.values().fold(0, |count, curr_value| {
        if curr_value < &MAX_DISTANCE {
            return count + 1;
        } else {
            return count;
        }
    });
}

fn sum_of_all_dist_from_instr_to_coord(instructions: &[(u32, u32)], coords: (u32, u32)) -> u32 {
    return instructions
    .iter()
    .map(|value| man_dist(&value, coords))
    .sum();

}

fn solve(instructions: &Vec<(u32, u32)>) -> u32 {
    // fill each coords of map (0 to 400) with : -1 for equality and index of the point for the closest point to the square
    let map: HashMap<(u32, u32), i32> = (0..=2)
        .map(|_| MIN_SIZE_MAP..=MAX_SIZE_MAP)
        .multi_cartesian_product()
        .map(|coords| {
            let coords_tuple = (coords[0] as u32, coords[1] as u32);
            (
                coords_tuple,
                closest_point_index(&instructions, coords_tuple),
            )
        })
        .collect::<HashMap<(u32, u32), i32>>();

    let infinite_areas: Vec<i32> =
        [get_all_points_with_infinite_area(&map), [-1].to_vec()].concat();
    let all_areas = map
        .values()
        .fold(vec![0; instructions.len()], |mut all_areas, owner_index| {
            if !infinite_areas.contains(&owner_index) {
                all_areas[(*owner_index as u32) as usize] =
                    all_areas[(*owner_index as u32) as usize] + 1;
            }
            return all_areas;
        });

    return all_areas.iter().fold(0, |largest_area, current_area| {
        max(largest_area, *current_area)
    });
}

fn get_all_points_with_infinite_area(map: &HashMap<(u32, u32), i32>) -> Vec<i32> {
    return map
        .iter()
        .fold(
            HashSet::new(),
            |mut infinte_points: HashSet<i32>, current_arg| {
                if is_edge(current_arg.0) {
                    infinte_points.insert(*current_arg.1);
                }
                return infinte_points;
            },
        )
        .into_iter()
        .collect::<Vec<i32>>();
}


fn closest_point_index(instructions: &Vec<(u32, u32)>, coords: (u32, u32)) -> i32 {
    let mut all_distances = instructions
        .iter()
        .enumerate()
        .map(|(index, value)| (index, man_dist(&value, coords)))
        .collect::<Vec<(usize, u32)>>();

    all_distances.sort_by_key(|a| a.1);
    if all_distances[0] == all_distances[1] {
        return -1;
    } else {
        return all_distances[0].0 as u32 as i32;
    }
}

fn is_edge(coords: &(u32, u32)) -> bool {
    return coords.0 == MIN_SIZE_MAP
        || coords.0 == MAX_SIZE_MAP
        || coords.1 == MIN_SIZE_MAP
        || coords.1 == MAX_SIZE_MAP;
}
fn man_dist(current_point: &(u32, u32), coords: (u32, u32)) -> u32 {
    return current_point.0.abs_diff(coords.0) + current_point.1.abs_diff(coords.1);
}

fn parse_input(input: Vec<String>) -> Vec<(u32, u32)> {
    return input
        .iter()
        .map(|line| {
            let args = line
                .split(", ")
                .into_iter()
                .flat_map(|num| num.parse())
                .collect::<Vec<u32>>();
            return (args[0], args[1]);
        })
        .collect::<Vec<(u32, u32)>>();
}
