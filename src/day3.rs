use regex::Regex;

use crate::utils::read_lines;

pub fn main() {
    let input_path = "data/day3-input.txt";
    let haystack = String::from_iter(read_lines(input_path).unwrap().map(|l| l.unwrap()));

    let re = Regex::new(r"mul\((?P<one>\d+),(?P<two>\d+)\)").unwrap();

    // let dates: Vec<(&str, &str, &str)> = re.captures_iter(hay).map(|caps| {
    //     let (_, [year, month, day]) = caps.extract();
    //     (year, month, day)
    // }).collect();
    let total = re
        .captures_iter(&haystack)
        .map(|c| c.extract().1)
        .map(|c: [&str; 2]| (c[0], c[1]))
        .map(|(o, t)| o.parse::<i32>().unwrap() * t.parse::<i32>().unwrap())
        .sum::<i32>();
    println!("Total Sum of Products: {total}");
}
