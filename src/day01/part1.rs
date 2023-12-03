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
    let mut vec: Vec<u32> = Vec::new();
    for value in value.iter() {
        if value.is_digit(10) {
            vec.push(value.to_digit(10).unwrap());
        }
    }
    println!("{:?}", vec);
    if (vec.len() == 0) {
        return Ok(0);
    } else if (vec.len() == 1) {
        return Ok(vec[0] * 10 + vec[0]);
    } else {
        return Ok(vec[0] * 10 + vec.last().unwrap());
    }
}
