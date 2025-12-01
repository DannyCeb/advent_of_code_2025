use std::fs;

const BUFF_SIZE: i32 = 100;

fn main() {
    let input =
        fs::read_to_string("/home/danny/learning/rust/advent_of_code_2025/input/day1.txt").unwrap();

    let (_final_cursor, result) = input
        .lines()
        .map(|line| {
            let sign = if line.starts_with('R') { 1 } else { -1 };
            let value: i32 = line[1..].parse().unwrap();
            sign * value
        })
        .fold((50, 0), |(mut cursor, mut result), order| {
            let direction = order.signum();
            for _ in 0..order.abs() {
                cursor = (cursor + direction).rem_euclid(BUFF_SIZE);

                if cursor == 0 {
                    result += 1;
                }
            }
            (cursor, result)
        });

    println!("{}", result);
}
