use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;

// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


pub fn main() {
    let path = Path::new("data/day1-input.txt");

    if let Ok(lines) = read_lines(&path) {
        let both_lr: Vec<(i64, i64)> = lines.flatten().map(|line| {line.split_whitespace().map(|n| { match n.parse::<i64>() {
            Ok(num) => num,
            Err(e) => panic!("cannot parse integer. ERror is {e}"),
        }}).collect()}).map(|v: Vec<i64>| (v[0], v[1])).collect();
        let mut left_list: Vec<i64> = both_lr.iter().map(|(l, _)| *l).collect(); 
        let mut right_list: Vec<i64> = both_lr.iter().map(|(_, r)| *r).collect();

        left_list.sort();
        right_list.sort();
        let total: i64 = left_list.iter().zip(right_list.iter()).map(|(l, r)| (l - r).abs()).sum();
        println!("The total found was {total}");
    }
}
