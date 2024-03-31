advent_of_code::solution!(13);

use int_enum::IntEnum;
use num::Complex;
use std::collections::HashMap;
use std::collections::VecDeque;

type Coord = Complex<i32>;
#[repr(u8)]
#[derive(Clone, PartialEq, Eq, IntEnum, Debug, Copy)]
enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}
#[derive(Clone, PartialEq, Eq)]
enum Tile {
    Vertical,
    Horizontal,
    Slash,
    BackSlash,
    Intersection,
}
#[derive(Clone, PartialEq, Eq, Debug)]
struct Cart {
    position: Coord,
    direction: Direction,
    state: u32,
}

pub fn part_one(input: &str) -> Option<String> {
    let (map, carts) = parse_input(input);
    let coord_of_first_crash = simulate_until_first_crash(map, carts);
    Some(format!(
        "{},{}",
        coord_of_first_crash.re.to_string(),
        coord_of_first_crash.im.to_string()
    ))
}
pub fn part_two(input: &str) -> Option<String> {
    let (map, carts) = parse_input(input);
    let coord_of_last_cart = simulate_until_last_crash(map, carts);
    Some(format!(
        "{},{}",
        coord_of_last_cart.re.to_string(),
        coord_of_last_cart.im.to_string()
    ))
}

fn simulate_until_last_crash(map: HashMap<Complex<i32>, Tile>, carts: Vec<Cart>) -> Coord {
    let mut queue: VecDeque<Cart> = carts.iter().fold(VecDeque::new(), |mut queue, ele| {
        queue.push_back(ele.clone());
        return queue;
    });
    while queue.len() > 1 {
        let curr_cart = queue.pop_front().unwrap();
        let next_cart_state = get_next_cart_state(&curr_cart, &map);
        if let Some((i,_)) = queue.iter().enumerate().find(|(_,sub_cart)| sub_cart.position == next_cart_state.position) {
            queue.remove(i);
        }
        else {
            queue.push_back(next_cart_state);
        }
    }
    let last_cart = queue.pop_front().unwrap();
    dbg!(get_next_cart_state(&last_cart, &map).position);
    return last_cart.position;
}

fn simulate_until_first_crash(map: HashMap<Complex<i32>, Tile>, carts: Vec<Cart>) -> Coord {
    let mut queue: VecDeque<Cart> = carts.iter().fold(VecDeque::new(), |mut queue, ele| {
        queue.push_back(ele.clone());
        return queue;
    });
    while let Some(curr_cart) = queue.pop_front() {
        let next_cart_state = get_next_cart_state(&curr_cart, &map);
        if queue.iter().any(|sub_cart| sub_cart.position == next_cart_state.position) {
            return next_cart_state.position;
        }
        queue.push_back(next_cart_state);
    }
    return Complex::new(0, 0);
}

fn get_next_cart_state(curr_cart: &Cart, map: &HashMap<Complex<i32>, Tile>) -> Cart {
    let new_direction = match map.get(&curr_cart.position).unwrap() {
        Tile::Vertical => curr_cart.direction,
        Tile::Horizontal => curr_cart.direction,
        // " / "
        Tile::Slash => match curr_cart.direction {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        },
        // " \ "
        Tile::BackSlash => match curr_cart.direction {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        },
        Tile::Intersection => Direction::try_from(
            (curr_cart.direction as i32 - 1 + curr_cart.state as i32).rem_euclid(4) as u8,
        )
        .unwrap(),
    };
    let new_state = {
        if *map.get(&curr_cart.position).unwrap() == Tile::Intersection {
            (curr_cart.state + 1).rem_euclid(3)
        } else {
            curr_cart.state
        }
    };
    let new_pos = curr_cart.position.clone() + get_next_position(&new_direction);
    Cart {
        position: new_pos,
        direction: new_direction,
        state: new_state,
    }
}

fn get_next_position(direction: &Direction) -> Coord {
    return match direction {
        Direction::Up => Complex::<i32>::new(0, -1),
        Direction::Down => Complex::<i32>::new(0, 1),
        Direction::Left => Complex::<i32>::new(-1, 0),
        Direction::Right => Complex::<i32>::new(1, 0),
    };
}

fn parse_input(input: &str) -> (HashMap<Coord, Tile>, Vec<Cart>) {
    let mut map = HashMap::new();
    let mut carts = Vec::new();

    for (y, line) in input.split("\r\n").enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == ' ' {
                continue;
            }
            let current_coord = Complex::new(x as i32, y as i32);
            map.insert(current_coord.clone(), get_tile_type(char));

            if let Some(i) = get_cart(char, current_coord) {
                carts.push(i);
            }
        }
    }

    return (map, carts);
}

fn get_cart(char: char, current_coord: Coord) -> Option<Cart> {
    if char == '<' {
        return Some(Cart {
            position: current_coord,
            direction: Direction::Left,
            state: 0,
        });
    } else if char == '>' {
        return Some(Cart {
            position: current_coord,
            direction: Direction::Right,
            state: 0,
        });
    } else if char == 'v' {
        return Some(Cart {
            position: current_coord,
            direction: Direction::Down,
            state: 0,
        });
    } else if char == '^' {
        return Some(Cart {
            position: current_coord,
            direction: Direction::Up,
            state: 0,
        });
    }
    None
}

fn get_tile_type(char: char) -> Tile {
    return match char {
        '-' => Tile::Horizontal,
        '>' => Tile::Horizontal,
        '<' => Tile::Horizontal,
        '|' => Tile::Vertical,
        '^' => Tile::Vertical,
        'v' => Tile::Vertical,
        '/' => Tile::Slash,
        '\\' => Tile::BackSlash,
        '+' => Tile::Intersection,
        a => panic!("unexpected char : {}", a as u32),
    };
}


