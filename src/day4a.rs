use itertools::Itertools;
use crate::utils::*;

fn read_text(filename: &str) -> Vec<Vec<char>> {
    read_lines(filename)
        .expect("Failed to read input")
        .map(|l| l.unwrap().to_uppercase().chars().collect())
        .collect()
}

fn get_diagonal_counts(matrix: &Vec<Vec<char>>) -> usize {
    // Vector of (row, col, column difference)
    let mut indices: Vec<(usize, usize, i32)> = vec![];

    const DOWN_LEFT: i32 = 1;
    const DOWN_RIGHT: i32 = -1;

    for row in 0..matrix.len() {
        for col in 0..matrix[row].len() {
            // Check if we have a MAS going down and to the left.
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
            // Check if we have a MAS going down and to the right.
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

    // This is the fun part :)
    /*
        Here's our situation (in ASCII art!)
        M.S
        .A.
        M.S
        For this pattern, we'll have two entries in indices
        Those two are:
        - (0, 0, 1) the down right pattern starting in the top left corner going to the bottom right spelling MAS
        - (0, 2, -1) the down left pattern starting in the top right corner going to the bottom left spelling SAM
        So if we calculate the centers by doing
        (row + 1, col + direction)
        Then the resulting centers would be:
        - (1, 1)
        - (1, 1)
        You may be seeing what we're trying to do now.
        So then we get the counts of each center. For this instance we only have 1 center (1, 1)
        and that center has a count of 2.

        BuT wHaT iF wE hAvE
        M.M
        .A.
        M.S
        We would only get the down right pattern to pass our checks before we add to indices
        So indices would look like
        - (0, 0, 1)
        AND THAT'S IT
        So when we calculate our centers, we get:
        - (1, 1)
        There's only 1 of them so that pattern does not contribute to the final count.

        The first pattern is able to be tiled. That explanation is left as an exercise to the reader.
     */
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
