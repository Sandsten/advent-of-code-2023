use std::collections::HashMap;
use num::integer::lcm;

fn main() {
    let input = include_str!("./input01.txt");
    let answer = part02(input);
    println!("The answer is: {}", answer);
}

fn part02(input: &str) -> u128 {
    let cycle_lengths = get_cycle_lengths(&input);

    // Once you land on a node ending with Z, it will take the same amount of steps to get back to a node ending with Z.
    // This means that LCM can be used to find the number of steps required so that all cycles line up on a node ending with Z.
    // A bit far fetched...
    let cycle_length = cycle_lengths.into_iter().reduce(|a,b| lcm(a, b));

    return cycle_length.expect("No LCM found");
}

fn get_cycle_lengths(input: &str) -> Vec<u128> {
    let (starting_keys, instructions, nodes) = parse_input(&input);

    let mut current_keys = starting_keys.clone();
    let mut nr_of_steps = 0;
    let mut cycle_lengths: Vec<u128> = Vec::new();

    while current_keys.len() > 0 {
        for instruction in instructions.iter() {
            nr_of_steps += 1;
            for i in 0..current_keys.len() {
                if instruction == &'L' {
                    current_keys[i] = nodes.get(current_keys[i]).expect("Node not found").0;
                } else {
                    current_keys[i] = nodes.get(current_keys[i]).expect("Node not found").1;
                }
            }

            let mut keys_to_remove: Vec<usize> = Vec::new();
            for i in 0..current_keys.len() {
                if current_keys[i].ends_with('Z') {
                    keys_to_remove.push(i);
                    cycle_lengths.push(nr_of_steps);
                }
            }

            for i in keys_to_remove.iter() {
                current_keys.remove(*i);
            }
        }
    }
    cycle_lengths
}

fn parse_input(input: &str) -> (Vec<&str>, Vec<char>, HashMap<&str, (&str, &str)>) {
    let mut line = input.lines();

    let instructions: Vec<char> = line
        .next()
        .expect("No instructions found")
        .chars()
        .collect();

    line.next();

    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut starting_keys: Vec<&str> = Vec::new();

    for (i, node) in line.enumerate() {
        let key = &node[0..3];
        let left = &node[7..10];
        let right = &node[12..15];
        nodes.insert(key, (left, right));

        if key.ends_with('A') {
            starting_keys.push(key);
        }
    }

    (starting_keys, instructions, nodes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input02_test.txt");
        let result = part02(input);
        assert_eq!(result, 6);
    }
}
