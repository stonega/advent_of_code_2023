use crate::day02::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let mut results: Vec<u32> = Vec::new();
    for (i, d) in input.iter().enumerate() {
        let mut v = true;
        for res in d.iter() {
            let (red, blue, green) = res;
            if *red > 12 || *blue > 14 || *green > 13 {
                v = false;
            }
        }
        if v == true {
            results.push((i + 1).try_into().unwrap());
        }
    }
    results.iter().sum::<u32>().into()
}
