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

    let mut result = 0;

    ranges.sort();

    let mut new_ranges: Vec<(usize, usize)> = vec![];

    for (init, end) in ranges {
        if let Some(last_mut) = new_ranges.last_mut() {
            if init <= last_mut.1 {
                last_mut.1 = last_mut.1.max(end);
                continue;
            }
        }

        new_ranges.push((init, end));
    }

    for (init, end) in new_ranges {
        result += end - init + 1;
    }

    println!("result: {}", result);
}
