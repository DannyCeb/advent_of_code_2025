use std::fs;

fn main() {
    let input =
        fs::read_to_string("/home/danny/learning/rust/advent_of_code_2025/input/day5.txt").unwrap();

    let mut input_iter = input.lines();

    let mut ranges: Vec<(usize, usize)> = vec![];
    while let Some(line) = input_iter.next() {
        if line == "".to_string() {
            break;
        }

        let mut line_iter = line.split("-");
        let init = line_iter.next().unwrap().parse::<usize>().unwrap();
        let end = line_iter.next().unwrap().parse::<usize>().unwrap();

        ranges.push((init, end));
    }

    let result = input_iter.fold(0, |res, line| {
        let num = line.parse::<usize>().unwrap();
        println!("looking for: {}", num);
        for (init, end) in ranges.iter() {
            if num >= *init && num <= *end {
                return res + 1;
            }
        }
        res
    });

    println!("result: {}", result);
}
