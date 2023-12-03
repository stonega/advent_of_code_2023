use crate::day02::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let mut results: Vec<u32> = Vec::new();
    for (i, d) in input.iter().enumerate() {
        let (mut r, mut g, mut b) = (0u32, 0u32, 0u32);
        for res in d.iter() {
            let (red, blue, green) = res;
            if u32::from(*red) > r {
                r = u32::from(*red);
            }
            if u32::from(*blue) > b {
                b = u32::from(*blue);
            }
            if u32::from(*green) > g {
                g = u32::from(*green);
            }
        }
        println!("{:?}", (r, b, g));
        results.push((r * b * g));
    }
    results.iter().sum::<u32>().into()
}
