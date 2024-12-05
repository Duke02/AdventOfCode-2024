use regex::Regex;

use crate::utils::read_lines;

pub fn main() {
    let input_path = "data/day3-input.txt";
    let haystack = String::from_iter(read_lines(input_path).unwrap().map(|l| l.unwrap()));

    let re =
        Regex::new(r"mul\((?P<one>\d+),(?P<two>\d+)\)|(?P<enable>do\(\))|(?P<disable>don't\(\))")
            .unwrap();

    let mut enabled_multipliers: Vec<(i32, i32)> = vec![];
    let mut enabled: bool = true;

    for m in re.find_iter(&haystack).map(|m| m.as_str()) {
        if &m[..2] == "do" {
            enabled = m.len() == 4; // do()'s length is 4 so enable future mul calls,
                                    // don't() is not 4 characters long so disable future mul calls.
        } else {
            // We're dealing with a multiplier.
            if enabled {
                let ot = re
                    .captures(&m)
                    .expect("Was not able to get a match from the match.");
                let one = ot["one"].parse::<i32>().unwrap();
                let two = ot["two"].parse::<i32>().unwrap();
                enabled_multipliers.push((one, two));
            }
        }
    }

    let total = enabled_multipliers.iter().map(|(o, t)| o * t).sum::<i32>();
    println!("Total Sum of Products: {total}");
}
