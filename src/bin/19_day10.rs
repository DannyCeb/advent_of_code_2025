use std::collections::VecDeque;
use std::fs;

// Calcula el número mínimo de pasos (pulsaciones de botón) para alcanzar un estado 'target' desde el estado 0.
// Utiliza una Búsqueda en Anchura (BFS), que es ideal para encontrar el camino más corto en un grafo no ponderado.
// Los "nodos" del grafo son los posibles estados (representados por máscaras de bits), y las "aristas" son las pulsaciones de botón.
fn min_steps(target: u16, n_bits: u8, buttons: Vec<Vec<u8>>) -> Option<usize> {
    // El número total de estados posibles es 2 elevado al número de bits.
    let max_state = 1 << n_bits;
    // 'dp' almacena la distancia (número de pasos) desde el estado 0 a cada otro estado.
    // Se inicializa con un valor máximo para marcar los estados no visitados.
    let mut dp = vec![usize::MAX; max_state];
    // 'queue' es la cola para el algoritmo BFS, que almacena los estados a visitar.
    let mut queue = VecDeque::new();

    // El estado inicial es 0 (todos los bits a 0), y la distancia para llegar a él es 0 pasos.
    dp[0] = 0;
    queue.push_back(0);

    // El bucle BFS continúa mientras haya estados en la cola por procesar.
    while let Some(state) = queue.pop_front() {
        let steps = dp[state];

        // Si hemos alcanzado el estado objetivo, devolvemos el número de pasos.
        // Como BFS explora por niveles, la primera vez que llegamos al objetivo es garantizado el camino más corto.
        if state == target as usize {
            return Some(steps);
        }

        // Prueba cada "botón" desde el estado actual.
        for button in &buttons {
            let mut new_state = state;
            // Un botón invierte un conjunto de bits. La operación XOR (^) es perfecta para esto.
            for &pos in button {
                if pos < n_bits {
                    new_state ^= 1 << pos; // Invierte el bit en la posición 'pos'.
                }
            }

            // Si hemos encontrado un camino más corto hacia 'new_state' (o es la primera vez que lo visitamos)...
            if dp[new_state] > steps + 1 {
                // ...actualizamos su distancia y lo añadimos a la cola para explorarlo más tarde.
                dp[new_state] = steps + 1;
                queue.push_back(new_state);
            }
        }
    }

    // Si la cola se vacía y no hemos llegado al 'target', es inalcanzable.
    None
}

fn main() {
    let input = fs::read_to_string("/home/danny/learning/rust/advent_of_code_2025/input/day10.txt")
        .unwrap();

    // --- PARSEO DEL INPUT ---
    // 1. Extraer el estado objetivo (target) y el número de bits (n_bits) de cada línea.
    let bits: Vec<(u16, u8)> = input
        .lines()
        .map(|line| {
            // El primer elemento de la línea es el patrón de bits (ej: ".##.").
            let bit: Vec<char> = line.split(" ").next().unwrap().chars().collect();
            let n = bit.len();

            // Convierte el patrón de caracteres en una máscara de bits (u16).
            let mut mask: u16 = 0;
            for i in 1..n - 1 { // Ignora el primer y último carácter.
                if bit[i] == '#' {
                    let i = i - 1;
                    mask |= 1 << i; // Pone el bit 'i' a 1 si el carácter es '#'.
                }
            }
            // El número de bits relevantes y la máscara objetivo.
            (mask, (1..n - 1).count() as u8)
        })
        .collect();

    // 2. Extraer la configuración de los botones de cada línea.
    let buttons = input
        .lines()
        .map(|line| {
            // Los botones son los elementos después del patrón de bits (ej: "(0,1)" "(2)").
            let buttons = line.split(" ").skip(1);
            let mut v_buttons = vec![];

            for button in buttons {
                if button.starts_with("(") {
                    let n = button.len();
                    // Parsea los números dentro de los paréntesis.
                    let nums = &button[1..n - 1];
                    let nums = nums
                        .split(",")
                        .map(|num| num.parse::<u8>().unwrap())
                        .collect::<Vec<_>>();
                    v_buttons.push(nums); // Añade el botón a la lista.
                }
            }
            v_buttons
        })
        .collect::<Vec<_>>();

    // --- EJECUCIÓN DEL ALGORITMO ---
    let mut res = 0;
    // Itera sobre cada puzzle (target y sus botones) y suma los resultados.
    for ((target, n_bits), buttons) in bits.into_iter().zip(buttons.into_iter()) {
        match min_steps(target, n_bits, buttons) {
            Some(steps) => res += steps,
            None => println!("Target: {:b}, unreachable", target),
        }
    }

    println!("{}", res);
}
