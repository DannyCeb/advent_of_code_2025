use std::fs;

fn main() {
    // Lee el archivo de entrada y lo convierte en una matriz 2D de caracteres.
    // Cada línea del archivo se convierte en una fila de la matriz.
    let matrix: Vec<Vec<char>> =
        fs::read_to_string("/home/danny/learning/rust/advent_of_code_2025/input/day7.txt")
            .unwrap()
            .lines()
            .map(|line| line.chars().collect())
            .collect();

    // Obtiene las dimensiones de la matriz.
    let rows = matrix.len();
    let cols = matrix[0].len();

    // Encuentra la columna inicial ('start_col') buscando el carácter 'S' en la primera fila.
    let start_col = matrix[0].iter().position(|&c| c == 'S').unwrap();

    // Crea la tabla de programación dinámica 'dp' del mismo tamaño que la matriz.
    // dp[i][j] almacenará el número de "líneas de tiempo" o caminos posibles para llegar a la celda (i, j).
    // Se usa u128 para poder almacenar números muy grandes.
    let mut dp = vec![vec![0u128; cols]; rows];
    // Establece el caso base: hay 1 camino para estar en la posición inicial.
    dp[0][start_col] = 1;

    // Itera a través de cada fila de la matriz (excepto la última).
    for i in 0..rows - 1 {
        // Itera a través de cada columna en la fila actual.
        for j in 0..cols {
            // Obtiene el número de caminos que llegan a la celda actual (i, j).
            let timelines = dp[i][j];
            // Si no hay caminos que lleguen a esta celda, no se puede continuar desde aquí.
            if timelines == 0 {
                continue;
            }

            // Lógica de transición basada en el carácter de la celda actual.
            // Esto determina cómo se propagan los caminos a la siguiente fila.
            match matrix[i][j] {
                // Si el carácter permite un movimiento vertical...
                '.' | 'S' | '|' => {
                    // Todos los caminos se propagan hacia abajo a la misma columna en la siguiente fila.
                    dp[i + 1][j] += timelines;
                }
                // Si el carácter es una bifurcación...
                '^' => {
                    // Los caminos se propagan en diagonal hacia abajo-izquierda, si no está en el borde.
                    if j > 0 {
                        dp[i + 1][j - 1] += timelines;
                    }
                    // Y también en diagonal hacia abajo-derecha, si no está en el borde.
                    if j + 1 < cols {
                        dp[i + 1][j + 1] += timelines;
                    }
                }
                // Cualquier otro carácter no propaga caminos.
                _ => {}
            }
        }
    }

    // El resultado final es la suma de todos los valores en la última fila de la tabla 'dp'.
    // Esto representa el número total de caminos que lograron llegar a la última fila.
    let result: u128 = dp[rows - 1].iter().sum();
    println!("{}", result);
}
