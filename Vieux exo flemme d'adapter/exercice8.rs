use itertools::Itertools;

struct Node {
    sub_nodes: Vec<Node>,
    meta_data: Vec<u32>,
}
impl Node {
    fn get_sum_of_meta_values(&self) -> u32 {
        return self.meta_data.iter().sum::<u32>()
            + self
                .sub_nodes
                .iter()
                .map(|a| a.get_sum_of_meta_values())
                .sum::<u32>();
    }

    fn get_value(&self) -> u32 {
        if self.sub_nodes.len() == 0 {
            return self.meta_data.iter().sum::<u32>();
        } else {
            return self
                .meta_data
                .iter()
                .flat_map(|meta_data| self.sub_nodes.get((*meta_data-1) as usize))
                .map(|child_node| child_node.get_value())
                .sum::<u32>();
        }
    }
}
impl Default for Node {
    fn default() -> Self {
        Self {
            sub_nodes: vec![],
            meta_data: vec![],
        }
    }
}

pub fn compute_results(input: Vec<String>) -> (u32, u32) {
    let instructions = parse_input(input);
    let root_node = parse_nodes(&instructions, 0).0;
    return (solve(&root_node), solve2(&root_node));
}

fn solve2(root_node: &Node) -> u32 {
    return root_node.get_value();
}

fn solve(root_node: &Node) -> u32 {
    return root_node.get_sum_of_meta_values();
}

fn parse_nodes(instructions: &Vec<u32>, starting_pointer: usize) -> (Node, usize) {
    let mut pointer: usize = starting_pointer;
    let mut current_node: Node = Node::default();
    let child_node_quantity = instructions[pointer];
    let meta_data_quantity = instructions[pointer + 1];
    pointer += 2;

    for _i in 0..child_node_quantity {
        let args = parse_nodes(instructions, pointer);
        current_node.sub_nodes.push(args.0);
        pointer = args.1;
    }
    current_node.meta_data = instructions[pointer..pointer + meta_data_quantity as usize]
        .iter()
        .map(|val| *val)
        .collect_vec();

    return (current_node, pointer + meta_data_quantity as usize);
}

fn parse_input(input: Vec<String>) -> Vec<u32> {
    return input[0]
        .split(" ")
        .into_iter()
        .flat_map(|a| a.parse::<u32>())
        .collect_vec();
}
