use std::{
    collections::{HashMap, HashSet},
    slice::Iter,
    usize,
};

advent_of_code::solution!(16);
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Opcode {
    addr,
    addi,
    mulr,
    muli,
    banr,
    bani,
    borr,
    bori,
    setr,
    seti,
    gtir,
    gtri,
    gtrr,
    eqir,
    eqri,
    eqrr,
}
impl Opcode {
    pub fn iterator() -> Iter<'static, Opcode> {
        static OPCODES: [Opcode; 16] = [
            Opcode::addr,
            Opcode::addi,
            Opcode::mulr,
            Opcode::muli,
            Opcode::banr,
            Opcode::bani,
            Opcode::borr,
            Opcode::bori,
            Opcode::setr,
            Opcode::seti,
            Opcode::gtir,
            Opcode::gtri,
            Opcode::gtrr,
            Opcode::eqir,
            Opcode::eqri,
            Opcode::eqrr,
        ];
        OPCODES.iter()
    }
    pub fn from_usize(i: usize) -> Opcode {
        return **Opcode::iterator().collect::<Vec<&Opcode>>().get(i).unwrap();
    }
}
#[derive(Clone, Copy, Debug)]
struct Instruction {
    opcode_num: usize,
    input_a: usize,
    input_b: usize,
    output: usize,
}
impl Instruction {
    pub fn new(input: [usize; 4]) -> Self {
        return Instruction {
            opcode_num: input[0],
            input_a: input[1],
            input_b: input[2],
            output: input[3],
        };
    }

    pub fn run_instruction(&self, input_array: &[usize; 4], opcode: &Opcode) -> [usize; 4] {
        let mut new_array = input_array.clone();
        match opcode {
            Opcode::addr => {
                new_array[self.output] = new_array[self.input_a] + new_array[self.input_b];
            }
            Opcode::addi => {
                new_array[self.output] = new_array[self.input_a] + self.input_b;
            }
            Opcode::mulr => {
                new_array[self.output] = new_array[self.input_a] * new_array[self.input_b];
            }
            Opcode::muli => {
                new_array[self.output] = new_array[self.input_a] * self.input_b;
            }
            Opcode::banr => {
                new_array[self.output] = new_array[self.input_a] & new_array[self.input_b];
            }
            Opcode::bani => {
                new_array[self.output] = new_array[self.input_a] & self.input_b;
            }
            Opcode::borr => {
                new_array[self.output] = new_array[self.input_a] | new_array[self.input_b];
            }
            Opcode::bori => {
                new_array[self.output] = new_array[self.input_a] | self.input_b;
            }
            Opcode::setr => {
                new_array[self.output] = new_array[self.input_a];
            }
            Opcode::seti => {
                new_array[self.output] = self.input_a;
            }
            Opcode::gtir => {
                new_array[self.output] = (self.input_a > new_array[self.input_b]) as usize;
            }
            Opcode::gtri => {
                new_array[self.output] = (new_array[self.input_a] > self.input_b) as usize;
            }
            Opcode::gtrr => {
                new_array[self.output] =
                    (new_array[self.input_a] > new_array[self.input_b]) as usize;
            }
            Opcode::eqir => {
                new_array[self.output] = (self.input_a == new_array[self.input_b]) as usize;
            }
            Opcode::eqri => {
                new_array[self.output] = (new_array[self.input_a] == self.input_b) as usize;
            }
            Opcode::eqrr => {
                new_array[self.output] =
                    (new_array[self.input_a] == new_array[self.input_b]) as usize;
            }
        }
        return new_array;
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let (samples, instructions) = parse_input(input);
    Some(
        samples
            .iter()
            .map(|curr_sample| (get_all_valid_opcode(curr_sample).len() >= 3) as u32)
            .sum(),
    )
}
pub fn part_two(input: &str) -> Option<usize> {
    let (samples, instructions) = parse_input(input);
    let conversion_table: Vec<Opcode> = compute_conversion_table(samples);
    Some(compute_instructions(instructions, conversion_table)[0])
}

fn compute_instructions(
    instructions: Vec<[usize; 4]>,
    conversion_table: Vec<Opcode>,
) -> [usize; 4] {
    return instructions
        .iter()
        .fold([0; 4], |curr_registers: [usize; 4], curr_instr| {
            Instruction::new(*curr_instr)
                .run_instruction(&curr_registers, &conversion_table[curr_instr[0]])
        });
}

fn compute_conversion_table(samples: Vec<([usize; 4], Instruction, [usize; 4])>) -> Vec<Opcode> {
    let mut all_valid_matches_for_opcode: Vec<HashSet<Opcode>> = (0..16)
        .into_iter()
        .map(|curr_opcode: usize| Opcode::iterator().map(|a| *a).collect::<HashSet<Opcode>>())
        .collect();

    //1er guess depuis les samples
    for curr_sample in samples {
        let matching_opcode: HashSet<Opcode> = get_all_valid_opcode(&curr_sample)
            .iter()
            .map(|a| *a)
            .collect();
        all_valid_matches_for_opcode[curr_sample.1.opcode_num] = all_valid_matches_for_opcode
            [curr_sample.1.opcode_num]
            .intersection(&matching_opcode)
            .map(|a| *a)
            .collect();
    }
    //2ème en éliminant les opcodes déjà trouvés
    let mut opcode_found: HashSet<Opcode> = all_valid_matches_for_opcode
        .iter()
        .filter(|set| set.len() == 1)
        .map(|set| *set.iter().next().unwrap())
        .collect();

    while opcode_found.len() < 16 {
        all_valid_matches_for_opcode = all_valid_matches_for_opcode
            .iter()
            .map(|set| {
                if set.len() != 1 {
                    return set.difference(&opcode_found).map(|a| *a).collect();
                }
                return set.clone();
            })
            .collect::<Vec<HashSet<Opcode>>>();

        opcode_found = all_valid_matches_for_opcode
            .iter()
            .filter(|set| set.len() == 1)
            .map(|set| *set.iter().next().unwrap())
            .collect();
    }

    return all_valid_matches_for_opcode
        .iter()
        .map(|set| *set.iter().next().unwrap())
        .collect();
}

fn get_all_valid_opcode(curr_sample: &([usize; 4], Instruction, [usize; 4])) -> Vec<Opcode> {
    let (input, inst, output) = curr_sample;
    return Opcode::iterator().fold(Vec::<Opcode>::new(), |mut valid_opcode, curr_opcode| {
        if inst.run_instruction(input, curr_opcode) == *output {
            valid_opcode.push(*curr_opcode);
        }
        return valid_opcode;
    });
}

fn parse_input(input: &str) -> (Vec<([usize; 4], Instruction, [usize; 4])>, Vec<[usize; 4]>) {
    let mut parts = input.split("\r\n\r\n\r\n\r\n");
    let samples = parts
        .next()
        .unwrap()
        .split("\r\n\r\n")
        .map(parse_sample)
        .collect();

    let test_part = parts
        .next()
        .unwrap()
        .split("\r\n")
        .map(parse_line)
        .collect();

    return (samples, test_part);
}
fn parse_sample(input: &str) -> ([usize; 4], Instruction, [usize; 4]) {
    let mut args = input.split("\r\n");
    return (
        parse_line(args.next().unwrap()),
        Instruction::new(parse_line(args.next().unwrap())),
        parse_line(args.next().unwrap()),
    );
}

fn parse_line(line: &str) -> [usize; 4] {
    let args: Vec<usize> = line
        .trim_start_matches("Before: [")
        .trim_start_matches("After:  [")
        .trim_end_matches(']')
        .replace(",", "")
        .split(" ")
        .map(|ele| ele.parse().unwrap())
        .collect();

    return [args[0], args[1], args[2], args[3]];
}
