use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;

// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn get_count(val: usize, l: &Vec<usize>) -> usize {
    l.iter().filter(|item| **item == val).count()
}


pub fn main() {
    let path = Path::new("data/day1-input.txt");

    if let Ok(lines) = read_lines(&path) {
        let both_lr: Vec<(usize, usize)> = lines.flatten().map(|line| {line.split_whitespace().map(|n| { match n.parse::<usize>() {
            Ok(num) => num,
            Err(e) => panic!("cannot parse integer. ERror is {e}"),
        }}).collect()}).map(|v: Vec<usize>| (v[0], v[1])).collect();
        let left_list: Vec<usize> = both_lr.iter().map(|(l, _)| *l).collect(); //.sort();
        let right_list: Vec<usize> = both_lr.iter().map(|(_, r)| *r).collect(); //.sort();
        let similarity: usize = left_list.iter().map(|l| *l * get_count(*l, &right_list)).sum();
        println!("Similarity score is {similarity}");
    }
}
