use std::{fs, iter::zip};


#[derive(Debug)]
struct Shape {
    area: usize,
}

impl Shape {
    fn from_string(input: &str) -> Shape {
        // remove the number and colon
        let (_, input) = input.split_once("\n").unwrap();
        let area: usize = input.chars().filter(|&c| c == '#').count();
        Shape { area }
    }
}

#[derive(Debug)]
struct Space {
    width: usize,
    height: usize,
    counts: Vec<usize>,
}

impl Space {
    fn from_string(input: &str) -> Space {
        let (dims, counts) = input.split_once(": ").unwrap();
        let (width, height) = dims.split_once("x").unwrap();

        let counts: Vec<usize> = counts.split_whitespace().map(|c| c.parse().unwrap()).collect();
        let width: usize = width.parse().unwrap();
        let height: usize = height.parse().unwrap();

        Space { width, height, counts }
    }

    fn can_fit(&self, shapes: &[Shape]) -> bool {
        let slots = (self.width / 3) * (self.height / 3);
        let required_slots: usize = self.counts.iter().sum();

        if required_slots > slots {
            return false;
        }

        let tiles = self.width * self.height;
        let required_tile: usize = zip(&self.counts, shapes).map(|(c, s)| c * s.area).sum();
        if required_tile > tiles {
            return false;
        }

        return true;
    }
}

struct Puzzle {
    shapes: Vec<Shape>,
    spaces: Vec<Space>,
}

impl Puzzle {
    fn load(path: &str) -> Puzzle {
        let input = fs::read_to_string(path).unwrap();

        let test: Vec<&str> = input.split("\n\n").collect();

        let (&last, rest) = test.split_last().unwrap();

        let shapes: Vec<Shape> = rest.iter().map(|&s| Shape::from_string(s)).collect();
        let spaces: Vec<Space> = last.lines().map(Space::from_string).collect();
        
        Puzzle { shapes, spaces }
    }

    fn part1(&self) -> usize {
        self.spaces.iter().filter(|&space| space.can_fit(&self.shapes)).count()
    }
}

pub fn run() {
    let puzzle = Puzzle::load("input/day12.txt");

    println!("Part 1: {}", puzzle.part1());
}
