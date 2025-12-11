use std::{
    collections::{HashMap, VecDeque},
    fs,
};

fn main() {
    let input = fs::read_to_string("/home/danny/learning/rust/advent_of_code_2025/input/day11.txt")
        .unwrap();

    let graph: HashMap<String, Vec<String>> =
        input.lines().fold(HashMap::new(), |mut map, line| {
            let mut iter = line.split(":");

            let node = iter.next().unwrap().to_string();

            let neis: Vec<String> = iter
                .next()
                .unwrap()
                .split(" ")
                .map(|nei| nei.trim().to_string())
                .filter(|nei| nei != "")
                .collect();

            map.entry(node).or_insert(neis);

            map
        });

    let target = &"out".to_string();
    let init = &"you".to_string();
    let mut result = 0;

    let mut q: VecDeque<&String> = VecDeque::new();
    q.push_back(init);

    while let Some(node) = q.pop_front() {
        if node == target {
            result += 1;
            continue;
        }

        if let Some(neis) = graph.get(node) {
            for nei in neis {
                q.push_back(nei);
            }
        }
    }

    println!("res: {}", result);
}
