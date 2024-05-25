use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    path,
};

use num::Complex;

advent_of_code::solution!(15);

type Coord = num::Complex<i32>;
#[derive(Clone, Copy, Debug)]
struct Unit {
    unit_type_is_elf: bool,
    hit_point: i32,
    attack_power: u32,
    position: Coord,
}
impl Unit {
    fn is_dead(&self) -> bool {
        return self.hit_point <= 0;
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (units, map) = parse_input(input);
    Some(combat_outcome(units, &map))
}

fn combat_outcome(inital_units: Vec<Unit>, map: &HashSet<Complex<i32>>) -> u32 {
    let mut combat_turn = 0;
    let mut units = inital_units.clone();
    // les unités restent à la meme pos dans la liste et chaque unité est modifié

    while !combat_is_over(&units) {
        let mut playing_order = get_combat_order(&units);
        for curr_unit_index in playing_order {
            if units[curr_unit_index].is_dead() {
                continue;
            }
            let curr_unit = units[curr_unit_index];
            let available_squares = get_all_adjacent_square(&units, map);
            let distances_to_all_available_squares = get_all_distances(
                units.iter().map(|a| a.position).collect(),
                map,
                curr_unit.position,
            );
            let mut best_path: Vec<Coord> = get_best_path(distances_to_all_available_squares);
        }
        combat_turn += 1;
    }
    dbg!(combat_turn);
    return combat_turn
        * units
            .iter()
            .filter(|a| !a.is_dead())
            .map(|a| a.hit_point as u32)
            .sum::<u32>();
}

fn get_best_path(distances_to_all_available_squares: Vec<Vec<Coord>>) -> Vec<Coord>{
    let mut closest_squares = distances_to_all_available_squares.iter().fold(
        Vec::new(),
        |mut lowest_length_paths: Vec<&Vec<Coord>>, curr_path: &Vec<Coord>| {
            if lowest_length_paths[0].len() == curr_path.len() {
                lowest_length_paths.push(curr_path);
            } else if lowest_length_paths[0].len() > curr_path.len() {
                return vec![curr_path];
            }
            return lowest_length_paths;
        },
    );
    closest_squares.sort_by(|path_a: &&Vec<Coord>, path_b: &&Vec<Coord>| {
        cmp_reading_order(path_a.last().unwrap(), path_b.last().unwrap())
    });
    return closest_squares.clone().first().clone().unwrap();
}

//Returns a list of all paths to the given pos
fn get_all_distances(
    destinations: Vec<Coord>,
    map: &HashSet<Coord>,
    starting_pos: Coord,
) -> Vec<Vec<Coord>> {
    return destinations
        .iter()
        .map(|curr_dest| get_dijkstra_shortest_path(&starting_pos, curr_dest, map))
        .collect();
}

fn get_dijkstra_shortest_path(
    starting_pos: &Coord,
    curr_dest: &Coord,
    map: &HashSet<Coord>,
) -> Vec<Coord> {
    todo!()
}

fn get_all_adjacent_square(units: &Vec<Unit>, map: &HashSet<Coord>) -> Vec<Coord> {
    return units
        .iter()
        .filter(|a| !a.is_dead())
        .map(|a| get_adjacent_position(a.position))
        .collect::<Vec<Vec<Coord>>>()
        .concat()
        .iter()
        .filter(|coord| !map.contains(&coord) && !units.iter().any(|a| a.position == **coord))
        .map(|a| *a)
        .collect::<Vec<Coord>>();
}
fn get_adjacent_position(curr_coord: Coord) -> Vec<Coord> {
    return vec![
        curr_coord + Complex::<i32>::new(0, -1),
        curr_coord + Complex::<i32>::new(0, 1),
        curr_coord + Complex::<i32>::new(-1, 0),
        curr_coord + Complex::<i32>::new(1, 0),
    ];
}

fn get_combat_order(units: &Vec<Unit>) -> Vec<usize> {
    let mut unit_with_index = units.iter().enumerate().collect::<Vec<(usize, &Unit)>>();
    unit_with_index.sort_by(|(_, a), (_, b)| cmp_reading_order(&a.position, &b.position));
    return unit_with_index.iter().map(|(i, _)| *i).collect();
}

fn combat_is_over(curr_units: &Vec<Unit>) -> bool {
    return curr_units
        .iter()
        .filter(|a| a.unit_type_is_elf)
        .all(|a| a.is_dead())
        || curr_units
            .iter()
            .filter(|a| !a.unit_type_is_elf)
            .all(|a| a.is_dead());
}

fn cmp_reading_order(position_1: &Complex<i32>, position_2: &Complex<i32>) -> std::cmp::Ordering {
    if position_1.im != position_2.im {
        return position_1.im.cmp(&position_2.im);
    } else {
        return position_1.re.cmp(&position_2.re);
    }
}

fn parse_input(input: &str) -> (Vec<Unit>, HashSet<Coord>) {
    let mut units = Vec::new();
    let mut map = HashSet::new();

    for (y, line) in input.split("\r\n").enumerate() {
        for (x, char) in line.chars().enumerate() {
            let curr_pos = Complex::new(x as i32, y as i32);
            if char == '#' {
                map.insert(curr_pos.clone());
            } else if char != '.' {
                units.push(Unit {
                    unit_type_is_elf: char == 'E',
                    hit_point: 200,
                    attack_power: 3,
                    position: curr_pos,
                });
            }
        }
    }
    return (units, map.clone());
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
