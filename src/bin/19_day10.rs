use std::collections::VecDeque;
use std::fs;

fn min_steps(target: u16, n_bits: u8, buttons: Vec<Vec<u8>>) -> Option<usize> {
    let max_state = 1 << n_bits;
    let mut dp = vec![usize::MAX; max_state];
    let mut queue = VecDeque::new();

    dp[0] = 0;
    queue.push_back(0);

    while let Some(state) = queue.pop_front() {
        let steps = dp[state];

        if state == target as usize {
            return Some(steps);
        }

        for button in &buttons {
            let mut new_state = state;
            for &pos in button {
                if pos < n_bits {
                    new_state ^= 1 << pos;
                }
            }

            if dp[new_state] > steps + 1 {
                dp[new_state] = steps + 1;
                queue.push_back(new_state);
            }
        }
    }

    None
}

fn main() {
    let input = fs::read_to_string("/home/danny/learning/rust/advent_of_code_2025/input/day10.txt")
        .unwrap();

    let bits: Vec<(u16, u8)> = input
        .lines()
        .map(|line| {
            let bit: Vec<char> = line.split(" ").next().unwrap().chars().collect();
            let n = bit.len();

            let mut mask: u16 = 0;
            for i in 1..n - 1 {
                if bit[i] == '#' {
                    let i = i - 1;
                    mask |= 1 << i;
                }
            }
            (mask, (1..n - 1).count() as u8)
        })
        .collect();

    let buttons = input
        .lines()
        .map(|line| {
            let buttons = line.split(" ").skip(1);
            let mut v_buttons = vec![];

            for button in buttons {
                if button.starts_with("(") {
                    let n = button.len();
                    let nums = &button[1..n - 1];
                    let nums = nums
                        .split(",")
                        .map(|num| num.parse::<u8>().unwrap())
                        .collect::<Vec<_>>();
                    v_buttons.push(nums);
                }
            }
            v_buttons
        })
        .collect::<Vec<_>>();

    let mut res = 0;
    for ((target, n_bits), buttons) in bits.into_iter().zip(buttons.into_iter()) {
        match min_steps(target, n_bits, buttons) {
            Some(steps) => res += steps,
            None => println!("Target: {:b}, unreachable", target),
        }
    }

    println!("{}", res);
}
