fn main() {
    let input = include_str!("./input01.txt");
    let answer = part02(input);
    println!("The answer is: {}", answer);
}

#[derive(Debug)]
struct PotentialNumber {
    value: String
}

impl PotentialNumber {
    fn new() -> PotentialNumber {
        PotentialNumber { value: String::new() }
    }
    
    fn return_char_of_number_if_number(&self) -> Option<char> {
        match self.value.as_str() {
            number if number.contains("one") => Some('1'),
            number if number.contains( "two" ) => Some('2'),
            number if number.contains( "three" ) => Some('3'),
            number if number.contains( "four" ) => Some('4'),
            number if number.contains( "five" ) => Some('5'),
            number if number.contains( "six" ) => Some('6'),
            number if number.contains( "seven" ) => Some('7'),
            number if number.contains( "eight" ) => Some('8'),
            number if number.contains( "nine" ) => Some('9'),
            _ => None
        }
    }
}

enum Direction {
    LEFT,
    RIGHT
}

fn find_first_number(input: &str, direction: Direction) -> Option<char> {
    let mut input = String::from(input);
    let mut potential_number = PotentialNumber::new();

    if let Direction::RIGHT = direction {
        input = input.chars().rev().collect::<String>();
    }

    for c in input.chars() {
        match c {
            c if c.is_digit(10) => {
                return Some(c);
            },
            c => {
                match direction {
                    Direction::LEFT => potential_number.value.push(c),
                    Direction::RIGHT => potential_number.value.insert(0, c)
                }
                let number: Option<char> = potential_number.return_char_of_number_if_number();
                match number {
                    Some( char ) => return Some(char),
                    _ => ()
                }
            }
        }
    }
    None
}

fn part02(input: &str) -> i32 {
    let mut answer = 0;
    for row in input.lines() {
        let left_most_number = find_first_number(row, Direction::LEFT).unwrap();
        let right_most_number = find_first_number(row, Direction::RIGHT).unwrap();
        
        let joined = String::from(left_most_number) + &String::from(right_most_number);

        answer += joined.parse::<i32>().unwrap();
    }
    answer    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input01_test_p2.txt");
        let result = part02(input);
        assert_eq!(result, 281);
    }
}