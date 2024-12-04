use crate::utils::*;
use itertools::{self, Itertools};
use std::path::Path;

fn num_mistakes(diffs: &Vec<i64>) -> i64 {
    let mut out = 0;
    let mut direction = i64::signum(diffs[0]);

    if direction == 0 {
        out += 1;
    }

    for &diff in diffs {
        let curr_dir = i64::signum(diff);
        if curr_dir != direction {
            out += 1;
            println!("Bad direction");
            if curr_dir != 0 {
                direction = curr_dir;
            }
        } else if !(diff.abs() >= 1 && diff.abs() <= 3) {
            out += 1;
            println!("Out of bounds");
        }
    }

    // let num_wrong_direction = (diffs.len() as i64) - diffs.iter().map(|&d| i64::signum(d)).sum::<i64>().abs();
    // let num_not_in_range = diffs.iter().map(|d| d.abs()).filter(|&d| (d < 1) || (d > 3)).count() as i64;
    // let out = diffs.as_slice().windows(2).filter(|dd| (dd[0] < dd[1])  // num_wrong_direction + num_not_in_range;
    println!("Num Mistakes = {out}");
    // println!("Wrong dir = {num_wrong_direction}");
    // println!("Not in range = {num_not_in_range}");
    println!("Differences = {diffs:?}");
    println!("============");
    out
}

pub fn main() {
    let path = Path::new("data/day2-input.txt");

    if let Ok(lines) = read_lines(&path) {
        let num_safe: usize = lines
            .flatten()
            .map(|line| {
                line.split_whitespace()
                    .map(|n| match n.parse::<i64>() {
                        Ok(num) => num,
                        Err(e) => panic!("Could not parse integer. Error is {e}"),
                    })
                    .collect()
            })
            .map(|line: Vec<i64>| {
                line.as_slice()
                    .windows(2)
                    .map(|window| window[1] - window[0])
                    .collect_vec()
            })
            .filter(|diffs: &Vec<i64>| num_mistakes(diffs) <= 1)
            .count();
        println!("The number of safe lines are {num_safe}");
    }
}
