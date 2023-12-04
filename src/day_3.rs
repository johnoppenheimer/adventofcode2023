use core::fmt;
use std::{
    cmp::min,
    fs::File,
    io::{BufRead, BufReader},
};

use log::{debug, info};

#[derive(Eq, PartialEq, Clone, Debug, Copy)]
struct Position {
    row: usize,
    col: usize,
}

#[derive(Eq, PartialEq, Clone, Copy)]
struct MatrixNumber {
    value: usize,
    start_position: Position,
    end_position: Position,
}

impl MatrixNumber {
    fn overlap(&self, bounds: (Position, Position)) -> bool {
        let (start, end) = bounds;
        return self.start_position.col <= end.col
            && self.end_position.col >= start.col
            && self.start_position.row <= end.row
            && self.end_position.row >= start.row;
    }
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

fn get_surroundings(
    start_position: Position,
    end_position: Position,
    matrix: &Vec<Vec<char>>,
) -> Vec<char> {
    let mut surrounding: Vec<char> = vec![];

    let horizontal_length = matrix[0].len() - 1;

    if start_position.col > 0 {
        let row = start_position.row.saturating_sub(1);
        let from = start_position.col.saturating_sub(1);
        let to = min(end_position.col + 1, horizontal_length);

        let above = matrix[row][from..=to].to_vec();
        surrounding.append(&mut above.clone());
    } else {
        debug!("No above");
    }

    if start_position.col < matrix.len() - 1 {
        let row = min(start_position.row + 1, horizontal_length);
        let from = start_position.col.saturating_sub(1);
        let to = min(end_position.col + 1, horizontal_length);

        let below = matrix[row][from..=to].to_vec();
        surrounding.append(&mut below.clone());
    } else {
        debug!("No below");
    }

    let row = start_position.row;
    debug!("col: {}", row);
    if start_position.row > 0 {
        let left = matrix[row][start_position.row.saturating_sub(1)];
        debug!("Left: {}", left);
        surrounding.push(left);
    } else {
        debug!("No left");
    }

    if end_position.row < horizontal_length {
        let y = min(end_position.row + 1, horizontal_length);
        let right = matrix[row][y];
        debug!("Right({}, {}): {}", row, y, right);
        surrounding.push(right);
    } else {
        debug!("No right");
    }

    return surrounding;
}

/// Check if there is a sign around the given matrix_number
fn has_sign_around(matrix_number: MatrixNumber, matrix: Vec<Vec<char>>) -> bool {
    debug!("----");
    debug!("Checking {:?}", matrix_number);
    let surrounding = get_surroundings(
        matrix_number.start_position,
        matrix_number.end_position,
        &matrix,
    );

    surrounding.into_iter().any(|el| is_symbol(el))
}

fn add_nb_stack(stack: &mut Vec<char>, numbers: &mut Vec<MatrixNumber>, i: usize, j: usize) {
    let value = stack.iter().collect::<String>().parse().unwrap();
    numbers.push(MatrixNumber {
        value,
        start_position: Position {
            row: i,
            col: j - (stack.len() - 1),
        },
        end_position: Position { row: i, col: j },
    })
}

fn has_part_number_around(gear: Position, numbers: Vec<MatrixNumber>) -> usize {
    let top_left = Position {
        row: gear.row.saturating_sub(1),
        col: gear.col.saturating_sub(1),
    };

    let bottom_right = Position {
        row: gear.row + 1,
        col: gear.col + 1,
    };

    let surrounds: Vec<usize> = numbers
        .into_iter()
        .filter_map(|nb| {
            if nb.overlap((top_left, bottom_right)) {
                return Some(nb.value);
            }
            return None;
        })
        .collect();

    debug!(
        "Gear({}, {}) - Surrounds: {:?}",
        gear.col, gear.row, surrounds
    );

    if surrounds.len() < 2 {
        return 0;
    }

    return surrounds.iter().fold(1, |acc, el| acc * el);
}

fn extract_part_numbers(matrix: &Vec<Vec<char>>) -> (Vec<MatrixNumber>, Vec<Position>) {
    let mut matrix_numbers: Vec<MatrixNumber> = vec![];
    let mut gears: Vec<Position> = vec![];

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
                if col.eq(&'*') {
                    gears.push(Position { row: i, col: j });
                }
                // Reset stack
                number_stack = vec![];
            }
        }
        debug!("{:?}", row);
    }

    return (matrix_numbers, gears);
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

    let (matrix_numbers, gears) = extract_part_numbers(&matrix);

    // Part 1
    debug!("Matrix numbers: {:?}", matrix_numbers);

    let number_with_sign: Vec<MatrixNumber> = matrix_numbers
        .clone()
        .into_iter()
        .filter(|nb| has_sign_around(nb.clone(), matrix.clone()))
        .collect();

    let sum = number_with_sign
        .into_iter()
        .map(|nb| nb.value)
        .sum::<usize>();

    // Part 2
    debug!("Gears: {:?}", gears);
    let mut gears_surrounds: Vec<usize> = vec![];
    for gear in gears {
        let nb = has_part_number_around(gear, matrix_numbers.clone());
        gears_surrounds.push(nb);
    }

    info!("Part 1: {}", sum);
    info!("Part 2: {}", gears_surrounds.iter().sum::<usize>());
}
