use std::env;

use std::fs::read_to_string;

// New method!
// For each line, find all text numbers and digit numbers. Add each found to a new string. Then just
// use the first and last characters from that string.

fn main() {
    let path = env::args().nth(1).unwrap();

    let mut calibration_values: Vec<i64> = Vec::new();

    for line in read_to_string(path).unwrap().lines().map(String::from) {
        // For the whole line... get each substring from [i..] where i starts at 0.
        // Check if that substring starts with one of the spelled numbers OR is a number.
        // Add to the new string.

        let mut num_line = String::new();

        for index in 0..line.len() {
            let substring_to_end = &line[index..];

            let first_char = substring_to_end.chars().nth(0).unwrap();

            if first_char.is_digit(10) {
                num_line.push(first_char);
            } else if substring_to_end.starts_with("one") {
                num_line.push('1');
            } else if substring_to_end.starts_with("two") {
                num_line.push('2');
            } else if substring_to_end.starts_with("three") {
                num_line.push('3');
            } else if substring_to_end.starts_with("four") {
                num_line.push('4');
            } else if substring_to_end.starts_with("five") {
                num_line.push('5');
            } else if substring_to_end.starts_with("six") {
                num_line.push('6');
            } else if substring_to_end.starts_with("seven") {
                num_line.push('7');
            } else if substring_to_end.starts_with("eight") {
                num_line.push('8');
            } else if substring_to_end.starts_with("nine") {
                num_line.push('9');
            }
        }

        let first = num_line.clone().chars().nth(0).unwrap();
        let last = num_line.clone().chars().nth_back(0).unwrap();

        let mut calibration_value = String::new();
        calibration_value.push(first);
        calibration_value.push(last);

        calibration_values.push(calibration_value.parse::<i64>().unwrap_or_default());
    }

    let sum: i64 = calibration_values.iter().sum();

    println!("Sum: {}", sum);


}
