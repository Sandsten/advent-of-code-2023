fn main() {
    let input = include_str!("./input01.txt");
    let answer = part01(input);
    println!("The answer is: {}", answer);
}

#[derive(Debug, Clone)]
struct ScratchCard {
    card: String,
    nr_of_copies: u32,
    index: usize,
}

fn part01(input: &str) -> u32 {
    let mut total_number_of_scratchcards = 0;
    let mut scratchcards: Vec<ScratchCard> = Vec::new();

    for (index, card) in input.lines().enumerate() {
        scratchcards.push(ScratchCard {
            card: String::from(card),
            nr_of_copies: 1,
            index,
        })
    }

    for i in 0..scratchcards.len() {
        // Create a clone of one item in the vec, to avoid mutable reference to scratchcards
        // Since that prevents us from modifying it at line 39. There's probably a better way.
        let scratchcard = scratchcards[i].clone();
        total_number_of_scratchcards += scratchcard.nr_of_copies;
        
        let nr_of_matching_numbers = get_nr_of_matching_numbers(&scratchcard.card);

        let cards_to_create_copies_of =
            scratchcard.index + 1..scratchcard.index + &nr_of_matching_numbers + 1;

        for j in cards_to_create_copies_of {
            // Add as many copies as there are copies of the current card
            // One copy will be added per card!
            scratchcards[j].nr_of_copies += scratchcard.nr_of_copies;
        }
    }
    total_number_of_scratchcards
}

fn get_nr_of_matching_numbers(scratchcard: &str) -> usize {
    let split1: Vec<&str> = scratchcard.split(":").collect();
    let game_board = split1[1];

    let game_board: Vec<&str> = game_board
        .split("|")
        .map(|section| section.trim())
        .collect();

    let mut nr_of_matching_numbers: usize = 0;

    let winning_numbers: Vec<&str> = game_board[0].split_whitespace().collect();

    for my_number in game_board[1].split_whitespace() {
        if winning_numbers.contains(&my_number.trim()) {
            nr_of_matching_numbers += 1;
        }
    }
    nr_of_matching_numbers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input01_test.txt");
        let result = part01(input);
        assert_eq!(result, 30);
    }
}
