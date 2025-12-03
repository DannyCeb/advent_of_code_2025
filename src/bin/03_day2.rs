use std::fs;
// 18595663903
fn main() {
    let input =
        fs::read_to_string("/home/danny/learning/rust/advent_of_code_2025/input/day2.txt").unwrap();

    let result = input.split(",").fold(0, |mut sum, range| {
        let mut iter = range.split("-");
        let init = iter.next().unwrap().parse::<i64>().unwrap();
        let end = iter.next().unwrap().parse::<i64>().unwrap();

        for num in init..=end {
            let num_s = num.to_string();
            if num_s.len() % 2 == 0 {
                let num_b = num_s.as_bytes();
                let (first, last) = num_b.split_at(num_b.len() / 2);

                if first == last {
                    sum += num;
                }
            }
        }

        sum
    });

    println!("{}", result);
}
