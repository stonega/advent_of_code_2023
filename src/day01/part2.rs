use regex::Regex;

use crate::day01::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let values = input
        .iter()
        .map(try_parse_calibration_value)
        .collect::<Result<Vec<u32>, _>>()
        .expect("Could not parse input!");
    let sum: u32 = values.iter().sum();
    sum.into()
}

fn try_parse_calibration_value(value: &Vec<char>) -> Result<u32, std::num::ParseIntError> {
    let input = value.iter().collect::<String>();
    let re = Regex::new(r"([1-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let mut numbers: Vec<u32> = Vec::new();

    let mut new_input = input.clone();
    while new_input.len() > 0 {
        let mut results = vec![];
        for (_, [value]) in re.captures_iter(&new_input).map(|c| c.extract()) {
            results.push(value);
        }
        println!("{:?}", results);
        if results.len() == 0 {
            new_input = String::from("");
            break;
        }
        let cap = results[0];
        numbers.push(convert_to_number(cap));
        new_input.replace_range(0..1, "");
        println!("{:?}", new_input);
    }
    println!("{:?}", numbers);
    if (numbers.len() == 0) {
        return Ok(0);
    } else if (numbers.len() == 1) {
        return Ok(numbers[0] * 10 + numbers[0]);
    } else {
        return Ok(numbers[0] * 10 + numbers.last().unwrap());
    }
}

fn convert_to_number(value: &str) -> u32 {
    match value {
        "one" | "1" => 1,
        "two" | "2" => 2,
        "three" | "3" => 3,
        "four" | "4" => 4,
        "five" | "5" => 5,
        "six" | "6" => 6,
        "seven" | "7" => 7,
        "eight" | "8" => 8,
        "nine" | "9" => 9,
        _ => 0,
    }
}
