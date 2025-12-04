use std::fs;
// 173848577117276

fn main() {
    let input =
        fs::read_to_string("/home/danny/learning/rust/advent_of_code_2025/input/day3.txt").unwrap();

    let result = input.lines().fold(0_i64, |acc, line| {
        let line: Vec<u8> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();

        let n = line.len();
        //                                              indice, elementos
        let mut dp = vec![vec![i64::MIN; n + 1]; 13];

        dp[0][0] = 0;

        for i in 0..n {
            for e in 0..12 {
                if dp[e][i] == i64::MIN {
                    continue;
                }
                dp[e][i + 1] = dp[e][i + 1].max(dp[e][i]);
                let suma = dp[e][i] * 10 + line[i] as i64;
                dp[e + 1][i + 1] = dp[e + 1][i + 1].max(suma);
            }
        }

        acc + *dp.last().unwrap().iter().max().unwrap()
    });

    println!("{}", result);
}
