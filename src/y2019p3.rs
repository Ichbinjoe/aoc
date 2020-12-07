use anyhow::anyhow;
use std::cmp;
use std::collections::HashSet;
use std::path::PathBuf;

#[derive(Eq, Hash, PartialEq, Debug)]
struct Point {
    x: i32, // horizontal
    y: i32, // vertical
}

impl Point {
    fn manhattan_distance(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn horizontal_offset(&self, x_off: i32) -> (Line, Point) {
        let new_x = self.x + x_off;

        (
            Line {
                anchor: Point {
                    x: cmp::min(new_x, self.x),
                    y: self.y,
                },
                distance: x_off.abs(),
                direction: Direction::Horizontal,
                reverse: x_off < 0,
            },
            Point {
                x: new_x,
                y: self.y,
            },
        )
    }

    fn vertical_offset(&self, y_off: i32) -> (Line, Point) {
        let new_y = self.y + y_off;

        (
            Line {
                anchor: Point {
                    x: self.x,
                    y: cmp::min(new_y, self.y),
                },
                distance: y_off.abs(),
                direction: Direction::Vertical,
                reverse: y_off < 0,
            },
            Point {
                x: self.x,
                y: new_y,
            },
        )
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    Vertical,
    Horizontal,
}

#[derive(Debug)]
struct Line {
    anchor: Point,
    distance: i32,
    direction: Direction,
    reverse: bool,
}

impl Line {
    fn polarize_partial_len(&self, i: i32) -> i32 {
        if self.reverse {
            self.distance - i
        } else {
            i
        }
    }

    fn intersects(&self, other: &Line) -> Option<(Point, i32)> {
        if self.direction == other.direction {
            return None;
        }

        let (horizontal, vertical) = match self.direction {
            Direction::Vertical => (other, self),
            Direction::Horizontal => (self, other),
        };

        if horizontal.anchor.x <= vertical.anchor.x
            && vertical.anchor.x <= horizontal.anchor.x + horizontal.distance
            && vertical.anchor.y <= horizontal.anchor.y
            && horizontal.anchor.y <= vertical.anchor.y + vertical.distance
        {
            Some((
                Point {
                    x: vertical.anchor.x,
                    y: horizontal.anchor.y,
                },
                (horizontal.polarize_partial_len(vertical.anchor.x - horizontal.anchor.x)
                    + vertical.polarize_partial_len(horizontal.anchor.y - vertical.anchor.y)),
            ))
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Path {
    segments: Vec<Line>,
}

impl Path {
    fn new() -> Path {
        Path {
            segments: Vec::new(),
        }
    }

    fn intersections<F>(&self, other: &Path, mut f: F)
    where
        F: FnMut(Point, i32),
    {
        let mut path1_dist = 0;
        for line_a in &self.segments {
            let mut path2_dist = 0;
            for line_b in &other.segments {
                if let Some((intersection, distance)) = line_a.intersects(&line_b) {
                    f(intersection, path1_dist + path2_dist + distance);
                }
                path2_dist += line_b.distance;
            }
            path1_dist += line_a.distance;
        }
    }
}

struct PathBuilder<'a> {
    end: Point,
    path: &'a mut Path,
}

enum Motion {
    Up,
    Down,
    Left,
    Right,
}

impl<'a> PathBuilder<'a> {
    fn new(p: &'a mut Path) -> PathBuilder<'a> {
        PathBuilder {
            end: Point { x: 0, y: 0 },
            path: p,
        }
    }

    fn append(&mut self, m: Motion, magnitude: i32) {
        let (line, point) = match m {
            Motion::Up => self.end.vertical_offset(magnitude),
            Motion::Down => self.end.vertical_offset(-magnitude),
            Motion::Left => self.end.horizontal_offset(-magnitude),
            Motion::Right => self.end.horizontal_offset(magnitude),
        };

        self.path.segments.push(line);
        self.end = point;
    }
}

fn parse_motion(m: &str) -> Result<(Motion, i32), anyhow::Error> {
    let mut chars = m.chars();
    let motion = match chars.next() {
        Some(c) => match c {
            'U' => Motion::Up,
            'D' => Motion::Down,
            'L' => Motion::Left,
            'R' => Motion::Right,
            c => return Err(anyhow!("Invalid motion: {}", c)),
        },
        None => return Err(anyhow!("No motion character")),
    };

    let magnitude = chars.as_str().parse::<u32>()? as i32;

    Ok((motion, magnitude))
}

fn build_path_from_str(s: &str) -> Result<Path, anyhow::Error> {
    let motions = s.split(",").map(|point| point.trim()).map(parse_motion);
    let mut path = Path::new();
    let mut builder = PathBuilder::new(&mut path);

    for m in motions {
        let (motion, magnitude) = m?;
        builder.append(motion, magnitude);
    }

    Ok(path)
}

pub fn y2019p3(input: &PathBuf) -> Result<(), anyhow::Error> {
    let path_strings = crate::futil::read_lines(input)?;
    let mut paths = Vec::new();
    for path_string_maybe in path_strings {
        let path_string = path_string_maybe?;

        paths.push(build_path_from_str(&path_string)?);
    }

    let mut intersections = HashSet::new();
    let mut fewest_combined_steps = i32::MAX;

    for (i, path_a) in paths.iter().enumerate() {
        for path_b in paths.iter().skip(i + 1) {
            path_a.intersections(path_b, |p, d| {
                intersections.insert(p);
                if d < fewest_combined_steps {
                    fewest_combined_steps = d;
                }
            });
        }
    }

    let center = Point { x: 0, y: 0 };
    let mut closest = i32::MAX;
    for intersection in intersections {
        let distance = intersection.manhattan_distance(&center);
        if distance < closest && distance != 0 {
            closest = distance;
        }
    }

    println!(
        "Closest: {}, Lowest Dist: {}",
        closest, fewest_combined_steps
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_intersects() {
        let line_a = Line {
            anchor: Point { x: 3, y: 5 },
            distance: 5,
            direction: Direction::Horizontal,
            reverse: true,
        };
        let line_b = Line {
            anchor: Point { x: 6, y: 3 },
            distance: 4,
            direction: Direction::Vertical,
            reverse: true,
        };

        assert_eq!(
            line_a.intersects(&line_b).unwrap(),
            (Point { x: 6, y: 5 }, 4)
        );
    }

    #[test]
    fn test_1() {
        let mut path1 = build_path_from_str("R8,U5,L5,D3").unwrap();
        let mut path2 = build_path_from_str("U7,R6,D4,L4").unwrap();
    }
}
