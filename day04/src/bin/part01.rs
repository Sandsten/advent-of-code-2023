fn main() {
    let input = include_str!("./input01.txt");
    let answer = part01(input);
    println!("The answer is: {}", answer);
}

fn part01(input: &str) -> u32 {
    let mut total_score = 0;
    for scratchcard in input.lines() {
        total_score += get_scratchcard_score(&scratchcard);
    }
    total_score
}

fn get_scratchcard_score(scratchcard: &str) -> u32 {
    let split1: Vec<&str> = scratchcard.split(":").collect();
    let game_board = split1[1];

    let game_board: Vec<&str> = game_board
        .split("|")
        .map(|section| section.trim())
        .collect();

    let mut score: u32 = 0;

    let winning_numbers: Vec<&str> = game_board[0].split_whitespace().collect();

    for my_number in game_board[1].split_whitespace() {
        if winning_numbers.contains(&my_number.trim()) {
            if score == 0 {
                score += 1;
            } else {
                score *= 2;
            }
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input01_test.txt");
        let result = part01(input);
        assert_eq!(result, 13);
    }
}