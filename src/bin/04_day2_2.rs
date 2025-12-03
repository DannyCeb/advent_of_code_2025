use std::fs;

fn main() {
    let input =
        fs::read_to_string("/home/danny/learning/rust/advent_of_code_2025/input/day2.txt").unwrap();

    let result = input.split(",").fold(0, |mut sum, range| {
        let mut iter = range.split('-');
        let init = iter.next().unwrap().parse::<i64>().unwrap();
        let end = iter.next().unwrap().parse::<i64>().unwrap();

        for num in init..=end {
            let s = num.to_string();
            let len = s.len();
            for pat_len in 1..=len / 2 {
                if len % pat_len == 0 {
                    let pattern = &s[0..pat_len];
                    let repeated = pattern.repeat(len / pat_len);
                    if repeated == s {
                        sum += num;
                        break;
                    }
                }
            }
        }

        sum
    });

    println!("{}", result);
}
