fn main() {
    let input = include_str!("./input01.txt");
    let answer = part01(input);
    println!("The answer is: {}", answer);
}

#[derive(Debug)]
struct NumberPair {
    first_found: bool,
    left: Option<char>,
    right: Option<char>,
}

impl NumberPair {
    fn join(&self) -> i32 {
        let left_string = String::from(self.left.unwrap());
        let right_string = String::from(self.right.unwrap());
        let joined = left_string + &right_string;
        joined.parse().unwrap_or_else(|_e| 0)
    }

    fn init() -> NumberPair {
        NumberPair {
            first_found: false,
            left: None,
            right: None,
        }
    }
}

fn part01(input: &str) -> i32 {
    let mut answer = 0;
    for line in input.lines() {
        let mut row = NumberPair::init();
        for c in line.chars() {
            if c.is_numeric() {
                if row.first_found == false {
                    row.left = Some(c);
                    row.right = Some(c);
                    row.first_found = true;
                } else {
                    row.right = Some(c);
                }
            }
        }
        answer += row.join();
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input01_test.txt");
        let result = part01(input);
        assert_eq!(result, 142);
    }
}
