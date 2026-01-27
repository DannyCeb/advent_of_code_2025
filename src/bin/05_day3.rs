use std::fs;

fn main() {
    // Lee el contenido del archivo de entrada especificado y lo carga en un String.
    // La ruta es absoluta, apuntando directamente al archivo de input.
    // .unwrap() se usa para obtener el resultado de la lectura, pero causará un 'panic' si el archivo no se puede leer.
    let input =
        fs::read_to_string("/home/danny/learning/rust/advent_of_code_2025/input/day3.txt").unwrap();

    // Procesa el input línea por línea, acumulando un resultado.
    // .lines() crea un iterador sobre las líneas del String.
    // .fold() procesa cada línea y acumula un valor, comenzando con 0.
    // 'result' es el acumulador, 'line' es la línea actual.
    let result = input.lines().fold(0_i64, |result, line| {
        println!("line: {:?}", line); // Imprime la línea actual para depuración.

        // Convierte la línea (un String) en un vector de dígitos (u8).
        // .chars() itera sobre los caracteres de la línea.
        // .map() aplica una transformación a cada caracter.
        // c.to_digit(10).unwrap() convierte un caracter como '5' a un número 5.
        let line: Vec<u8> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();

        // Inicializa las variables para encontrar el número más grande de dos dígitos.
        // 'num_mas_grande' se inicializa con el número formado por los dos primeros dígitos de la línea.
        // 'digito_mas_grande' se inicializa con el valor máximo entre los dos primeros dígitos.
        let (mut num_mas_grande, mut digito_mas_grande) =
            (line[0] * 10 + line[1], line[0].max(line[1]));

        // Itera sobre el resto de los dígitos de la línea, a partir del tercero.
        for idx in 2..line.len() {
            // Crea un nuevo número de dos dígitos combinando el dígito más grande encontrado hasta ahora
            // con el dígito actual de la línea.
            let idx_num = digito_mas_grande * 10 + line[idx];
            // Actualiza 'num_mas_grande' si el nuevo número 'idx_num' es mayor que el máximo actual.
            num_mas_grande = num_mas_grande.max(idx_num);
            // Actualiza 'digito_mas_grande' si el dígito actual es mayor que el máximo encontrado hasta ahora.
            digito_mas_grande = digito_mas_grande.max(line[idx]);
        }

        // Suma el 'num_mas_grande' encontrado en esta línea al resultado total acumulado.
        result + num_mas_grande as i64
    });

    // Imprime el resultado final después de procesar todas las líneas.
    println!("{}", result);
}
