use std::{fs, time};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Line {
    a: Point,
    b: Point,
}

impl Line {
    fn new(a: Point, b: Point) -> Self {
        Line { a, b }
    }

    fn is_horizontal(&self) -> bool {
        self.a.y == self.b.y
    }

    fn is_vertical(&self) -> bool {
        self.a.x == self.b.x
    }

    fn cross(&self, other: &Self) -> bool {
        // checks for '+' shaped intersections
        let (h_seg, v_seg) = if self.is_horizontal() {
            (self, other)
        } else if other.is_horizontal() {
            (other, self)
        } else {
            return false;
        };

        let v_min_y = std::cmp::min(v_seg.a.y, v_seg.b.y);
        let v_max_y = std::cmp::max(v_seg.a.y, v_seg.b.y);
        let v_x = v_seg.a.x;

        let h_min_x = std::cmp::min(h_seg.a.x, h_seg.b.x);
        let h_max_x = std::cmp::max(h_seg.a.x, h_seg.b.x);
        let h_y = h_seg.a.y;

        return h_min_x < v_x && v_x < h_max_x && v_min_y < h_y && h_y < v_max_y;
    }
}

struct Rect {
    a: Point,
    b: Point,
    corners: [Point; 4],
    borders: [Line; 4],
}

impl Rect {
    fn new(a: Point, b: Point) -> Self {
        let min_x = std::cmp::min(a.x, b.x);
        let max_x = std::cmp::max(a.x, b.x);
        let min_y = std::cmp::min(a.y, b.y);
        let max_y = std::cmp::max(a.y, b.y);

        let corners = [
            Point::new(min_x, min_y),
            Point::new(max_x, min_y),
            Point::new(max_x, max_y),
            Point::new(min_x, max_y),
        ];

        let borders = [
            Line::new(corners[0], corners[1]),
            Line::new(corners[1], corners[2]),
            Line::new(corners[2], corners[3]),
            Line::new(corners[3], corners[0]),
        ];

        Self {
            a,
            b,
            corners,
            borders,
        }
    }

    fn area(&self) -> i64 {
        ((self.a.x - self.b.x).abs() + 1) * ((self.a.y - self.b.y).abs() + 1)
    }

    fn cross(&self, line: &Line) -> bool {
        self.borders.iter().any(|b| b.cross(line))
    }
}

#[derive(Debug)]
struct Puzzle {
    points: Vec<Point>,
    polygon: Vec<Line>,
}

impl Puzzle {
    fn load(path: &str) -> Self {
        let text = fs::read_to_string(path).unwrap();

        let points: Vec<Point> = text
            .lines()
            .map(|line| {
                let (x, y) = line.split_once(",").unwrap();

                Point {
                    x: x.parse().unwrap(),
                    y: y.parse().unwrap(),
                }
            })
            .collect();

        let mut polygon: Vec<Line> = points[..points.len() - 1]
            .iter()
            .zip(points[1..].iter())
            .map(|(&a, &b)| Line::new(a, b))
            .collect();
        polygon.push(Line::new(*points.last().unwrap(), *points.first().unwrap()));

        return Puzzle { points, polygon };
    }

    fn point_in_polygon(&self, point: &Point) -> bool {
        for line in self.polygon.iter() {
            let min_x = std::cmp::min(line.a.x, line.b.x);
            let max_x = std::cmp::max(line.a.x, line.b.x);
            let min_y = std::cmp::min(line.a.y, line.b.y);
            let max_y = std::cmp::max(line.a.y, line.b.y);

            if (line.is_horizontal() && point.y == line.a.y && min_x <= point.x && point.x <= max_x)
                || (line.is_vertical()
                    && point.x == line.a.x
                    && min_y <= point.y
                    && point.y <= max_y)
            {
                return true; // we are laying directly on a line
            }
        }

        let count = self
            .polygon
            .iter()
            .filter(|&line| {
                let min_y = std::cmp::min(line.a.y, line.b.y);
                let max_y = std::cmp::max(line.a.y, line.b.y);
                //ray extends to the left
                line.a.x < point.x && min_y <= point.y && point.y < max_y
            })
            .count();

        // the point is within the body of the polygon
        !count.is_multiple_of(2)
    }

    fn rect_in_polygon(&self, rect: &Rect) -> bool {
        rect.corners.iter().all(|c| self.point_in_polygon(c))
            && !self.polygon.iter().any(|line| rect.cross(line))
    }
}

fn part1(puzzle: &Puzzle) {
    let dist = puzzle.points[..puzzle.points.len() - 1]
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            puzzle.points[i + 1..]
                .iter()
                .map(|b| Rect::new(*a, *b).area())
        })
        .max()
        .unwrap();

    println!("Part 1: {}", dist);
}

fn part2(puzzle: &Puzzle) {
    let dist = puzzle.points[..puzzle.points.len() - 1]
        .iter()
        .enumerate()
        .flat_map(|(i, a)| puzzle.points[i + 1..].iter().map(|b| Rect::new(*a, *b)))
        .filter(|rect| puzzle.rect_in_polygon(rect))
        .map(|rect| rect.area())
        .max()
        .unwrap();

    println!("Part 2: {}", dist);
}

pub fn run() {
    let puzzle = Puzzle::load("input/day9.txt");

    part1(&puzzle);

    let now = time::Instant::now();
    part2(&puzzle);
    let duration = now.elapsed();

    println!("Part 2 time: {:.2?}", duration);
}
