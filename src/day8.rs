use std::{fs, ops::Sub, time};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Point {
    fn scale_squared(&self) -> i64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

#[derive(Debug)]
struct Connection {
    from: Point,
    to: Point,
    dist: i64,
}

#[derive(Debug)]
struct Puzzle {
    points: Vec<Point>,
}

impl Puzzle {
    fn load(path: &str) -> Puzzle {
        let inputs = fs::read_to_string(path).unwrap();

        let points: Vec<Point> = inputs
            .lines()
            .map(|line| {
                let mut comps = line.split(",").map(|x| x.parse::<i64>().unwrap());

                Point {
                    x: comps.next().unwrap(),
                    y: comps.next().unwrap(),
                    z: comps.next().unwrap(),
                }
            })
            .collect();

        Puzzle { points }
    }
}

fn solve(puzzle: &Puzzle, part1_iterations: usize) {
    let mut connections: Vec<Connection> = puzzle.points[..puzzle.points.len() - 1]
        .iter()
        .enumerate()
        .flat_map(|(i, &from)| {
            puzzle.points[i + 1..].iter().map(move |&to| Connection {
                from,
                to,
                dist: (from - to).scale_squared(),
            })
        })
        .collect();

    connections.sort_by_key(|conn| conn.dist);

    let mut groups: Vec<Vec<Point>> = puzzle.points.iter().map(|&point| vec![point]).collect();

    for (i, conn) in connections.iter().enumerate() {
        let from_index = groups.iter().position(|g| g.contains(&conn.from)).unwrap();
        let mut to_index = groups.iter().position(|g| g.contains(&conn.to)).unwrap();

        if from_index != to_index {
            let mut from_group = groups.remove(from_index);
            if from_index < to_index {
                to_index -= 1;
            }

            groups[to_index].append(&mut from_group);
        }

        if i == part1_iterations {
            let mut group_sizes: Vec<usize> = groups.iter().map(|g| g.len()).collect();
            group_sizes.sort();
            group_sizes.reverse();

            let part1 = group_sizes[0] * group_sizes[1] * group_sizes[2];
            println!("Part 1: {}", part1);
        }
        if groups.len() == 1 {
            println!("Part 2: {}", conn.from.x * conn.to.x);
            break;
        }
    }
}

pub fn run() {
    let start = time::Instant::now();
    let puzzle = Puzzle::load("input/day8.txt");
    solve(&puzzle, 1000);

    let delta = start.elapsed();
    println!("Time: {:.2?}", delta);
}
