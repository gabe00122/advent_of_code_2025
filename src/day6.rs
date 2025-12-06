use std::fs;
use std::iter::zip;

use itertools::{Itertools, iproduct};

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Mul,
}

impl Op {
    fn apply(&self, left: i64, right: i64) -> i64 {
        match self {
            Op::Add => left + right,
            Op::Mul => left * right,
        }
    }
}

#[derive(Debug)]
struct Puzzle {
    nums: Vec<i64>,
    rotated_nums: Vec<Vec<i64>>,
    ops: Vec<Op>,
    width: usize,
    height: usize,
}

impl Puzzle {
    fn load(path: &str) -> Self {
        let input = fs::read_to_string(path).unwrap();

        let lines: Vec<&str> = input.lines().collect();
        let nums: Vec<i64> = lines[..lines.len() - 1]
            .iter()
            .flat_map(|&line| line.split_whitespace())
            .map(|num| num.parse().unwrap())
            .collect();

        let height = lines.len() - 1;
        let width = nums.len() / height;

        let ops: Vec<Op> = lines
            .last()
            .unwrap()
            .split_whitespace()
            .map(|s| match s {
                "*" => Op::Mul,
                "+" => Op::Add,
                _ => Op::Add,
            })
            .collect();

        // parsing for part2
        let chars: Vec<char> = lines[..lines.len() - 1]
            .iter()
            .flat_map(|&line| line.chars())
            .collect();

        let rotated_chars: Vec<char> = rotate_counter_clockwise(&chars, lines[0].len(), height);
        let rotated_lines: Vec<String> = rotated_chars
            .chunks(height)
            .map(|line| line.iter().collect())
            .collect();

        let mut rotated_nums: Vec<Vec<i64>> = Vec::new();
        for (_, chunk) in &rotated_lines
            .iter()
            .map(|line| line.trim().parse::<i64>().ok())
            .chunk_by(|value| value.is_some())
        {
            let line_nums: Vec<i64> = chunk.filter_map(|value| value).collect();
            if !line_nums.is_empty() {
                rotated_nums.push(line_nums);
            }
        }
        rotated_nums.reverse(); // So that we align with ops

        Self {
            nums,
            rotated_nums,
            ops,
            width,
            height,
        }
    }
}

fn rotate_counter_clockwise<T: Clone + Copy>(data: &[T], width: usize, height: usize) -> Vec<T> {
    iproduct!((0..width).rev(), 0..height)
        .map(|(x, y)| data[y * width + x])
        .collect()
}

fn part1(puzzle: &Puzzle) -> i64 {
    let mut output = 0;
    for x in 0..puzzle.width {
        let op = puzzle.ops[x];
        let mut problem = puzzle.nums[x];
        for y in 1..puzzle.height {
            let num = puzzle.nums[x + y * puzzle.width];
            problem = op.apply(problem, num);
        }

        output += problem;
    }

    output
}

fn part2(puzzle: &Puzzle) -> i64 {
    zip(&puzzle.ops, &puzzle.rotated_nums)
        .map(|(op, nums)| {
            nums.iter()
                .copied()
                .reduce(|acc, x| op.apply(acc, x))
                .unwrap()
        })
        .sum()
}

pub fn run() {
    println!("\n--- Day 6 ---");

    let input = Puzzle::load("input/day6.txt");

    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
