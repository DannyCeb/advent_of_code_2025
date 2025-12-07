use std::fs;

fn main() {
    let mut matrix: Vec<Vec<char>> =
        fs::read_to_string("/home/danny/learning/rust/advent_of_code_2025/input/day7.txt")
            .unwrap()
            .lines()
            .map(|line| line.chars().collect())
            .collect();

    for i in 0..matrix[0].len() {
        if matrix[0][i] == 'S' {
            matrix[1][i] = '|';
        }
    }

    let mut result = 0;

    for i in 1..matrix.len() - 1 {
        for j in 0..matrix[i].len() {
            let mut splited = false;
            match matrix[i][j] {
                '^' => {
                    if matrix[i - 1][j] == '|' {
                        let (nj1, nj2) = ((j as i32 - 1) as usize, j + 1);

                        if nj1 < matrix[i].len() && matrix[i + 1][nj1] != '^' {
                            matrix[i + 1][nj1] = '|';
                            splited = true;
                        }

                        if nj2 < matrix[i].len() && matrix[i + 1][nj1] != '^' {
                            matrix[i + 1][nj2] = '|';
                            splited = true;
                        }
                    }
                }
                '|' => {
                    if matrix[i + 1][j] == '.' {
                        matrix[i + 1][j] = '|';
                    }
                }
                _ => {
                    continue;
                }
            }
            if splited {
                result += 1;
            }
        }
    }

    println!("{}", result)
}
