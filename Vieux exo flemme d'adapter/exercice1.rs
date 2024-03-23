use std::collections::HashSet;

pub fn solve_part2(instructions: &Vec<i32>) -> i32 {
    let mut frequence: i32 = 0;
    let mut previous_frequences: HashSet<i32> = HashSet::new();
    return instructions.iter().cycle().find_map(|curr_instr| {
        frequence += curr_instr;
        if previous_frequences.contains(&frequence) {
            return Some(frequence);
        } else {
            previous_frequences.insert(frequence);
            return None;
                
        }                                   

                                                    
    }).expect("jsp");
    // loop {
    //     let current_instr: i32 = instructions[(i as usize) % &instructions.len()];
    //     frequence += current_instr;
    //     if previous_frequences.contains(&frequence) {
    //         return frequence;
    //     } else {
    //         previous_frequences.insert(frequence);
    //     }
    //     i+=1;
    // }
}
pub fn solve(instructions: &Vec<i32>) -> i32 {
    // let mut frequence: i32 = 0;
    // for curr_inst in instructions {
    //     frequence += curr_inst;
    // }
    return instructions.iter().fold(0, |a, e| a + e);
}

pub fn parse_input(instructions: Vec<String>) -> Vec<i32> {
    return instructions.iter().flat_map(|a| a.parse()).collect();
}

pub fn compute_results(input: Vec<String>) -> (i32, i32) {
    let instructions = parse_input(input);
    return (solve(&instructions), solve_part2(&instructions));
}
