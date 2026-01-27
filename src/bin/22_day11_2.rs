use std::{collections::HashMap, fs};

// Esta función realiza una Búsqueda en Profundidad (DFS) con memoización para contar caminos válidos.
// Un camino es válido si va de 'node' a 'target' y pasa por "dac" y "fft".
fn dfs(
    node: &str,                          // El nodo actual en el grafo.
    target: &str,                        // El nodo objetivo final ("out").
    mut has_dac: bool,                   // true si el camino actual ya ha pasado por "dac".
    mut has_fft: bool,                   // true si el camino actual ya ha pasado por "fft".
    graph: &HashMap<String, Vec<String>>, // Referencia al grafo.
    memo: &mut HashMap<(String, bool, bool), i64>, // Caché para memoización.
) -> i64 {
    // La clave del caché debe capturar el estado completo de la recursión:
    // el nodo actual y si ya hemos visitado "dac" y "fft" en este camino.
    let memo_key = (node.to_string(), has_dac, has_fft);
    // Si ya hemos calculado el resultado para este estado, lo devolvemos directamente.
    // Esto evita cálculos redundantes y es la esencia de la memoización.
    if let Some(value) = memo.get(&memo_key) {
        return *value;
    }

    // Caso Base: Hemos llegado al nodo objetivo.
    if node == target {
        // El camino solo cuenta como 1 si hemos pasado por AMBOS "dac" y "fft".
        let result = if has_dac && has_fft { 1 } else { 0 };
        return result;
    }

    // Actualiza las banderas para las llamadas recursivas subsiguientes desde este nodo.
    // Si el nodo actual es "dac" o "fft", marcamos la bandera correspondiente como true.
    if node == "dac" {
        has_dac = true;
    }

    if node == "fft" {
        has_fft = true;
    }

    // Paso Recursivo: Explora los vecinos del nodo actual.
    let value = if let Some(neis) = graph.get(node) {
        let mut sum = 0;

        // Suma los resultados de los caminos válidos encontrados desde cada vecino.
        for nei in neis {
            sum += dfs(nei, target, has_dac, has_fft, graph, memo);
        }

        sum
    } else {
        // Si el nodo no tiene vecinos (y no es el target), es un callejón sin salida.
        0
    };

    // Antes de devolver el valor, lo guardamos en el caché para uso futuro.
    memo.insert(memo_key, value);

    value
}

fn main() {
    // La construcción del grafo es idéntica a la del ejercicio anterior.
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

    // Se definen los nodos de inicio y fin. Notar que el inicio es "svr".
    let target = &"out".to_string();
    let init = &"svr".to_string();
    
    // Se inicializa el caché para la memoización.
    let mut memo = HashMap::new();
    
    // Se inicia la búsqueda DFS desde el nodo inicial "svr".
    // Las banderas 'has_dac' y 'has_fft' empiezan en 'false'.
    let result = dfs(&init, &target, false, false, &graph, &mut memo);

    println!("res: {}", result);
}
