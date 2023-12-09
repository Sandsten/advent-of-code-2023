fn main() {
    let input = include_str!("./input01.txt");
    let answer = part01(input);
    println!("The answer is: {}", answer);
}

fn part01(input: &str) -> u32 {
    let mut answer = 0;

    for line in input.lines() {
        let mut split = line.split(":");
        split.next();
        let rounds = split.last().unwrap();

        let mut largest_nr_red_cubes = 0;
        let mut largets_nr_green_cubes = 0;
        let mut largest_nr_blue_cubes = 0;
        
        let splits: Vec<&str> = rounds.split(|c| c == ';').collect();
        for split in splits {
            let moves: Vec<&str> = split.split(|c| c == ',').collect();
            for mut my_move in moves {
                my_move = my_move.trim();
                let (mut number, mut color) = my_move.split_at(my_move.find(" ").unwrap());
                number = number.trim();
                let number = number.parse::<u32>().unwrap();
                color = color.trim();
    
                match color {
                    "red" => {
                        if number > largest_nr_red_cubes {
                            largest_nr_red_cubes = number;
                        }
                    },
                    "green" => {
                        if number > largets_nr_green_cubes {
                            largets_nr_green_cubes = number;
                        }
                    },
                    "blue" => {
                        if number > largest_nr_blue_cubes {
                            largest_nr_blue_cubes = number;
                        }
                    } 
                    _ => ()
                }
            }
        }
        answer += largest_nr_red_cubes * largets_nr_green_cubes * largest_nr_blue_cubes;
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
        assert_eq!(result, 2286);
    }
}