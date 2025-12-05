use std::fs;

#[derive(Debug)]
struct Puzzle {
    valid_ranges: Vec<(u64, u64)>,
    ids: Vec<u64>,
}

impl Puzzle {
    fn load(file_path: &str) -> Self {
        let text = fs::read_to_string(file_path).unwrap();

        let mut valid_ranges = Vec::new();
        let mut ids = Vec::new();

        for line in text.lines() {
            if line.is_empty() {
                continue;
            }

            if let Some((left, right)) = line.split_once("-") {
                let left: u64 = left.parse().unwrap();
                let right: u64 = right.parse().unwrap();
                valid_ranges.push((left, right));
            } else {
                let line: u64 = line.parse().unwrap();
                ids.push(line);
            }
        }

        Self { valid_ranges, ids }
    }
}

fn part1(puzzle: &Puzzle) -> usize {
    let mut output = 0;
    let mut ids = puzzle.ids.clone();
    ids.sort();

    for &(start_id, stop_id) in puzzle.valid_ranges.iter() {
        let start_index = ids.partition_point(|&x| x < start_id);
        let stop_index = ids.partition_point(|&x| x <= stop_id);

        if start_index == stop_index {
            continue;
        }

        output += stop_index - start_index;
        ids.drain(start_index..stop_index);

        if ids.is_empty() {
            break;
        }
    }

    output
}

fn part2(puzzle: &Puzzle) -> u64 {
    let mut ranges: Vec<(u64, i32)> = Vec::new();

    for &(start_id, stop_id) in puzzle.valid_ranges.iter() {
        ranges.push((start_id, 1));
        ranges.push((stop_id + 1, -1)); // convert to exclusive range
    }

    ranges.sort_by_key(|&(id, _)| id); // start will sort before stop because it has -1 even if they land on the same id

    let mut last_id = 0;
    let mut nesting_counter = 0;
    let mut output = 0;

    for &(id, t) in ranges.iter() {
        if nesting_counter == 0 {
            last_id = id;
        }

        nesting_counter += t;

        if nesting_counter == 0 {
            output += id - last_id;
        }
    }

    output
}

pub fn run() {
    println!("\n--- Day 5 ---");
    let puzzle = Puzzle::load("input/day5.txt");

    let now1 = std::time::Instant::now();
    let result1 = part1(&puzzle);
    let elapsed1 = now1.elapsed();
    println!("part1: {}", result1);
    println!("part1 took: {:.2?}", elapsed1);

    let now2 = std::time::Instant::now();
    println!("part2: {}", part2(&puzzle));
    let elapsed2 = now2.elapsed();
    println!("part2 took: {:.2?}", elapsed2);
}
