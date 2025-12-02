use std::fs;

fn read_input() -> Vec<(i64, i64)> {
    let input = fs::read_to_string("input/day2.txt").unwrap();

    input
        .split(",")
        .map(|pair| {
            let (start, end) = pair.split_once("-").unwrap();
            (start.parse().unwrap(), end.parse().unwrap())
        })
        .collect()
}

fn is_invalid_part1(i: i64) -> bool {
    let str_version = i.to_string();
    let length = str_version.len();
    if length % 2 == 0 {
        let mid = length / 2;
        let (left, right) = str_version.split_at(mid);
        left == right
    } else {
        false
    }
}

fn is_invalid_part2(i: i64) -> bool {
    let chars: Vec<char> = i.to_string().chars().collect();
    let length = chars.len();
    (1..=(length / 2))
        .filter(|&chunk_size| length.is_multiple_of(chunk_size))
        .any(|chunk_size| {
            let first = &chars[..chunk_size];
            chars.chunks(chunk_size).all(|item| item == first)
        })
}

fn solve(input: &[(i64, i64)], predicate: impl Fn(i64) -> bool) -> i64 {
    input
        .iter()
        .flat_map(|&(start, finish)| (start..=finish).filter(|&i| predicate(i)))
        .sum()
}

pub fn run() {
    println!("\n--- day2 ---");

    let input = read_input();
    let part1 = solve(&input, &is_invalid_part1);
    println!("part1: {part1}");

    let part2 = solve(&input, &is_invalid_part2);
    println!("part2: {part2}");
}
