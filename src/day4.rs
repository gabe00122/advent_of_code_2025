use itertools::iproduct;
use std::fs;

#[derive(Debug)]
struct Puzzle {
    map: Vec<bool>,
    width: i32,
    height: i32,
}

impl Puzzle {
    fn read_input(file: &str) -> Self {
        let input_text = fs::read_to_string(file).unwrap();

        let map: Vec<bool> = input_text
            .chars()
            .filter(|&c| c  != '\n')
            .map(|c| match c {
                '@' => true,
                _ => false,
            })
            .collect();

        let width = input_text.chars().position(|x| x == '\n').unwrap();
        let height = map.len() / width;

        Self { map, width: width as i32, height: height as i32 }
    }

    fn get_at(&self, x: i32, y: i32) -> bool {
        x >= 0
            && y >= 0
            && x < self.width
            && y < self.height
            && self.map[(y * self.width + x) as usize]
    }

    fn set_at(&mut self, x: i32, y: i32, value: bool) {
        self.map[(y * self.width + x) as usize] = value
    }

    fn iter_removable(&self) -> impl Iterator<Item=(i32, i32)> {
        iproduct!(0..self.width, 0..self.height)
            .filter(|&(x, y)| self.get_at(x, y))
            .map(|(x, y)| {
                let count = iproduct!(-1..=1, -1..=1)
                    .filter(|&(ox, oy)| self.get_at(x + ox, y + oy))
                    .count();

                (x, y, count)
            })
            .filter(|&(_, _, count)| count <= 4)
            .map(|(x, y, _)| (x, y))
    }
}

fn part2(puzzle: &mut Puzzle) -> usize {
    let mut total = 0;

    loop {
        let removable: Vec<_> = puzzle.iter_removable().collect();

        if removable.is_empty() {
            break;
        }
        total += removable.len();

        for (x, y) in removable {
            puzzle.set_at(x, y, false);
        }
    }

    total
}

pub fn run() {
    println!("\n--- day4 ---");
    let mut puzzle = Puzzle::read_input("input/day4.txt");

    let result1 = puzzle.iter_removable().count();
    println!("part1: {}", result1);

    let result2 = part2(&mut puzzle);
    println!("part2: {}", result2);
}