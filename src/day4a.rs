use itertools::Itertools;
use crate::utils::*;

fn read_text(filename: &str) -> Vec<Vec<char>> {
    read_lines(filename)
        .expect("Failed to read input")
        .map(|l| l.unwrap().to_uppercase().chars().collect())
        .collect()
}

fn get_diagonal_counts(matrix: &Vec<Vec<char>>) -> usize {
    let mut indices: Vec<(usize, usize, i32)> = vec![];

    const DOWN_LEFT: i32 = 1;
    const DOWN_RIGHT: i32 = -1;

    for row in 0..matrix.len() {
        for col in 0..matrix[row].len() {
            if row < matrix.len() - 2 && col < matrix[row].len() - 2 {
                let window = [
                    matrix[row][col],
                    matrix[row + 1][col + DOWN_LEFT as usize],
                    matrix[row + 2][col + 2 * DOWN_LEFT as usize],
                ]
                .iter()
                .collect::<String>();
                if window == "MAS" || window == "SAM" {
                    indices.push((row, col, DOWN_LEFT));
                }
            }
            if row < matrix.len() - 2 && col >= 2 {
                let window = [
                    matrix[row][col],
                    matrix[row + 1][(col as i32 + DOWN_RIGHT) as usize],
                    matrix[row + 2][(col as i32 + 2 * DOWN_RIGHT) as usize],
                ]
                .iter()
                .collect::<String>();
                if window == "MAS" || window == "SAM" {
                    indices.push((row, col, DOWN_RIGHT));
                }
            }
        }
    }

    let count = indices
        .iter()
        .map(|(r, c, d)| ((*r as i32) + 1, (*c as i32) + d))
        .counts()
        .iter()
        .filter(|(_, count)| **count == 2)
        .count();

    count
}

pub fn main() {
    let filename = "data/day4-input.txt";
    let chars = read_text(filename);

    let diagonal_counts = get_diagonal_counts(&chars);

    println!("Diagonal Counts: {}", diagonal_counts);
}
