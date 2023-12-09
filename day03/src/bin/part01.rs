fn main() {
    let input = include_str!("./input01.txt");
    let answer = part01(input);
    println!("The answer is: {}", answer);
}

fn part01(input: &str) -> u32 {
    let (part_numbers, symbols) = parse_schematics(input);

    let mut included_part_numbers: Vec<u32> = Vec::new();
    // filter out all part numbers that aren't adjacent to a symbol
    for part_number in part_numbers.clone() {
        let mut is_already_included = false;

        for part_number_location in part_number.locations {
            if is_already_included {
                break;
            }

            let mut north = part_number_location.clone();
            north.y -= 1;

            let mut north_east = part_number_location.clone();
            north_east.y -= 1;
            north_east.x += 1;

            let mut east = part_number_location.clone();
            east.x += 1;

            let mut south_east = part_number_location.clone();
            south_east.x += 1;
            south_east.y += 1;

            let mut south = part_number_location.clone();
            south.y += 1;

            let mut south_west = part_number_location.clone();
            south_west.x -= 1;
            south_west.y += 1;

            let mut west = part_number_location.clone();
            west.x -= 1;

            let mut north_west = part_number_location.clone();
            north_west.x -= 1;
            north_west.y -= 1;

            // Do any adjacent location match the location of the symbol?
            for symbol in symbols.clone() {

                if north == symbol.location {
                    included_part_numbers.push(part_number.value);
                    is_already_included = true;
                    break;
                }

                if north_east == symbol.location {
                    included_part_numbers.push(part_number.value);
                    is_already_included = true;
                    break;
                }

                if east == symbol.location {
                    included_part_numbers.push(part_number.value);
                    is_already_included = true;
                    break;
                }

                if south_east == symbol.location {
                    included_part_numbers.push(part_number.value);
                    is_already_included = true;
                    break;
                }

                if south == symbol.location {
                    included_part_numbers.push(part_number.value);
                    is_already_included = true;
                    break;
                }

                if south_west == symbol.location {
                    included_part_numbers.push(part_number.value);
                    is_already_included = true;
                    break;
                }

                if west == symbol.location {
                    included_part_numbers.push(part_number.value);
                    is_already_included = true;
                    break;
                }

                if north_west == symbol.location {
                    included_part_numbers.push(part_number.value);
                    is_already_included = true;
                    break;
                }
            }
        }
    }

    let answer: u32 = included_part_numbers.iter().sum();
    answer
}

#[derive(Debug, Clone, PartialEq)]
struct Location {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct PartNumber {
    value: u32,
    locations: Vec<Location>,
}

#[derive(Debug, Clone)]
struct Symbol {
    location: Location,
}

fn parse_schematics(input: &str) -> (Vec<PartNumber>, Vec<Symbol>) {
    let rows = input.lines();
    let mut part_numbers: Vec<PartNumber> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();

    for (i, row) in rows.enumerate() {
        let mut part_number: String = String::new();
        let mut locations: Vec<Location> = Vec::new();

        for (j, char) in row.chars().enumerate() {
            let as_string = char.to_string();
            let character = as_string.as_str();
            let x = j as i32;
            let y = i as i32;
            let location = Location { x, y };

            let number_section = character.parse::<u32>();
            match number_section {
                Ok(n) => {
                    part_number += &n.to_string();
                    locations.push(location);
                }
                Err(_) => {
                    let number = part_number.parse::<u32>();
                    if let Ok(n) = number {
                        let part_number_obj = PartNumber {
                            value: n,
                            locations: locations,
                        };
                        part_numbers.push(part_number_obj);

                        // Reset
                        part_number = String::new();
                        locations = Vec::new();
                    }

                    if character != "." {
                        let symbol = Symbol {
                            location: location,
                        };
                        symbols.push(symbol);
                    }
                }
            }
        }
        let number = part_number.parse::<u32>();
        if let Ok(n) = number {
            let part_number_obj = PartNumber {
                value: n,
                locations: locations,
            };
            part_numbers.push(part_number_obj);
        }
    }
    (part_numbers, symbols)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input01_test.txt");
        let result = part01(input);
        assert_eq!(result, 4361);
    }
}
