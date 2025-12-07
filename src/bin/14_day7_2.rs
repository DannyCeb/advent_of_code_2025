use std::fs;

fn main() {
    let matrix: Vec<Vec<char>> =
        fs::read_to_string("/home/danny/learning/rust/advent_of_code_2025/input/day7.txt")
            .unwrap()
            .lines()
            .map(|line| line.chars().collect())
            .collect();

    let rows = matrix.len();
    let cols = matrix[0].len();

    let start_col = matrix[0].iter().position(|&c| c == 'S').unwrap();

    let mut dp = vec![vec![0u128; cols]; rows];
    dp[0][start_col] = 1;

    for i in 0..rows - 1 {
        for j in 0..cols {
            let timelines = dp[i][j];
            if timelines == 0 {
                continue;
            }

            match matrix[i][j] {
                '.' | 'S' | '|' => {
                    dp[i + 1][j] += timelines;
                }
                '^' => {
                    if j > 0 {
                        dp[i + 1][j - 1] += timelines;
                    }
                    if j + 1 < cols {
                        dp[i + 1][j + 1] += timelines;
                    }
                }
                _ => {}
            }
        }
    }

    let result: u128 = dp[rows - 1].iter().sum();
    println!("{}", result);
}
