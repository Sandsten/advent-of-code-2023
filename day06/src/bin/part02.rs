fn main() {
    let input = include_str!("./input01.txt");
    let answer = part02(input);
    println!("The answer is: {}", answer);
}

fn part02(input: &str) -> u32 {
    let mut line = input.lines();
    let race_durations = line
        .next()
        .expect("Failed to find Race durations")
        .split(":")
        .last()
        .expect("Failed to remove Title");
    
    let distance_records = line
        .next()
        .expect("Failed to find Race durations")
        .split(":")
        .last()
        .expect("Failed to remove Title");

    let race_duration: u128 = race_durations
        .split_whitespace()
        .fold(String::new(), |this, next| this + next)
        .parse::<u128>()
        .expect("Failed to parse race duration");

    let distance_record: u128 = distance_records
        .split_whitespace()
        .fold(String::new(), |this, next| this + next)
        .parse::<u128>()
        .expect("Failed to parse distance record");

    let mut nr_of_ways_to_beat_record = 0;
    let mut stop_if_curve_going_down = false;

    for hold_duration in 1..race_duration {
        let distance = hold_duration * (race_duration - hold_duration);
        if distance > distance_record {
            nr_of_ways_to_beat_record += 1;
            stop_if_curve_going_down = true;
        } else if distance <= distance_record && stop_if_curve_going_down {
            break;
        }
    }
    
    nr_of_ways_to_beat_record
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input01_test.txt");
        let result = part02(input);
        assert_eq!(result, 71503);
    }
}
