use std::fs;
// 173848577117276

fn main() {
    // Lee todo el contenido del archivo de entrada en un String.
    let input =
        fs::read_to_string("/home/danny/learning/rust/advent_of_code_2025/input/day3.txt").unwrap();

    // Procesa cada línea del input y acumula los resultados usando 'fold'.
    let result = input.lines().fold(0_i64, |acc, line| {
        // Convierte la línea de texto en un vector de dígitos (u8).
        let line: Vec<u8> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();

        let n = line.len(); // Longitud de la línea (número de dígitos).
        
        // Se utiliza programación dinámica (DP) para resolver el problema.
        // 'dp' es la tabla de DP. dp[e][i] almacenará el número más grande que se puede formar
        // usando 'e' elementos (dígitos) de los primeros 'i' dígitos de la línea.
        // Dimensiones: 13 (para 0 a 12 elementos) x n+1 (para 0 a n dígitos de la línea).
        // Se inicializa con i64::MIN para representar estados no alcanzados.
        let mut dp = vec![vec![i64::MIN; n + 1]; 13];

        // Caso base: con 0 elementos y 0 dígitos de la línea, el número formado es 0.
        dp[0][0] = 0;

        // Itera sobre cada dígito de la línea (representado por el índice 'i').
        for i in 0..n {
            // Itera sobre el número de elementos 'e' que estamos intentando formar (de 0 a 11).
            for e in 0..12 {
                // Si el estado dp[e][i] no es alcanzable, no hay nada que hacer.
                if dp[e][i] == i64::MIN {
                    continue;
                }
                
                // Transición 1: No incluir el dígito actual (line[i]).
                // El mejor número con 'e' elementos hasta el índice i+1 es al menos tan bueno
                // como el mejor con 'e' elementos hasta el índice 'i'.
                dp[e][i + 1] = dp[e][i + 1].max(dp[e][i]);

                // Transición 2: Incluir el dígito actual (line[i]).
                // Se forma un nuevo número 'suma' añadiendo el dígito actual al número formado con 'e' elementos.
                let suma = dp[e][i] * 10 + line[i] as i64;
                // Se actualiza el estado para 'e+1' elementos y 'i+1' dígitos considerados,
                // si 'suma' es mayor que el valor actual.
                dp[e + 1][i + 1] = dp[e + 1][i + 1].max(suma);
            }
        }

        // Después de llenar la tabla, el resultado para la línea es el valor máximo
        // en la última fila (dp[12]), que corresponde al mayor número formado con 12 elementos.
        // Se suma este máximo al acumulador 'acc'.
        acc + *dp.last().unwrap().iter().max().unwrap()
    });

    // Imprime el resultado final acumulado.
    println!("{}", result);
}
