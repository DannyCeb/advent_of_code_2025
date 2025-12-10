use std::fs;

fn main() {
    let coords: Vec<(i128, i128)> =
        fs::read_to_string("/home/danny/learning/rust/advent_of_code_2025/input/day9.txt")
            .unwrap()
            .lines()
            .map(|line| {
                let nums: Vec<i128> = line
                    .split(",")
                    .map(|num| num.parse::<i128>().unwrap())
                    .collect();
                (nums[0], nums[1])
            })
            .collect();
    let mut res = i128::MIN;
    for (i, (x1, y1)) in coords.iter().enumerate() {
        for (x2, y2) in coords[i + 1..].iter() {
            let l1 = (*x1 - *x2).abs() + 1;
            let l2 = (*y1 - *y2).abs() + 1;
            let area = l1 * l2;
            res = res.max(area);
        }
    }

    println!("result: {}", res)
}
