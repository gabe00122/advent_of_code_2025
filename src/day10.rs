use microlp::{ComparisonOp, OptimizationDirection, Problem, Variable};
use std::{
    collections::{HashSet, VecDeque},
    fs, time,
};

type BitFlags = u16;

struct Machine {
    start: BitFlags,
    buttons: Vec<BitFlags>,
    target: Vec<u32>,
}

struct Puzzle {
    machines: Vec<Machine>,
}

fn parse_diagram(input: &str) -> BitFlags {
    let mut flags: BitFlags = 0;

    for (i, c) in input[1..input.len() - 1].chars().enumerate() {
        if c == '#' {
            flags |= 1 << i;
        }
    }

    return flags;
}

fn parse_buttons(input: &str) -> BitFlags {
    let mut flags: BitFlags = 0;

    for c in input[1..input.len() - 1].split(',') {
        let i: BitFlags = c.parse().unwrap();
        flags |= 1 << i;
    }

    return flags;
}

impl Puzzle {
    fn load(path: &str) -> Puzzle {
        let text = fs::read_to_string(path).unwrap();

        let problems: Vec<Machine> = text
            .lines()
            .map(|line| {
                let split_line: Vec<&str> = line.split_whitespace().collect();
                if let [first, middle @ .., last] = split_line.as_slice() {
                    let start: BitFlags = parse_diagram(&first);
                    let buttons: Vec<BitFlags> = middle.iter().map(|x| parse_buttons(x)).collect();
                    let target: Vec<u32> = last[1..last.len() - 1]
                        .split(",")
                        .map(|x| x.parse().unwrap())
                        .collect();

                    Machine {
                        start,
                        buttons,
                        target,
                    }
                } else {
                    panic!()
                }
            })
            .collect();

        Puzzle { machines: problems }
    }
}

fn part1(puzzle: &Puzzle) {
    let mut queue = VecDeque::<(u16, BitFlags)>::new();
    let mut seen = HashSet::<u16>::new();

    let result: u16 = puzzle
        .machines
        .iter()
        .filter_map(|p| {
            queue.clear();
            seen.clear();
            queue.push_front((0, p.start));

            while let Some((count, flags)) = queue.pop_back() {
                if flags == 0 {
                    return Some(count);
                }

                let next_count = count + 1;
                for &b in p.buttons.iter() {
                    let next = flags ^ b;

                    if seen.insert(next) {
                        queue.push_front((next_count, next));
                    }
                }
            }

            return None;
        })
        .sum();
    println!("Part 1: {}", result);
}

fn part2(puzzle: &Puzzle) {
    let solutions: f64 = puzzle
        .machines
        .iter()
        .map(|machine| {
            let mut problem = Problem::new(OptimizationDirection::Minimize);

            let vars: Vec<Variable> = machine
                .buttons
                .iter()
                .map(|_| problem.add_integer_var(1.0, (0, i32::MAX)))
                .collect();

            for (i, &t) in machine.target.iter().enumerate() {
                let expr: Vec<(Variable, f64)> = machine
                    .buttons
                    .iter()
                    .zip(vars.iter())
                    .filter(|(b, _)| **b & 1 << i == 1 << i)
                    .map(|(_, v)| (*v, 1.0))
                    .collect();

                problem.add_constraint(&expr, ComparisonOp::Eq, t as f64);
            }

            problem.solve().unwrap().objective()
        })
        .map(|x| x.round())
        .sum();

    println!("Part 2: {}", solutions);
}

pub fn run() {
    println!("--- Day 10 ---");

    let puzzle = Puzzle::load("input/day10.txt");

    let start1 = time::Instant::now();
    part1(&puzzle);
    let delta1 = start1.elapsed();

    println!("{:.2?}", delta1);

    let start2 = time::Instant::now();
    part2(&puzzle);
    let delta2 = start2.elapsed();

    println!("{:.2?}", delta2);
}
