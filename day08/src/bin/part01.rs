use std::collections::HashMap;

fn main() {
    let input = include_str!("./input01.txt");
    let answer = part01(input);
    println!("The answer is: {}", answer);
}

fn part01(input: &str) -> u32 {
    let (instructions, nodes) = parse_input(&input);

    let mut current_key = "AAA";
    let mut nr_of_steps = 0;
    loop {
        for instruction in instructions.iter() {
            nr_of_steps += 1;
            if instruction == &'L' {
                current_key = nodes.get(current_key).expect("Node not found").0;
            } else if instruction == &'R' {
                current_key = nodes.get(current_key).expect("Node not found").1;
            }

            if current_key == "ZZZ" {
                return nr_of_steps;
            }
        }
    }
}

fn parse_input(input: &str) -> (Vec<char>, HashMap<&str, (&str, &str)>) {
    let mut line = input.lines();

    let instructions: Vec<char> = line
        .next()
        .expect("No instructions found")
        .chars()
        .collect();

    line.next();

    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();

    for (i, node) in line.enumerate() {
        let key = &node[0..3];
        let left = &node[7..10];
        let right = &node[12..15];
        nodes.insert(key, (left, right));
    }

    (instructions, nodes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input01_test.txt");
        let result = part01(input);
        assert_eq!(result, 6);
    }
}
