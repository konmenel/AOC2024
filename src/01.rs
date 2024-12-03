use std::fs;

fn main() {
    // const INPUT_FILE: &str = "data/examples/01.txt";
    const INPUT_FILE: &str = "data/01.txt";
    let contents = fs::read_to_string(INPUT_FILE).unwrap();

    let mut v1 = contents.lines().map(|line| line.split_whitespace().nth(0).unwrap().parse::<i32>().unwrap()).collect::<Vec<_>>();
    let mut v2 = contents.lines().map(|line| line.split_whitespace().nth(1).unwrap().parse::<i32>().unwrap()).collect::<Vec<_>>();

    v1.sort();
    v2.sort();

    let diff = v1.iter().zip(v2.iter()).map(|x| (x.0 - x.1).abs()).collect::<Vec<_>>();

    println!("{}", diff.iter().sum::<i32>());

    // part 2
    println!("{}", v1.iter().map(|i| v2.iter().filter(|&j| j == i).sum::<i32>()).sum::<i32>());
}