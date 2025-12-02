use std::fs;

fn read_input() -> Vec<(i64, i64)> {
    let input = fs::read_to_string("./input/day2.txt").unwrap();

    input
        .split(",")
        .map(|pair| {
            let mut it = pair.split("-").map(|s| s.parse::<i64>().unwrap());
            (it.next().unwrap(), it.next().unwrap())
        })
        .collect()
}

fn is_invalid(i: i64) -> bool {
    let str_version = format!("{}", i);
    let length = str_version.len();
    if length % 2 == 0 {
        let mid = length / 2;
        let (left, right) = str_version.split_at(mid);
        left == right
    } else {
        false
    }
}

fn sum_invalid(start: i64, finish: i64) -> i64 {
    (start..=finish).filter(|i| is_invalid(*i)).sum()
}

fn part1(input: &[(i64, i64)]) {
    let result: i64 = input
        .iter()
        .map(|(start, finish)| sum_invalid(*start, *finish))
        .sum();

    println!("Part1: {}", result);
}

fn is_invalid_part2(i: i64) -> bool {
    let str_version: Vec<char> = format!("{}", i).chars().collect();
    let length = str_version.len();
    for chunk_size in 1..length {
        if length % chunk_size == 0 {
            let mut chunk_iter = str_version.chunks(chunk_size);
            let first = chunk_iter.next().unwrap();
            if chunk_iter.all(|item| item == first) {
                return true;
            }
        }
    }
    false
}

fn sum_invalid_part2(start: i64, finish: i64) -> i64 {
    (start..=finish).filter(|i| is_invalid_part2(*i)).sum()
}

fn part2(input: &[(i64, i64)]) {
    let result: i64 = input
        .iter()
        .map(|(start, finish)| sum_invalid_part2(*start, *finish))
        .sum();

    println!("Part2: {}", result);
}

fn main() {
    let input = read_input();
    part1(&input);
    part2(&input);
}
