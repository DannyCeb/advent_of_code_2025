use std::{collections::HashMap, fs};

fn dfs(
    node: &str,
    target: &str,
    mut has_dac: bool,
    mut has_fft: bool,
    graph: &HashMap<String, Vec<String>>,
    memo: &mut HashMap<(String, bool, bool), i64>,
) -> i64 {
    let memo_key = (node.to_string(), has_dac, has_fft);
    if let Some(value) = memo.get(&memo_key) {
        return *value;
    }

    if node == target {
        let result = if has_dac && has_fft { 1 } else { 0 };
        return result;
    }

    if node == "dac" {
        has_dac = true;
    }

    if node == "fft" {
        has_fft = true;
    }

    let value = if let Some(neis) = graph.get(node) {
        let mut sum = 0;

        for nei in neis {
            sum += dfs(nei, target, has_dac, has_fft, graph, memo);
        }

        sum
    } else {
        0
    };

    memo.insert(memo_key, value);

    value
}

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
    let init = &"svr".to_string();
    let mut memo = HashMap::new();
    let result = dfs(&init, &target, false, false, &graph, &mut memo);

    println!("res: {}", result);
}
