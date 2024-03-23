use std::{
    cmp::min, collections::HashMap
};

advent_of_code::solution!(11);
type Coord = (i32, i32);

pub fn part_one(input: &str) -> Option<String> {
    let map = generate_map(input.parse().unwrap(), 300);
    let best_coord = find_best_square(map, false, 300);
    Some(format!("{},{},{}", best_coord.0, best_coord.1, best_coord.2))
}
pub fn part_two(input: &str) -> Option<String> {
    let map = generate_map(input.parse().unwrap(), 300);
    let best_coord = find_best_square(map, true, 300);
    Some(format!("{},{},{}", best_coord.0, best_coord.1, best_coord.2))
}

fn generate_map(serial_number: u32, square_size: i32) -> HashMap<Coord, i32> {
    let mut map = HashMap::new();
    for x in 1..=square_size {
        for y in 1..=square_size {
            map.insert((x, y), get_power_level(x, y, serial_number));
        }
    }

    map.clone()
}

fn get_power_level(x: i32, y: i32, serial_number: u32) -> i32 {
    let rack_id = x + 10;
    let mut power_level = rack_id * y;
    power_level += serial_number as i32;
    power_level *= rack_id;
    power_level = power_level.div_euclid(100) % 10;
    power_level - 5
}

fn find_best_square(map: HashMap<Coord, i32>, is_part_2: bool, square_size: u32) -> (u32, u32, u32) {
    let mut best_position = (1, 1, 0);
    let mut best_score = -100;

    for x in 1..=square_size {
        // dbg!(best_position);
        for y in 1..=square_size {
            let mut curr_score = 0;
            let max_size = {
                if !is_part_2 {
                    2
                }
                else {
                    min(square_size - x, square_size - y)
                }
            };
            
            //add row/collomn by row/collomn
            for size in 0..=max_size {
                let new_row: i32 = (x..=x +size).map(|sub_x| map.get(&(sub_x as i32, (y+size) as i32)).unwrap_or(&0)).sum();
                let new_colomn: i32 = (y..=y +size).map(|sub_y| map.get(&((x+size) as i32, sub_y as i32)).unwrap_or(&0)).sum();
                curr_score += new_row + new_colomn - map.get(&((x+size) as i32, (y + size) as i32)).unwrap_or(&0);
                if curr_score > best_score {
                    best_position = (x, y, size+1);
                    best_score = curr_score;
                }
            }
        }
    }
    best_position
}
