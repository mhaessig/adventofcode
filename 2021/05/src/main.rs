use std::ops::Add;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

impl Add for Point {
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + self.y,
        }
    }

    type Output = Self;
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn points(&self) -> Vec<Point> {
        let mut points = Vec::new();
        if self.start.x == self.end.x || self.start.y == self.end.y {
            for x in if self.start.x <= self.end.x {
                self.start.x..=self.end.x
            } else {
                self.end.x..=self.start.x
            } {
                for y in if self.start.y <= self.end.y {
                    self.start.y..=self.end.y
                } else {
                    self.end.y..=self.start.y
                } {
                    points.push(Point { x, y });
                }
            }
        } else {
            let mut idx = self.start.clone();
            while idx != self.end {
                points.push(idx);
                idx = Point {
                    x: if self.start.x < self.end.x {
                        idx.x + 1
                    } else {
                        idx.x - 1
                    },
                    y: if self.start.y < self.end.y {
                        idx.y + 1
                    } else {
                        idx.y - 1
                    },
                };
            }
            points.push(idx);
        }

        points
    }
}

trait Plane {
    fn cover(&mut self, line: &Line);
}

impl Plane for [[usize; 1000]; 1000] {
    fn cover(&mut self, line: &Line) {
        for Point { x, y } in line.points() {
            self[x][y] += 1;
        }
    }
}

fn main() {
    //let lines: Vec<Line> = include_str!("../example_input.txt")
    let lines: Vec<Line> = include_str!("../input.txt")
        .lines()
        .map(|s| s.split_once(" -> ").unwrap())
        .map(|(s1, s2)| (s1.split_once(',').unwrap(), s2.split_once(',').unwrap()))
        .map(|((x1, y1), (x2, y2))| Line {
            start: Point {
                x: x1.parse().unwrap(),
                y: y1.parse().unwrap(),
            },
            end: Point {
                x: x2.parse().unwrap(),
                y: y2.parse().unwrap(),
            },
        })
        .collect();

    let mut plane = [[0; 1000]; 1000];
    for line in &lines {
        plane.cover(line);
    }

    let count = plane.iter().fold(0, |count, row| {
        count + row.into_iter().filter(|&&n| n >= 2).count()
    });

    dbg!(count);
}
