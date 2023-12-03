use crate::day01::Input;

const EXAMPLE_INPUT: &str = include_str!("../../input/01/example.txt");
const INPUT: &str = include_str!("../../input/01/input.txt");

pub fn read() -> Input {
    read_internal(INPUT)
}

pub fn read_example() -> Input {
    read_internal(EXAMPLE_INPUT)
}

/// Read and parse the input file
pub fn read_internal(input: &str) -> Input {
    input
        .trim()
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_input() {
        let input = read_example();

        let first = input.first().unwrap();
        assert_eq!(*first, "two1nine".chars().collect::<Vec<char>>());

        let last = input.last().unwrap();
        assert_eq!(*last, "7pqrstsixteen".chars().collect::<Vec<char>>());
    }
}
