use core::fmt;
use std::{
    cmp::min,
    fs::File,
    io::{BufRead, BufReader},
};

use log::{debug, info};

#[derive(Eq, PartialEq, Clone)]
struct MatrixNumber {
    value: usize,
    start_position: (usize, usize),
    end_position: (usize, usize),
}

impl fmt::Debug for MatrixNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({:?} - {:?})",
            self.value, self.start_position, self.end_position
        )
    }
}

fn is_symbol(c: char) -> bool {
    return !(c.is_numeric() || c == '.');
}

/// Check if there is a sign around the given matrix_number
fn has_sign_around(matrix_number: MatrixNumber, matrix: Vec<Vec<char>>) -> bool {
    debug!("----");
    debug!("Checking {:?}", matrix_number);
    let mut surrounding: Vec<char> = vec![];

    let horizontal_length = matrix[0].len() - 1;

    if matrix_number.start_position.0 > 0 {
        let y = matrix_number.start_position.0.saturating_sub(1);
        let from = matrix_number.start_position.1.saturating_sub(1);
        let to = min(matrix_number.end_position.1 + 1, horizontal_length);

        let above = matrix[y][from..=to].to_vec();

        debug!(
            "Above: {} ({}, {}) - {:?} {}",
            y,
            from,
            to,
            above,
            above.clone().into_iter().any(|el| is_symbol(el))
        );

        surrounding.append(&mut above.clone());
    } else {
        debug!("No above");
    }

    if matrix_number.start_position.0 < matrix.len() - 1 {
        let y = min(matrix_number.start_position.0 + 1, horizontal_length);
        let from = matrix_number.start_position.1.saturating_sub(1);
        let to = min(matrix_number.end_position.1 + 1, horizontal_length);

        let below = matrix[y][from..=to].to_vec();
        debug!(
            "Below: {} ({}, {}) - {:?} {}",
            y,
            from,
            to,
            below,
            below.clone().into_iter().any(|el| is_symbol(el))
        );
        surrounding.append(&mut below.clone());
    } else {
        debug!("No below");
    }

    let x = matrix_number.start_position.0;
    debug!("x: {}", x);
    if matrix_number.start_position.1 > 0 {
        let left = matrix[x][matrix_number.start_position.1.saturating_sub(1)];
        debug!("Left: {}", left);
        surrounding.push(left);
    } else {
        debug!("No left");
    }

    if matrix_number.end_position.1 < horizontal_length {
        let y = min(matrix_number.end_position.1 + 1, horizontal_length);
        let right = matrix[x][y];
        debug!("Right({}, {}): {}", x, y, right);
        surrounding.push(right);
    } else {
        debug!("No right");
    }

    surrounding.into_iter().any(|el| is_symbol(el))
}

fn add_nb_stack(stack: &mut Vec<char>, numbers: &mut Vec<MatrixNumber>, i: usize, j: usize) {
    let value = stack.iter().collect::<String>().parse().unwrap();
    numbers.push(MatrixNumber {
        value,
        start_position: (i, j - (stack.len() - 1)),
        end_position: (i, j),
    })
}

fn extract_part_numbers(matrix: &Vec<Vec<char>>) -> Vec<MatrixNumber> {
    let mut matrix_numbers: Vec<MatrixNumber> = vec![];

    for (i, row) in matrix.iter().enumerate() {
        let mut number_stack: Vec<char> = vec![];

        for (j, col) in row.iter().enumerate() {
            if col.is_numeric() {
                number_stack.push(*col);

                if let Some(next) = row.get(j + 1) {
                    if !next.is_numeric() {
                        add_nb_stack(&mut number_stack, &mut matrix_numbers, i, j);
                        number_stack = vec![];
                    }
                } else {
                    add_nb_stack(&mut number_stack, &mut matrix_numbers, i, j);
                }
            } else {
                // Reset stack
                number_stack = vec![];
            }
        }
        debug!("{:?}", row);
    }

    return matrix_numbers;
}

pub fn run() {
    let filename = "./src/inputs/day_3.txt";
    let file = File::open(filename).expect("Couldn't read file");
    let reader = BufReader::new(file);

    let mut matrix: Vec<Vec<char>> = vec![];

    for line in reader.lines() {
        if let Ok(l) = line {
            matrix.push(l.chars().collect());
        }
    }

    let matrix_numbers: Vec<MatrixNumber> = extract_part_numbers(&matrix);

    debug!("Matrix numbers: {:?}", matrix_numbers);

    let number_with_sign: Vec<MatrixNumber> = matrix_numbers
        .into_iter()
        .filter(|nb| has_sign_around(nb.clone(), matrix.clone()))
        .collect();

    let sum = number_with_sign
        .into_iter()
        .map(|nb| nb.value)
        .sum::<usize>();
    info!("{}", sum);
}
