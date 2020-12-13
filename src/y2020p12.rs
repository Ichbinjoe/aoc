use crate::futil::read_lines;
use std::path::PathBuf;

use regex::Regex;

#[derive(Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn with_rotation(&self, is_right: bool, amount: i32) -> Direction {
        let mut a = amount / 90;
        if !is_right {
            a = 4 - a;
        }

        let mut d = *self;
        for _ in 0..a {
            // god forgive me
            d = match d {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            }
        }
        d
    }
}

struct Coord {
    x: i32,
    y: i32,
    orientation: Direction,
}

impl Coord {
    fn apply(&mut self, d: Direction, i: i32) {
        match d {
            Direction::East => self.x -= i,
            Direction::West => self.x += i,
            Direction::North => self.y += i,
            Direction::South => self.y -= i,
        }
    }

    fn waypoint_move(&mut self, waypoint: &Coord) {
        self.x += waypoint.x;
        self.y += waypoint.y;
    }

    fn rotate_about_zero(&mut self, is_right: bool, amount: i32) {
        let mut a = amount / 90;
        if !is_right {
            a = 4 - a;
        }

        for _ in 0..a {
            let x = self.x;
            let y = self.y;
            self.x = -y;
            self.y = x;
        }
    }
}

pub fn y2020p12(input: &PathBuf) -> Result<(), anyhow::Error> {
    let re = Regex::new("(\\w)(\\d+)")?;

    let mut ship = Coord {
        x: 0,
        y: 0,
        orientation: Direction::East,
    };

    for maybe_line in read_lines(input)? {
        let line = maybe_line?;

        let caps = re.captures(&line).unwrap();

        let c0 = caps.get(1).unwrap().as_str();
        let c1 = caps.get(2).unwrap().as_str();

        let a = c1.parse::<i32>()?;
        match c0 {
            "N" => ship.apply(Direction::North, a),
            "S" => ship.apply(Direction::South, a),
            "E" => ship.apply(Direction::East, a),
            "W" => ship.apply(Direction::West, a),
            "L" => ship.orientation = ship.orientation.with_rotation(false, a),
            "R" => ship.orientation = ship.orientation.with_rotation(true, a),
            "F" => ship.apply(ship.orientation, a),
            _ => panic!("FUCK"),
        };
    }

    println!("{} {} {}", ship.x, ship.y, ship.x.abs() + ship.y.abs());

    let mut ship = Coord {
        x: 0,
        y: 0,
        orientation: Direction::East,
    };
    let mut waypoint = Coord {
        x: -10,
        y: 1,
        orientation: Direction::East,
    };

    for maybe_line in read_lines(input)? {
        let line = maybe_line?;

        let caps = re.captures(&line).unwrap();

        let c0 = caps.get(1).unwrap().as_str();
        let c1 = caps.get(2).unwrap().as_str();

        let a = c1.parse::<i32>()?;
        match c0 {
            "N" => waypoint.apply(Direction::North, a),
            "S" => waypoint.apply(Direction::South, a),
            "E" => waypoint.apply(Direction::East, a),
            "W" => waypoint.apply(Direction::West, a),
            "L" => waypoint.rotate_about_zero(false, a),
            "R" => waypoint.rotate_about_zero(true, a),
            "F" => {
                for _ in 0..a {
                    ship.waypoint_move(&waypoint)
                }
            }
            _ => panic!("FUCK"),
        };
    }
    println!("{} {} {}", ship.x, ship.y, ship.x.abs() + ship.y.abs());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
