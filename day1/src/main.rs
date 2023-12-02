use std::env;

use std::fs::read_to_string;

fn main() {
    let path = env::args().nth(1).unwrap();

    let mut calibration_values: Vec<i64> = Vec::new();

    for line in read_to_string(path).unwrap().lines() {
        let mut calibration_value = String::new();

        for char in line.chars() {
            if char.is_digit(10) {
                calibration_value.push(char);
                break;
            }
        }

        for char in line.chars().rev() {
            if char.is_digit(10) {
                calibration_value.push(char);
                break;
            }
        }

        calibration_values.push(calibration_value.parse::<i64>().unwrap_or_default());
    }

    let sum = calibration_values.iter().sum::<i64>();
    println!("Sum: {}", sum);
}
