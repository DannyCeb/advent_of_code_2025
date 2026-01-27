use std::{
    collections::{HashMap, VecDeque},
    fs,
};

fn main() {
    let input = fs::read_to_string("/home/danny/learning/rust/advent_of_code_2025/input/day11.txt")
        .unwrap();

    // Construye un grafo dirigido a partir del archivo de entrada.
    // El grafo se representa como un HashMap donde la clave es un nodo (String)
    // y el valor es un vector de sus vecinos (Vec<String>).
    let graph: HashMap<String, Vec<String>> =
        input.lines().fold(HashMap::new(), |mut map, line| {
            let mut iter = line.split(":");

            // El nodo de origen es la parte antes de los dos puntos.
            let node = iter.next().unwrap().to_string();

            // Los nodos de destino (vecinos) están después de los dos puntos, separados por espacios.
            let neis: Vec<String> = iter
                .next()
                .unwrap()
                .split(" ")
                .map(|nei| nei.trim().to_string())
                .filter(|nei| nei != "") // Filtra strings vacíos que pueden resultar del parseo.
                .collect();

            // Inserta la entrada en el grafo: nodo -> [vecinos].
            map.entry(node).or_insert(neis);

            map
        });

    // Define el nodo objetivo ("out") y el nodo de inicio ("you").
    let target = &"out".to_string();
    let init = &"you".to_string();
    let mut result = 0;

    // Se utiliza una cola (VecDeque) para implementar una búsqueda en anchura (BFS).
    let mut q: VecDeque<&String> = VecDeque::new();
    q.push_back(init); // Comienza la búsqueda desde el nodo "you".

    // El bucle se ejecuta mientras haya nodos en la cola para procesar.
    while let Some(node) = q.pop_front() {
        // Comprueba si el nodo actual es el objetivo.
        if node == target {
            result += 1; // Incrementa el contador cada vez que se llega al objetivo.
            continue; // Continúa la búsqueda para encontrar otros caminos.
        }

        // Si el nodo actual tiene vecinos, los añade a la cola.
        if let Some(neis) = graph.get(node) {
            for nei in neis {
                q.push_back(nei);
            }
        }
    }
    // NOTA IMPORTANTE: Este algoritmo no lleva un registro de los nodos visitados.
    // Esto significa que un nodo puede ser visitado múltiples veces si hay varios caminos que llevan a él.
    // En un Grafo Acíclico Dirigido (DAG), este comportamiento cuenta el número total de caminos
    // distintos desde el nodo de inicio ("you") hasta el nodo final ("out").
    // Si el grafo tuviera ciclos, este bucle podría no terminar nunca.

    println!("res: {}", result);
}
