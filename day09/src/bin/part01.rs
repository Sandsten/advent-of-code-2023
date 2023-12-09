fn main() {
    let input = include_str!("./input01.txt");
    let answer = part01(input);
    print!("The answer is: {}", answer);
}

fn part01(input: &str) -> i128 {
    let parsed_data = parse_input(input);
    let mut answer = 0;

    for line in parsed_data {
        let mut differences: Vec<Vec<i128>> = Vec::new();
        differences.push(line.clone());

        let mut all_differences_are_zero = false;
        let mut current_line = line.clone();

        while !all_differences_are_zero {
            let mut difference: Vec<i128> = Vec::new();
            for i in 0..current_line.len() - 1 {
                let diff = current_line[i + 1] - current_line[i];
                difference.push(diff);
            }
            current_line = difference;

            differences.push(current_line.clone());
            all_differences_are_zero = current_line.iter().all(|&value| value == 0);
        }

        // Find the next number
        let nr_iter = differences.clone();
        let nr_iter = nr_iter.len();
        for i in (0..nr_iter - 1).rev() {
            let a = differences[i].last().expect("").clone();
            let b = differences[i + 1].last().expect("").clone();
            differences[i].push(a + &b);
        }

        answer += differences[0].last().expect("");
    }

    answer
}

fn parse_input(input: &str) -> Vec<Vec<i128>> {
    let inputs = input.lines();
    let mut parsed_data: Vec<Vec<i128>> = Vec::new();

    for line in inputs {
        let new_data: Vec<&str> = line.split_whitespace().collect();
        let new_data: Vec<i128> = new_data
            .into_iter()
            .map(|n| n.parse::<i128>().expect("Failed to parse"))
            .collect();
        parsed_data.push(new_data);
    }

    parsed_data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input01_test.txt");
        let result = part01(input);
        assert_eq!(result, 114);
    }
}
