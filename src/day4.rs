use crate::utils::*;

fn read_text(filename: &str) -> Vec<Vec<char>> {
    read_lines(filename)
        .expect("Failed to read input")
        .map(|l| l.unwrap().to_uppercase().chars().collect())
        .collect()
}

fn transpose(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    if matrix.len() == 0 {
        return vec![];
    }
    let mut transposed = vec![vec![' '; matrix.len()]; matrix[0].len()];
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            transposed[j][i] = matrix[i][j];
        }
    }
    transposed
}

fn get_horizontal_counts(matrix: &Vec<Vec<char>>) -> usize {
    let forward_counts: usize = matrix
        .iter()
        .map(|line| {
            line.as_slice()
                .windows(4)
                .filter(|&w| w == ['X', 'M', 'A', 'S'])
                .count()
        })
        .sum();
    let backward_counts: usize = matrix
        .iter()
        .map(|line| {
            line.as_slice()
                .windows(4)
                .map(|w| w.iter().rev().collect::<String>())
                .filter(|w| w == "XMAS")
                .count()
        })
        .sum();
    forward_counts + backward_counts
}

fn get_diagonal_counts(matrix: &Vec<Vec<char>>) -> i32 {
    let mut count: i32 = 0;
    for row in 0..matrix.len() {
        for col in 0..matrix[row].len() {
            if row < matrix.len() - 3 && col < matrix[row].len() - 3 {
                let window = [
                    matrix[row][col],
                    matrix[row + 1][col + 1],
                    matrix[row + 2][col + 2],
                    matrix[row + 3][col + 3],
                ]
                    .iter()
                    .collect::<String>();
                if window == "XMAS" || window == "SAMX" {
                    count += 1;
                }
            }
            if row < matrix.len() - 3 && col >= 3 {
                let window = [
                    matrix[row][col],
                    matrix[row + 1][col - 1],
                    matrix[row + 2][col - 2],
                    matrix[row + 3][col - 3],
                ]
                    .iter()
                    .collect::<String>();
                if window == "XMAS" || window == "SAMX" {
                    count += 1;
                }
            }
        }
    }
    count
}

pub fn main() {
    let filename = "data/day4-input.txt";
    let chars = read_text(filename);

    let horizontal_counts = get_horizontal_counts(&chars) as i32;
    let vertical_counts = get_horizontal_counts(&transpose(&chars)) as i32;
    let diagonal_counts = get_diagonal_counts(&chars);
    let total = horizontal_counts + vertical_counts + diagonal_counts;

    println!("Horizontal Counts: {}", horizontal_counts);
    println!("Vertical Counts: {}", vertical_counts);
    println!("Diagonal Counts: {}", diagonal_counts);
    println!("Total: {}", total);
}
