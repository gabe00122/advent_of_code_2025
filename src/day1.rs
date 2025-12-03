use std::fs;

fn read_input() -> Vec<i32> {
    let input = fs::read_to_string("input/day1.txt").unwrap();
    input
        .lines()
        .map(|line| {
            (if line.starts_with("L") { 1 } else { -1 }) * line[1..].parse::<i32>().unwrap()
        })
        .collect()
}

fn part1(input: &[i32]) -> i32 {
    let mut x = 50;
    let mut hit_zero = 0;

    for &turn in input {
        x = (x + turn).rem_euclid(100);
        if x == 0 {
            hit_zero += 1;
        }
    }

    hit_zero
}

fn part2(input: &[i32]) -> i32 {
    let mut x = 50;
    let mut passed_zero = 0;

    for &turn in input {
        let starts_at_zero = x == 0;

        x += turn;
        if x <= 0 {
            passed_zero += -x / 100 + if starts_at_zero { 0 } else { 1 };
        } else {
            passed_zero += x / 100;
        }

        x = x.rem_euclid(100);
    }

    passed_zero
}

pub fn run() {
    println!("\n--- day1 ---");
    let input = read_input();

    let result1 = part1(&input);
    println!("part1: {result1}");

    let result2 = part2(&input);
    println!("part2: {result2}");
}
