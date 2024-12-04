use std::fs;

fn is_safe(row: &Vec<i32>) -> bool {
    if !(row.is_sorted() || row.is_sorted_by(|a, b| a > b)) {
        return false;
    }
    for win in row.windows(2) {
        if (win[1] - win[0]).abs() > 3 || (win[1] - win[0]).abs() < 1 {
            return false;
        }
    }
    true
}

fn is_safe2(row: &Vec<i32>) -> bool {
    for i in 0..row.len() {
        let (n1, n2) = row.split_at(i);
        let mut new_row = Vec::from(n1);
        new_row.extend(&n2[1..]);
        if is_safe(&new_row) {
            return true;
        }
    }
    false
}

fn main() {
    // const INPUT_FILE: &str = "data/examples/02.txt";
    const INPUT_FILE: &str = "data/02.txt";
    let contents = fs::read_to_string(INPUT_FILE).unwrap();
    let safes: i32 = contents
        .lines()
        .map(|line| {
            is_safe(
                &line
                    .split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<i32>>(),
            ) as i32
        })
        .sum();

    println!("{}", safes);

    let safes: i32 = contents
        .lines()
        .map(|line| {
            is_safe2(
                &line
                    .split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<i32>>(),
            ) as i32
        })
        .sum();

    println!("{}", safes);
}
