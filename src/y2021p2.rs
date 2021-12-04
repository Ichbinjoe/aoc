use crate::futil::read_lines;
use anyhow::{anyhow, Context};
use std::path::PathBuf;
use std::str::FromStr;

struct SubCoords {
    h: i64,
    d: i64,
    a: i64,
}

enum Command {
    Forward(i64),
    Down(i64),
    Up(i64),
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s_parts = s.split(" ");
        let cmd = s_parts.next().ok_or(anyhow!("missing command"))?;
        let v = s_parts
            .next()
            .ok_or(anyhow!("missing distance"))?
            .parse::<i64>()
            .context("malformed distance")?;
        match cmd {
            "forward" => Ok(Command::Forward(v)),
            "down" => Ok(Command::Down(v)),
            "up" => Ok(Command::Up(v)),
            _ => Err(anyhow!("unexpected command")),
        }
    }
}

impl Command {
    fn apply_to_sub(&self, sub: &mut SubCoords) {
        match self {
            Command::Forward(i) => sub.h += i,
            Command::Down(i) => sub.d += i,
            Command::Up(i) => sub.d -= i,
        }
    }

    fn apply_to_sub2(&self, sub: &mut SubCoords) {
        match self {
            Command::Forward(i) => {
                sub.h += i;
                sub.d += i * sub.a;
            }
            Command::Down(i) => sub.a += i,
            Command::Up(i) => sub.a -= i,
        }
    }
}

pub fn y2021p2(input: &PathBuf) -> Result<(), anyhow::Error> {
    let cmds = read_lines(input)?;
    let mut sub = SubCoords { h: 0, d: 0, a: 0 };
    let mut sub2 = SubCoords { h: 0, d: 0, a: 0 };
    for maybe_cmd in cmds {
        let cmd = Command::from_str(&maybe_cmd?)?;
        cmd.apply_to_sub(&mut sub);
        cmd.apply_to_sub2(&mut sub2);
    }

    println!("i {}, j {}", sub.h * sub.d, sub2.h * sub2.d);

    return Ok(());
}
