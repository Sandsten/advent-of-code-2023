fn main() {
    let input = include_str!("./input01.txt");
    let answer = part01(input);
    println!("The answer is: {}", answer);
}

fn part01(input: &str) -> u32 {
    let mut line = input.lines();
    let race_durations = line.next().expect("No times found");
    let distance_records = line.next().expect("No distance records found");

    let race_durations: Vec<u32> = race_durations
        .split_whitespace()
        .filter_map(|value| value.parse::<u32>().ok())
        .collect();

    let distance_records: Vec<u32> = distance_records
        .split_whitespace()
        .filter_map(|value| value.parse::<u32>().ok())
        .collect();

    let mut answer: u32 = 1;

    for i in 0..race_durations.len() {
        let race_duration = race_durations[i];
        let distance_record = distance_records[i];
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
        answer *= nr_of_ways_to_beat_record;
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
        assert_eq!(result, 288);
    }
}