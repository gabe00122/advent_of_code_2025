use std::{collections::VecDeque, fs};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
    Empty,
    Start,
    Splitter,
    Beam,
}

#[derive(Debug)]
struct Puzzle {
    map: Vec<Tile>,
    possibilities: Vec<i64>,
    width: i64,
    height: i64,
}

impl Puzzle {
    fn load(path: &str) -> Puzzle {
        let input = fs::read_to_string(path).unwrap();

        let map: Vec<Tile> = input
            .chars()
            .flat_map(|c| match c {
                '.' => Some(Tile::Empty),
                '^' => Some(Tile::Splitter),
                'S' => Some(Tile::Start),
                _ => None,
            })
            .collect();

        let width = input.chars().position(|c| c == '\n').unwrap();
        let height = map.len() / width;

        let possibilities: Vec<i64> = map.iter().map(|_| 0).collect();

        Puzzle {
            map,
            possibilities,
            width: width as i64,
            height: height as i64,
        }
    }

    fn in_bounds(&self, x: i64, y: i64) -> bool {
        x >= 0 && y >= 0 && x < self.width && y < self.height
    }

    fn get_index(&self, x: i64, y: i64) -> usize {
        (y * self.width + x) as usize
    }

    fn get(&self, x: i64, y: i64) -> Tile {
        self.map[self.get_index(x, y)]
    }

    fn set_beam(&mut self, queue: &mut VecDeque<(i64, i64)>, x: i64, y: i64, possibilities: i64) {
        if self.in_bounds(x, y) {
            let index = self.get_index(x, y);
            if self.map[index] == Tile::Empty {
                queue.push_back((x, y));
            }

            self.map[index] = Tile::Beam;
            self.possibilities[index] += possibilities;
        }
    }
}

fn sovle(puzzle: &mut Puzzle) -> (i64, i64) {
    let mut part1 = 0;
    let mut queue: VecDeque<(i64, i64)> = VecDeque::new();

    let start_index = puzzle.map.iter().position(|&t| t == Tile::Start).unwrap();
    let start_x = (start_index as i64) % puzzle.width;
    let start_y = (start_index as i64) / puzzle.width;

    puzzle.possibilities[start_index] = 1;
    queue.push_back((start_x, start_y));

    while let Some((x, y)) = queue.pop_front() {
        let y = y + 1;

        if puzzle.in_bounds(x, y) {
            let possibilities = puzzle.possibilities[puzzle.get_index(x, y - 1)];

            if puzzle.get(x, y) == Tile::Splitter {
                part1 += 1;
                puzzle.set_beam(&mut queue, x + 1, y, possibilities);
                puzzle.set_beam(&mut queue, x - 1, y, possibilities);
            } else {
                puzzle.set_beam(&mut queue, x, y, possibilities);
            }
        }
    }

    let part2: i64 = puzzle.possibilities[puzzle.possibilities.len() - puzzle.width as usize..]
        .iter()
        .sum();

    (part1, part2)
}

pub fn run() {
    let mut puzzle = Puzzle::load("input/day7.txt");
    let (part1, part2) = sovle(&mut puzzle);
    println!("part1: {}\npart2: {}", part1, part2);
}
