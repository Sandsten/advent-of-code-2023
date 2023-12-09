fn main() {
    let input = include_str!("./input01.txt");
    let answer = part01(input);
    println!("The answer is: {}", answer);
}

fn get_game_number(input: &str) -> u32 {
    let mut split = input.split(":");
    let game = split.next().unwrap();
    let game_number = game.split(" ").last().unwrap();
    game_number.parse::<u32>().unwrap()
}

fn part01(input: &str) -> u32 {
    let mut answer = 0;
    const MAX_RED_CUBES: u32 = 12;
    const MAX_GREEN_CUBES: u32 = 13;
    const MAX_BLUE_CUBES: u32 = 14;

    for line in input.lines() {
        let game_number = get_game_number(line);
        
        let mut split = line.split(":");
        split.next();
        let rounds = split.last().unwrap();
        let mut valid_round = true;

        let splits: Vec<&str> = rounds.split(';').collect();
        
        for split in splits {
            let mut nr_red_cubes = 0;
            let mut nr_green_cubes = 0;
            let mut nr_blue_cubes = 0;
            
            let moves: Vec<&str> = split.split(',').collect();
    
            for mut my_move in moves {
                my_move = my_move.trim();
                let (mut number, mut color) = my_move.split_at(my_move.find(" ").unwrap());
                number = number.trim();
                let number = number.parse::<u32>().unwrap();
                color = color.trim();
    
                match color {
                    "red" => nr_red_cubes += number,
                    "green" => nr_green_cubes += number,
                    "blue" => nr_blue_cubes += number,
                    _ => ()
                }
            }

            if nr_red_cubes > MAX_RED_CUBES || nr_green_cubes > MAX_GREEN_CUBES || nr_blue_cubes > MAX_BLUE_CUBES {
                valid_round = false;
            } 
        }

        if valid_round {
            answer += game_number;
        }        
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
        assert_eq!(result, 8);
    }
}