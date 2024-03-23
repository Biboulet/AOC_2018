use std::collections::HashMap;

pub fn compute_results(input: Vec<String>) -> (u32, u32) {
    let instructions: Vec<(u32, u32, u32, u32, u32)> = parse_input(input);
    return (solve(&instructions), solve_part2(instructions));
}

fn solve_part2(instructions: Vec<(u32, u32, u32, u32, u32)>) -> u32 {
    let map = get_filled_map(&instructions);

    return instructions.iter().find_map(|current_instr| {
        if !claim_overlaps(&map, current_instr) {
            return Some(current_instr.0);
        }
        else {
            return None;
        }
    }).expect("Pas de resultat trouvé");
}

fn claim_overlaps(map: &HashMap<(u32, u32), u32>, current_instr: &(u32, u32, u32, u32, u32)) -> bool {
    return (0..current_instr.3).any(|x| (0..current_instr.4).any(|y|map[&(current_instr.1 + x, current_instr.2+y)] > 1 ));
}

fn solve(instructions: &Vec<(u32, u32, u32, u32, u32)>) -> u32 {
    let map = get_filled_map(&instructions);

    return map.iter().fold(0, |square_overlaped_count, curr_key| {
        square_overlaped_count + ((curr_key.1 >= &2) as u32)
    });
}

fn get_filled_map(instructions: &Vec<(u32, u32, u32, u32, u32)>) -> HashMap<(u32, u32), u32> {
    let map: HashMap<(u32, u32), u32> = instructions.iter().fold(
        HashMap::new(),
        |mut map: HashMap<(u32, u32), u32>, current_instr| {
            for x in 0..current_instr.3 {
                for y in 0..current_instr.4 {
                    let key = (x + current_instr.1, current_instr.2 + y);
                    if map.contains_key(&key) {
                        map.insert(key, map[&key] + 1);
                    } else {
                        map.insert(key, 1);
                    }
                }
            }
            return map;
        },
    );
    map
}

// (id, x, y, length, height)
fn parse_input(input: Vec<String>) -> Vec<(u32, u32, u32, u32, u32)> {
    return input
        .iter()
        .map(|current_line| {
            let args: Vec<u32> = current_line[1..]
                .split(['@', ',', ':', 'x'].as_ref())
                .flat_map(|a| a.replace(" ", "").parse())
                .collect::<Vec<u32>>();

            return (args[0], args[1], args[2], args[3], args[4]);
        })
        .collect::<Vec<(u32, u32, u32, u32, u32)>>();
}
