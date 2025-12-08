use std::{cmp::Reverse, collections::BinaryHeap, fs};

struct OrderedF64(f64);

impl PartialEq for OrderedF64 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl Eq for OrderedF64 {}
impl PartialOrd for OrderedF64 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl Ord for OrderedF64 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

struct Pointers {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl Pointers {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, fa: usize, fb: usize) {
        self.parent[fa] = fb;
        self.size[fb] += self.size[fa];
        self.size[fa] = 0;
    }
}

fn get_nodes(path: &str) -> Vec<(i64, i64, i64)> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            let v = line
                .split(',')
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            (v[0], v[1], v[2])
        })
        .collect()
}

fn get_edges(nodes: Vec<(i64, i64, i64)>) -> BinaryHeap<Reverse<(OrderedF64, usize, usize)>> {
    let mut aux = vec![];

    for i in 0..nodes.len() {
        let (x1, y1, z1) = nodes[i];
        for j in i + 1..nodes.len() {
            let (x2, y2, z2) = nodes[j];
            let distance = ((x1 - x2).pow(2) + (y1 - y2).pow(2) + (z1 - z2).pow(2)) as f64;
            let distance = distance.sqrt();
            aux.push(Reverse((OrderedF64(distance), i, j)));
        }
    }

    BinaryHeap::from(aux)
}

fn main() {
    let nodes = get_nodes("/home/danny/learning/rust/advent_of_code_2025/input/day8.txt");
    let n = nodes.len();

    let mut edges: BinaryHeap<Reverse<(OrderedF64, usize, usize)>> = get_edges(nodes);
    let mut pointers = Pointers::new(n);
    let mut count = 0;

    while let Some(Reverse((_, u, v))) = edges.pop() {
        if count == 999 {
            break;
        }
        let f1 = pointers.find(u);
        let f2 = pointers.find(v);

        if f1 != f2 {
            pointers.union(f1, f2);
        }
        count += 1;
    }

    pointers.size.sort_by(|a, b| b.cmp(a));

    let res = pointers
        .size
        .into_iter()
        .filter(|element| *element > 0)
        .take(3)
        .fold(1, |res, val| res * val);
    println!("result: {}", res);
}
