pub fn compute_results(input: Vec<String>) -> (u32, u32) {
    let instructions: Vec<(u32, u32, u32, u32, u32)> = parse_input(input);
    return (solve(instructions),  0);
}


fn solve(mut instructions: Vec<(u32, u32, u32, u32, u32)>) -> u32 {
    instructions.sort_by_key(|current_instruction| get_date_as_key(&current_instruction));
    dbg!(instructions);
    return 0;
}

fn get_date_as_key(instr: &(u32, u32, u32, u32, u32)) -> u32 {
    let key = instr.3 + 100*instr.2 + 10000*instr.1 + 1000000*instr.0;
    dbg!(key);
    return key;
}

fn parse_input(input: Vec<String>) -> Vec<(u32, u32, u32, u32, u32)> {
    return input
        .iter()
        .map(|current_line| {
            return (
                current_line[1..5].parse().unwrap(),
                current_line[6..8].parse().unwrap(),
                current_line[9..11].parse().unwrap(),
                get_clamped_time(&current_line[12..17]),
                get_action(&current_line[19..]),
            );
        })
        .collect::<Vec<(u32, u32, u32, u32, u32)>>();
}

fn get_clamped_time(time_raw: &str) -> u32 {
    if time_raw[..2].parse::<u32>().unwrap() == 23{
        return 0;
    }
    else {
        return time_raw[3..].parse().unwrap();
    }
}

fn get_action(action: &str) -> u32 {
    match &action[0..1] {
        "w" => return 0,
        "f" => return 1,
        "G" => return action[7..11].replace(" ", "").parse().unwrap(),
        _ => panic!("action not found")
    }
}
//trop chiant a parse ( bouger les mois/ années en fction de l'heure ext)