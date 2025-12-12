use std::collections::HashSet;
use std::fs;
use std::time::Instant;

type Shape = Vec<Vec<bool>>;
type Point = (usize, usize);

#[derive(Debug, Clone)]
struct Region {
    width: usize,
    height: usize,
    required: Vec<usize>, // cantidad de cada forma
}

#[derive(Debug, Clone)]
struct Board {
    width: usize,
    height: usize,
    cells: Vec<Vec<Option<usize>>>, // None = vacío, Some(shape_idx) = ocupado por esa forma
}

impl Board {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![vec![None; width]; height],
        }
    }

    fn can_place(&self, shape: &Shape, x: usize, y: usize) -> bool {
        for (dy, row) in shape.iter().enumerate() {
            for (dx, &filled) in row.iter().enumerate() {
                if filled {
                    let nx = x + dx;
                    let ny = y + dy;

                    if nx >= self.width || ny >= self.height {
                        return false;
                    }
                    if self.cells[ny][nx].is_some() {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn place(&mut self, shape: &Shape, x: usize, y: usize, shape_idx: usize) {
        for (dy, row) in shape.iter().enumerate() {
            for (dx, &filled) in row.iter().enumerate() {
                if filled {
                    self.cells[y + dy][x + dx] = Some(shape_idx);
                }
            }
        }
    }

    fn remove(&mut self, shape: &Shape, x: usize, y: usize) {
        for (dy, row) in shape.iter().enumerate() {
            for (dx, &filled) in row.iter().enumerate() {
                if filled {
                    self.cells[y + dy][x + dx] = None;
                }
            }
        }
    }

    fn find_empty_cell(&self) -> Option<Point> {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.cells[y][x].is_none() {
                    return Some((x, y));
                }
            }
        }
        None
    }
}

// Genera todas las rotaciones y reflexiones únicas de una forma
fn generate_variants(shape: &Shape) -> Vec<Shape> {
    let mut variants = Vec::new();
    let mut seen = HashSet::new();

    // Rotaciones: 0°, 90°, 180°, 270°
    let mut current = shape.clone();

    for _ in 0..4 {
        // Guardar rotación actual
        add_variant(&mut variants, &mut seen, &current);

        // Guardar reflexión horizontal
        let flipped_h = flip_horizontal(&current);
        add_variant(&mut variants, &mut seen, &flipped_h);

        // Guardar reflexión vertical
        let flipped_v = flip_vertical(&current);
        add_variant(&mut variants, &mut seen, &flipped_v);

        // Guardar reflexión en ambas direcciones
        let flipped_both = flip_vertical(&flipped_h);
        add_variant(&mut variants, &mut seen, &flipped_both);

        // Rotar para siguiente iteración
        current = rotate90(&current);
    }

    variants
}

fn rotate90(shape: &Shape) -> Shape {
    let h = shape.len();
    let w = shape[0].len();
    let mut rotated = vec![vec![false; h]; w];

    for y in 0..h {
        for x in 0..w {
            rotated[x][h - 1 - y] = shape[y][x];
        }
    }
    rotated
}

fn flip_horizontal(shape: &Shape) -> Shape {
    let h = shape.len();
    let w = shape[0].len();
    let mut flipped = vec![vec![false; w]; h];

    for y in 0..h {
        for x in 0..w {
            flipped[y][w - 1 - x] = shape[y][x];
        }
    }
    flipped
}

fn flip_vertical(shape: &Shape) -> Shape {
    let h = shape.len();
    let w = shape[0].len();
    let mut flipped = vec![vec![false; w]; h];

    for y in 0..h {
        for x in 0..w {
            flipped[h - 1 - y][x] = shape[y][x];
        }
    }
    flipped
}

fn add_variant(variants: &mut Vec<Shape>, seen: &mut HashSet<Shape>, shape: &Shape) {
    if !seen.contains(shape) {
        seen.insert(shape.clone());
        variants.push(shape.clone());
    }
}

fn parse_input(input: &str) -> (Vec<Vec<Shape>>, Vec<Region>) {
    let lines: Vec<&str> = input.lines().collect();
    let mut shapes = vec![Vec::new(); 6]; // Sabemos que hay 6 formas (0-5)
    let mut regions = Vec::new();
    let mut i = 0;

    // Parse shapes (siempre son 6 formas en el input)
    while i < lines.len() {
        let line = lines[i].trim();
        if line.is_empty() {
            i += 1;
            continue;
        }

        // Si encontramos una línea que contiene 'x', hemos terminado con las formas
        if line.contains('x') {
            break;
        }

        // Parsear forma
        if line.ends_with(':') {
            let shape_idx: usize = line[..line.len() - 1].parse().expect("Invalid shape index");

            i += 1;
            let mut shape_lines = Vec::new();
            while i < lines.len()
                && !lines[i].is_empty()
                && !lines[i].contains(':')
                && !lines[i].contains('x')
            {
                shape_lines.push(lines[i]);
                i += 1;
            }

            let mut shape = Vec::new();
            for shape_line in shape_lines {
                let row: Vec<bool> = shape_line.chars().map(|c| c == '#').collect();
                shape.push(row);
            }

            // Generate variants for this shape
            let variants = generate_variants(&shape);
            shapes[shape_idx] = variants;
        } else {
            i += 1;
        }
    }

    // Parse regions
    while i < lines.len() {
        let line = lines[i].trim();
        if line.is_empty() {
            i += 1;
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            i += 1;
            continue;
        }

        // Parse dimensiones - quitar ':' si existe
        let mut dim_part = parts[0];
        if dim_part.ends_with(':') {
            dim_part = &dim_part[..dim_part.len() - 1];
        }

        let dims: Vec<&str> = dim_part.split('x').collect();
        if dims.len() != 2 {
            i += 1;
            continue;
        }

        let width: usize = dims[0]
            .parse()
            .expect(&format!("Invalid width: {}", dims[0]));
        let height: usize = dims[1]
            .parse()
            .expect(&format!("Invalid height: {}", dims[1]));

        // Parse cantidades de formas (deberían ser 6 números)
        let mut required = Vec::new();
        for j in 1..parts.len() {
            required.push(
                parts[j]
                    .parse()
                    .expect(&format!("Invalid count: {}", parts[j])),
            );
        }

        // Asegurarse de que tenemos exactamente 6 valores
        while required.len() < 6 {
            required.push(0);
        }

        regions.push(Region {
            width,
            height,
            required,
        });
        i += 1;
    }

    (shapes, regions)
}

// Algoritmo mejorado con pruning y ordenamiento inteligente
fn can_fit_region(shapes: &Vec<Vec<Shape>>, region: &Region) -> bool {
    let total_blocks: usize = region
        .required
        .iter()
        .enumerate()
        .map(|(idx, &count)| {
            if count == 0 {
                return 0;
            }
            let shape_area = shapes[idx][0]
                .iter()
                .map(|row| row.iter().filter(|&&c| c).count())
                .sum::<usize>();
            shape_area * count
        })
        .sum();

    // Pruning rápido: si el área total de bloques es mayor que el área del tablero, imposible
    if total_blocks > region.width * region.height {
        return false;
    }

    // Crear lista de todas las piezas a colocar
    let mut pieces = Vec::new();
    for (shape_idx, &count) in region.required.iter().enumerate() {
        for _ in 0..count {
            pieces.push(shape_idx);
        }
    }

    if pieces.is_empty() {
        return true;
    }

    // Ordenar piezas por tamaño (mayor primero) - heurística común
    pieces.sort_by_key(|&shape_idx| {
        let shape = &shapes[shape_idx][0];
        std::cmp::Reverse(shape.len() * shape[0].len())
    });

    let board = Board::new(region.width, region.height);

    // Intentar con diferentes órdenes de variantes
    for attempt in 0..3 {
        let mut board_clone = board.clone();
        if backtrack_improved(&mut board_clone, shapes, &pieces, 0, attempt) {
            return true;
        }
    }

    false
}

fn backtrack_improved(
    board: &mut Board,
    shapes: &Vec<Vec<Shape>>,
    pieces: &[usize],
    piece_idx: usize,
    attempt: usize,
) -> bool {
    if piece_idx == pieces.len() {
        return true;
    }

    let shape_idx = pieces[piece_idx];
    let variants = &shapes[shape_idx];

    // Elegir posición para colocar (heurística: primera celda vacía)
    let (start_x, start_y) = match board.find_empty_cell() {
        Some(pos) => pos,
        None => return false,
    };

    // Probar diferentes variantes de la forma
    for variant_idx in 0..variants.len() {
        let shape = &variants[variant_idx];
        let h = shape.len();
        let w = shape[0].len();

        // Intentar colocar en todas las posiciones posibles
        for y in start_y..board.height {
            for x in if y == start_y { start_x } else { 0 }..board.width {
                if x + w <= board.width && y + h <= board.height {
                    if board.can_place(shape, x, y) {
                        board.place(shape, x, y, shape_idx);

                        if backtrack_improved(board, shapes, pieces, piece_idx + 1, attempt) {
                            return true;
                        }

                        board.remove(shape, x, y);
                    }
                }
            }
        }
    }

    false
}

fn solve(input: &str) -> usize {
    let start = Instant::now();
    let (shapes, regions) = parse_input(input);
    println!(
        "Parsed {} shapes and {} regions in {:?}",
        shapes.len(),
        regions.len(),
        start.elapsed()
    );

    // Verificar que tenemos todas las formas
    for (i, shape_variants) in shapes.iter().enumerate() {
        println!(
            "Shape {} has {} variants, size: {}x{}",
            i,
            shape_variants.len(),
            shape_variants[0].len(),
            shape_variants[0][0].len()
        );
    }

    let mut count = 0;
    let total_regions = regions.len();

    for (i, region) in regions.iter().enumerate() {
        if i % 50 == 0 {
            println!("Processing region {}/{}...", i, total_regions);
        }

        if can_fit_region(&shapes, region) {
            count += 1;
            println!("Region {} fits! Total so far: {}", i, count);
        }
    }

    println!("Total time: {:?}", start.elapsed());
    count
}

fn main() {
    let input = fs::read_to_string("/home/danny/learning/rust/advent_of_code_2025/input/day12.txt")
        .expect("Failed to read input.txt");

    let result = solve(&input);
    println!("Number of regions that fit: {}", result);
}
