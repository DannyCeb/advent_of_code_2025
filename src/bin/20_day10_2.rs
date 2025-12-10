use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::fs;
fn validate_state(target: &Vec<u16>, candidate: &Vec<u16>) -> bool {
    // El candidato nunca debe exceder el target en ninguna posición
    for idx in 0..target.len() {
        if candidate[idx] > target[idx] {
            return false;
        }
    }
    true
}
fn min_steps(target: &Vec<u16>, buttons: Vec<Vec<usize>>) -> Option<usize> {
    let mut costs: HashMap<Vec<u16>, usize> = HashMap::new();
    let mut heap = BinaryHeap::new(); // Estado inicial: todos ceros 
    let start = vec![0; target.len()];
    costs.insert(start.clone(), 0);
    heap.push(Reverse((0, start)));
    while let Some(Reverse((steps, state))) = heap.pop() {
        // Si ya tenemos mejor costo, saltamos
        if let Some(&best) = costs.get(&state) {
            if steps > best {
                continue;
            }
        } // Caso base: alcanzamos el target exacto 
        if state == *target {
            return Some(steps);
        } // Expandimos vecinos aplicando cada botón 
        for button in &buttons {
            let mut new_state = state.clone();
            for &idx in button {
                if idx < new_state.len() {
                    new_state[idx] += 1;
                }
            } // Validamos que no exceda el target 
            if !validate_state(target, &new_state) {
                continue;
            }
            let next_steps = steps + 1;
            if next_steps < *costs.get(&new_state).unwrap_or(&usize::MAX) {
                costs.insert(new_state.clone(), next_steps);
                heap.push(Reverse((next_steps, new_state)));
            }
        }
    }
    None
}
fn main() {
    let input = fs::read_to_string("/home/danny/learning/rust/advent_of_code_2025/input/day10.txt")
        .unwrap();
    let joltage: Vec<Vec<u16>> = input
        .lines()
        .map(|line| {
            let jolt_target = line
                .split(" ")
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .next()
                .unwrap();
            let n = jolt_target.len();
            jolt_target[1..n - 1]
                .split(",")
                .map(|num| num.parse::<u16>().unwrap())
                .collect::<Vec<_>>()
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
                        .map(|num| num.parse::<usize>().unwrap())
                        .collect::<Vec<_>>();
                    v_buttons.push(nums);
                }
            }
            v_buttons
        })
        .collect::<Vec<_>>();
    let mut sum = 0;
    let mut c = 1;
    for (target, buttons) in joltage.into_iter().zip(buttons.into_iter()) {
        println!("line: {}", c);
        c += 1;
        match min_steps(&target, buttons) {
            Some(steps) => sum += steps,
            None => println!("Target {:?}, unreachable", target),
        }
    }
    println!("{}", sum);
}
