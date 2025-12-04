use std::fs;

fn main() {
    let input =
        fs::read_to_string("/home/danny/learning/rust/advent_of_code_2025/input/day3.txt").unwrap();

    let result = input.lines().fold(0_i64, |result, line| {
        println!("line: {:?}", line);
        let line: Vec<u8> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();

        let (mut num_mas_grande, mut digito_mas_grande) =
            (line[0] * 10 + line[1], line[0].max(line[1]));

        for idx in 2..line.len() {
            let idx_num = digito_mas_grande * 10 + line[idx];
            num_mas_grande = num_mas_grande.max(idx_num);
            digito_mas_grande = digito_mas_grande.max(line[idx]);
        }

        result + num_mas_grande as i64
    });

    println!("{}", result);
}
