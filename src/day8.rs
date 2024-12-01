use std::collections::HashMap;

pub fn part1(contents: &str) {
    let mut lines = contents.lines();

    let instructions = lines.next().unwrap().chars().collect::<Vec<char>>();

    lines.next();

    let node_network = lines
        .map(|line| {
            let (node, connected_nodes_str) = line.split_once(" = ").unwrap();

            let connected_nodes = connected_nodes_str[1..connected_nodes_str.len() - 1]
                .split_once(", ")
                .unwrap();

            (node, connected_nodes)
        })
        .collect::<HashMap<&str, (&str, &str)>>();

    let mut current_node = "AAA";
    let mut current_instruction_index = 0;
    let mut steps = 0;

    while current_node != "ZZZ" {
        let (left_node, right_node) = node_network.get(current_node).unwrap();

        current_node = match instructions[current_instruction_index] {
            'L' => left_node,
            'R' => right_node,
            _ => panic!("Invalid instruction"),
        };
        current_instruction_index = (current_instruction_index + 1) % instructions.len();
        steps += 1;
    }

    println!("[Part 1] Steps: {}", steps);
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(nums: &Vec<u64>) -> u64 {
    let mut nums = nums.clone();
    nums.sort();
    let mut lcm = nums[0];
    for i in 1..nums.len() {
        lcm = (nums[i] * lcm) / gcd(nums[i], lcm);
    }
    lcm
}

pub fn part2(contents: &str) {
    let mut lines = contents.lines();

    let instructions = lines.next().unwrap().chars().collect::<Vec<char>>();

    lines.next();

    let mut starting_nodes: Vec<&str> = Vec::new();
    let mut node_network: HashMap<&str, (&str, &str)> = HashMap::new();

    lines.for_each(|line| {
        let (node, connected_nodes_str) = line.split_once(" = ").unwrap();

        let connected_nodes = connected_nodes_str[1..connected_nodes_str.len() - 1]
            .split_once(", ")
            .unwrap();

        if node.ends_with("A") {
            starting_nodes.push(node);
        }
        node_network.insert(node, connected_nodes);
    });

    let mut steps_arr: Vec<u64> = Vec::new();

    starting_nodes.iter().for_each(|node| {
        let mut steps = 0;

        let mut current_instruction_index = 0;
        let mut current_node = *node;

        while !current_node.ends_with("Z") {
            let (left_node, right_node) = node_network.get(current_node).unwrap();

            current_node = match instructions[current_instruction_index] {
                'L' => left_node,
                'R' => right_node,
                _ => panic!("Invalid instruction"),
            };
            current_instruction_index = (current_instruction_index + 1) % instructions.len();
            steps += 1;
        }

        steps_arr.push(steps);
    });

    // lcm of steps is 55443343999,
    // 55443343999 * 283 = 15690466351717 <- answer

    println!("[Part 2] Steps: {}", lcm(&steps_arr));
}
