use itertools::{self, Itertools};
use std::path::Path;
use crate::utils::*;


pub fn main() {
   let path = Path::new("data/day2-input.txt");

   if let Ok(lines) = read_lines(&path) {
        let num_safe: usize = lines
            .flatten()
            .map(|line| {
                line.split_whitespace()
                    .map(|n| { 
                        match n.parse::<i64>() {
                            Ok(num) => num,
                            Err(e) => panic!("Could not parse integer. Error is {e}"),
                        }
                    })
                .collect()
            })
            .map(|line: Vec<i64>| {
                line.as_slice()
                    .windows(2)
                    .map(|window| window[1] - window[0])
                    .collect_vec()
            })
            .filter(|diffs: &Vec<i64>| {
                (diffs.iter().all(|d| *d > 0) || diffs.iter().all(|d| *d < 0)) &&
                    (diffs.iter().map(|d| d.abs()).fold(true, |acc, next| acc && (next >= 1) && (next <= 3)))
            }).count();
//        println!("Safe lines are {num_safe:?}");
        println!("The number of safe lines are {num_safe}");
    }
    
}










