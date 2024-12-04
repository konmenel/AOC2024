use lazy_static::lazy_static;
use regex::Regex;
use std::fs;

fn main() {
    #[cfg(debug_assertions)]
    const INPUT_FILE: &str = "data/examples/03.txt";
    #[cfg(not(debug_assertions))]
    const INPUT_FILE: &str = "data/03.txt";
    let contents = fs::read_to_string(INPUT_FILE).unwrap();
    lazy_static! {
        static ref RE: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
        static ref RE2: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    }
    let sum = RE
        .captures_iter(&contents)
        .map(|m| {
            m.get(1).unwrap().as_str().parse::<i64>().unwrap()
                * m.get(2).unwrap().as_str().parse::<i64>().unwrap()
        })
        .sum::<i64>();
    println!("{sum}");
    
    let mut enabled = true;
    let sum = RE2
        .captures_iter(&contents)
        .map(|m| {
            println!("{:?}", m.get(0).unwrap().as_str());
            if m.get(0).unwrap().as_str() == "do()" {
                enabled = true;
                return 0;
            } else if m.get(0).unwrap().as_str() == "don't()" {
                enabled = false;
                return 0;
            }
            if enabled {
                m.get(1).unwrap().as_str().parse::<i64>().unwrap()
                * m.get(2).unwrap().as_str().parse::<i64>().unwrap()
            } else {
                0
            }
        })
        .sum::<i64>();
    println!("{sum}");
}
