use std::fs;
use std::iter::Rev;
use std::str::Chars;

fn get_digits(numbers: &mut Vec<Rev<Chars<'_>>>) -> Vec<u8> {
    let mut res: Vec<u8> = vec![];

    for idx in 0..numbers.len() {
        if let Some(item) = numbers[idx].next() {
            if let Some(num) = item.to_digit(10) {
                res.push(num as u8);
            }
        }
    }

    res
}

fn main() {
    let input =
        fs::read_to_string("/home/danny/learning/rust/advent_of_code_2025/input/day6.txt").unwrap();

    let input: Vec<String> = input.lines().map(|line| line.to_string()).collect();

    let (numbers_list, instructions) = input.split_at(4);

    let instructions = instructions[0]
        .split_ascii_whitespace()
        .map(|ins| ins.chars().next().unwrap())
        .rev()
        .collect::<Vec<char>>();

    let mut numbers: Vec<Rev<Chars<'_>>> = numbers_list
        .iter()
        .map(|line| line.chars().rev())
        .collect::<Vec<_>>();

    let mut result: u64 = 0;

    for ins in instructions {
        let mut aux_res: u64 = if ins == '*' { 1 } else { 0 };

        loop {
            let digits = get_digits(&mut numbers);

            if digits.len() == 0 {
                break;
            }
            let mut num: u64 = 0;

            for digit in digits {
                num = num * 10 + digit as u64;
            }
            if ins == '*' {
                aux_res *= num
            } else {
                aux_res += num
            };
        }
        result += aux_res;
    }

    println!("result: {}", result);
}
