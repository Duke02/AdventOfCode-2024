use regex::Regex;

use crate::utils::read_lines;

pub fn main() {
    let input_path = "data/day3-input.txt";
    let haystack = String::from_iter(read_lines(input_path).unwrap().map(|l| l.unwrap()));

    let re =
        Regex::new(r"mul\((?P<one>\d+),(?P<two>\d+)\)|(?P<enable>do\(\))|(?P<disable>don't\(\))")
            .unwrap();

    // let dates: Vec<(&str, &str, &str)> = re.captures_iter(hay).map(|caps| {
    //     let (_, [year, month, day]) = caps.extract();
    //     (year, month, day)
    // }).collect();
    let mut enabled_multipliers: Vec<(i32, i32)> = vec![];
    let mut enabled: bool = true;

    for m in re.find_iter(&haystack).map(|m| m.as_str()) {
        if &m[..2] == "do" {
            enabled = m.len() == 4;
        } else {
            // We're dealing with a multiplier.
            if enabled {
                // mul(###,###)
                let ot = re.captures(&m).expect("Was not able to get a match from the match.");
                let one = ot["one"].parse::<i32>().unwrap();
                let two = ot["two"].parse::<i32>().unwrap();
                enabled_multipliers.push((one, two));
            }
        }
        // 5. After we processed all of the matches, we sum the products of the enabled multipliers.
    }

    let total = enabled_multipliers.iter().map(|(o, t)| o * t).sum::<i32>();
    // let total = re
    //     .captures_iter(&haystack)
    //     .map(|c| c.extract().1)
    //     .map(|c: [&str; 2]| (c[0], c[1]))
    //     .map(|(o, t)| o.parse::<i32>().unwrap() * t.parse::<i32>().unwrap())
    //     .sum::<i32>();
    println!("Total Sum of Products: {total}");
}
