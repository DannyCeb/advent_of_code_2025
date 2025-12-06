use std::fs;

fn main() {
    let input =
        fs::read_to_string("/home/danny/learning/rust/advent_of_code_2025/input/day6.txt").unwrap();

    let input: Vec<String> = input.lines().map(|line| line.to_string()).collect();

    let (numbers_list, instructions) = input.split_at(4);

    let instructions = instructions[0].split_ascii_whitespace();
    let numbers = numbers_list
        .iter()
        .map(|line| {
            let numbers = line.split_ascii_whitespace();
            numbers
                .map(|num| num.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let result = instructions
        .enumerate()
        .fold(0_u64, |res, (idx, instruction)| {
            let ins: char = instruction.chars().next().unwrap();
            let mut aux_res = if ins == '*' { 1 } else { 0 };
            for num_idx in 0..numbers.len() {
                if ins == '*' {
                    aux_res *= numbers[num_idx][idx];
                } else {
                    aux_res += numbers[num_idx][idx];
                };
            }

            aux_res + res
        });

    println!("result: {}", result);
}
