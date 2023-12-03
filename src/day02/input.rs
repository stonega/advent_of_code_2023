use crate::day02::Input;

const EXAMPLE_INPUT: &str = include_str!("../../input/02/example.txt");
const INPUT: &str = include_str!("../../input/02/input.txt");

pub fn read() -> Input {
    read_internal(INPUT)
}

pub fn read_example() -> Input {
    read_internal(EXAMPLE_INPUT)
}

/// Read and parse the input file
pub fn read_internal(input: &str) -> Input {
    let lines = input
        .trim()
        .split("\n")
        .into_iter()
        .map(extract_game_info)
        .collect::<Vec<Vec<(u8, u8, u8)>>>();
    lines
}

fn extract_game_info(line: &str) -> Vec<(u8, u8, u8)> {
    let mut results = Vec::new();
    let games = line
        .split(":")
        .last()
        .unwrap()
        .trim()
        .split(";")
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.trim())
        .collect::<Vec<&str>>();
    for game in games.iter() {
        let res = game
            .split(",")
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| s.trim())
            .collect::<Vec<&str>>();
        println!("{:?}", res);
        let (mut red, mut green, mut blue) = (0, 0, 0);
        for r in res {
            let p = r.trim().split(" ").collect::<Vec<&str>>();
            if p[1] == "red" {
                red = p[0].parse().unwrap();
            } else if p[1] == "blue" {
                blue = p[0].parse().unwrap();
            } else if p[1] == "green" {
                green = p[0].parse().unwrap();
            }
        }
        results.push((red, blue, green));
    }
    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_input() {
        let input = read();
        println!("{:?}", input);
    }
}
