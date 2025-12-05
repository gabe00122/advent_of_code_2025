use std::fs;

fn read_input() -> Vec<Vec<u64>> {
    let input = fs::read_to_string("input/day3.txt").unwrap();

    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect::<Vec<u64>>()
        })
        .collect()
}

fn left_max_index(data: &[u64]) -> usize {
    let max_value = *data.iter().max().unwrap();
    data.iter().position(|&x| x == max_value).unwrap()
}

fn bank_power(line: &Vec<u64>, count: usize) -> u64 {
    let mut out = 0;
    let mut idx = 0;

    for i in 0..count {
        let reserve = line.len() - count + i + 1;
        idx += left_max_index(&line[idx..reserve]);
        out = out * 10 + line[idx];

        idx += 1
    }
    out
}

fn solve(input: &Vec<Vec<u64>>, count: usize) -> u64 {
    input.iter().map(|x| bank_power(x, count)).sum()
}

pub fn run() {
    println!("\n--- day3 ---");
    let input = read_input();

    let result1 = solve(&input, 2);
    println!("part1: {}", result1);

    let result2 = solve(&input, 12);
    println!("part2: {}", result2);
}
