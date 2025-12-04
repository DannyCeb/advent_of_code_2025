use std::fs;

fn main() {
    let input =
        fs::read_to_string("/home/danny/learning/rust/advent_of_code_2025/input/day4.txt").unwrap();

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut result = 0;

    let directions: [(i32, i32); 8] = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (1, 1),
        (0, 1),
    ];

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '@' {
                let mut aux_counter = 0;
                for (xi, yi) in directions {
                    let ni = (i as i32 + xi) as usize;
                    let nj = (j as i32 + yi) as usize;

                    if ni < grid.len() && nj < grid[i].len() && grid[ni][nj] == '@' {
                        aux_counter += 1;
                    }
                }

                result += if aux_counter < 4 { 1 } else { 0 };
            }
        }
    }

    println!("result: {}", result);
}
