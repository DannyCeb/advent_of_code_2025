use std::{cmp::Reverse, collections::BinaryHeap, fs};

// Un wrapper para f64 que implementa las traits Ord y Eq.
// Esto es necesario para que los valores de tipo f64 (distancias) puedan ser
// almacenados en un BinaryHeap, que requiere que sus elementos sean ordenables.
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
        // .unwrap() asume que no se compararán valores NaN (Not a Number).
        self.partial_cmp(other).unwrap()
    }
}

// Implementación de una estructura de datos Disjoint Set Union (DSU) o Union-Find.
// Se utiliza para mantener un seguimiento de un conjunto de elementos particionados
// en varios subconjuntos disjuntos (los "componentes conectados").
struct Pointers {
    parent: Vec<usize>, // parent[i] almacena el padre del elemento i.
    size: Vec<usize>,   // size[i] almacena el tamaño del conjunto si i es una raíz.
}

impl Pointers {
    // Crea una nueva instancia de DSU con 'n' elementos, cada uno en su propio conjunto.
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(), // Inicialmente, cada elemento es su propio padre.
            size: vec![1; n],         // Cada conjunto tiene tamaño 1.
        }
    }

    // Encuentra el representante (raíz) del conjunto que contiene a 'x'.
    // Utiliza compresión de caminos para optimizar búsquedas futuras.
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    // Une los conjuntos que contienen a 'fa' y 'fb' (se asume que son raíces).
    fn union(&mut self, fa: usize, fb: usize) {
        self.parent[fa] = fb; // El conjunto de 'fa' se une al de 'fb'.
        self.size[fb] += self.size[fa]; // Se actualiza el tamaño del nuevo conjunto.
        self.size[fa] = 0; // El tamaño del antiguo conjunto se pone a cero.
    }
}

// Lee un archivo y parsea cada línea como coordenadas 3D (x, y, z).
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

// Calcula la distancia euclidiana entre cada par de nodos y devuelve las aristas
// en un Min-Heap (un BinaryHeap que saca el menor elemento primero).
fn get_edges(nodes: Vec<(i64, i64, i64)>) -> BinaryHeap<Reverse<(OrderedF64, usize, usize)>> {
    let mut aux = vec![];

    // Itera sobre cada par de nodos para calcular la distancia.
    for i in 0..nodes.len() {
        let (x1, y1, z1) = nodes[i];
        for j in i + 1..nodes.len() {
            let (x2, y2, z2) = nodes[j];
            let distance = ((x1 - x2).pow(2) + (y1 - y2).pow(2) + (z1 - z2).pow(2)) as f64;
            let distance = distance.sqrt();
            // La tupla es (distancia, nodo_u, nodo_v).
            // Se usa Reverse para que el BinaryHeap (que es un Max-Heap por defecto)
            // funcione como un Min-Heap y nos dé la arista con menor distancia.
            aux.push(Reverse((OrderedF64(distance), i, j)));
        }
    }

    BinaryHeap::from(aux)
}

fn main() {
    // 1. Cargar los nodos (puntos 3D) desde el archivo.
    let nodes = get_nodes("/home/danny/learning/rust/advent_of_code_2025/input/day8.txt");
    let n = nodes.len();

    // 2. Calcular todas las aristas y guardarlas en un Min-Heap.
    let mut edges: BinaryHeap<Reverse<(OrderedF64, usize, usize)>> = get_edges(nodes);
    // 3. Inicializar la estructura DSU para gestionar los componentes.
    let mut pointers = Pointers::new(n);
    let mut count = 0;

    // 4. Procesar las 999 aristas más cortas. Este es el núcleo del algoritmo.
    // Es similar al algoritmo de Kruskal para encontrar un Árbol de Expansión Mínima,
    // pero se detiene después de un número fijo de iteraciones.
    while let Some(Reverse((_, u, v))) = edges.pop() {
        if count == 999 {
            break; // Detenerse después de procesar 999 aristas.
        }
        // Encuentra las raíces de los conjuntos a los que pertenecen los nodos 'u' y 'v'.
        let f1 = pointers.find(u);
        let f2 = pointers.find(v);

        // Si los nodos no están ya en el mismo componente, únelos.
        if f1 != f2 {
            pointers.union(f1, f2);
        }
        count += 1;
    }

    // 5. Calcular el resultado final.
    // Ordena los tamaños de los componentes de mayor a menor.
    pointers.size.sort_by(|a, b| b.cmp(a));

    // Multiplica los tamaños de los 3 componentes más grandes.
    let res = pointers
        .size
        .into_iter()
        .filter(|element| *element > 0) // Filtra componentes que fueron unidos a otros.
        .take(3)
        .fold(1, |res, val| res * val);
    println!("result: {}", res);
}
