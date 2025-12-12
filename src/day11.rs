use std::{collections::HashMap, fs};

#[derive(Debug)]
struct Puzzle {
    codes: HashMap<String, usize>,
    connections: Vec<Vec<usize>>,
}

impl Puzzle {
    fn load(path: &str) -> Puzzle {
        let text = fs::read_to_string(path).unwrap();

        let mut codes: HashMap<String, usize> = [("out".to_string(), 0)].into_iter().collect();
        let mut connections: Vec<Vec<usize>> = vec![Vec::new()];

        for (i, line) in text.lines().enumerate() {
            codes.entry(line[..3].to_string()).insert_entry(i + 1);
            connections.push(Vec::new());
        }

        for line in text.lines() {
            let conn: Vec<usize> = line
                .split_whitespace()
                .flat_map(|x| codes.get(&x[..3]))
                .copied()
                .collect();

            if let [first, rest @ ..] = conn.as_slice() {
                connections[*first].extend(rest);
            }
        }

        Puzzle { codes, connections }
    }
}

fn count_paths(
    connections: &Vec<Vec<usize>>,
    from: usize,
    to: usize,
    fft_code: usize,
    dac_code: usize,
    fft_hit: bool,
    dac_hit: bool,
    memory: &mut HashMap<(usize, bool, bool), usize>,
) -> usize {
    let fft_hit = fft_hit || from == fft_code;
    let dac_hit = dac_hit || from == dac_code;

    if from == to {
        if fft_hit && dac_hit { 1 } else { 0 }
    } else {
        let key = (from, fft_hit, dac_hit);
        if let Some(&value) = memory.get(&key) {
            value
        } else {
            let calc = connections[from]
                .iter()
                .map(|&conn| {
                    count_paths(
                        connections,
                        conn,
                        to,
                        fft_code,
                        dac_code,
                        fft_hit,
                        dac_hit,
                        memory,
                    )
                })
                .sum();
            memory.entry(key).insert_entry(calc);
            calc
        }
    }
}

fn solve(puzzle: &Puzzle, from: &str, part1: bool) -> usize {
    let mut memory = HashMap::<(usize, bool, bool), usize>::new();

    let from = *puzzle.codes.get(from).unwrap();
    let to = *puzzle.codes.get(&"out".to_string()).unwrap();
    let fft_code = *puzzle.codes.get(&"fft".to_string()).unwrap();
    let dac_code = *puzzle.codes.get(&"dac".to_string()).unwrap();

    count_paths(
        &puzzle.connections,
        from,
        to,
        fft_code,
        dac_code,
        part1,
        part1,
        &mut memory,
    )
}

pub fn run() {
    let puzzle = Puzzle::load("input/day11.txt");

    println!("Part 1: {}", solve(&puzzle, "you", true));
    println!("Part 2: {}", solve(&puzzle, "svr", false));
}
