use crate::futil::read_lines;
use anyhow::{anyhow, Context};
use std::iter::Iterator;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
enum PopResult {
    One,
    Zero,
    Neither,
}

impl PopResult {
    fn invert(self) -> PopResult {
        match self {
            PopResult::One => PopResult::Zero,
            PopResult::Zero => PopResult::One,
            PopResult::Neither => PopResult::Neither,
        }
    }
}

fn bit_popularity<'a>(vs: impl Iterator<Item = &'a u64>, bit: usize) -> PopResult {
    let mut q = 0i64;
    for v in vs {
        if (v & (0x1 << bit)) > 0 {
            q += 1;
        } else {
            q -= 1;
        }
    }

    if q > 0 {
        PopResult::One
    } else if q < 0 {
        PopResult::Zero
    } else {
        PopResult::Neither
    }
}

fn filter_and_popularize(vs: &Vec<u64>, w: usize, func: impl Fn(PopResult) -> u64) -> u64 {
    let mut m = 0x0;
    let mut v = 0x0;
    for i in 0..w {
        let b = w - i - 1;
        let mut q = 0;
        let mut lv = 0;
        let p = bit_popularity(
            vs.iter().filter(|c| (*c & m) == v).inspect(|v| {
                q += 1;
                lv = **v;
            }),
            b,
        );
        if q == 1 {
            return lv;
        }
        let bv = func(p);
        v |= bv << b;
        m |= 0x1 << b;
    }
    return v;
}

fn o2fn(p: PopResult) -> u64 {
    match p {
        PopResult::One | PopResult::Neither => 1,
        PopResult::Zero => 0,
    }
}

fn co2fn(p: PopResult) -> u64 {
    match p {
        PopResult::One | PopResult::Neither => 0,
        PopResult::Zero => 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2test() {
        let v = vec![
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010,
        ];
        println!("{}", filter_and_popularize(&v, 5, co2fn));
        assert!(filter_and_popularize(&v, 5, o2fn) == 23);
        assert!(filter_and_popularize(&v, 5, co2fn) == 10);
    }
}

pub fn y2021p3(input: &PathBuf) -> Result<(), anyhow::Error> {
    let mut line_reader = read_lines(input)?;
    let mut line = line_reader.next().unwrap()?;
    let width = line.len();
    let mut vs = Vec::new();

    loop {
        let mut v = 0u64;
        for c in line.chars() {
            v <<= 1;
            if c == '1' {
                v += 1;
            } else if c != '0' {
                return Err(anyhow!("unexpected entry"));
            }
        }

        vs.push(v);

        let maybe_line = line_reader.next();
        if let Some(l) = maybe_line {
            line = l?;
        } else {
            break;
        }
    }

    let mut common = 0u64;
    for i in 0..width {
        common <<= 1;
        let pop = bit_popularity(vs.iter(), width - i - 1);
        match pop {
            PopResult::One => {
                common += 1;
            }
            PopResult::Neither => return Err(anyhow!("no popularity, yet expected popularity")),
            _ => {}
        }
    }

    let anticommon = (1 << width) - 1 - common;

    println!("{}", common * anticommon);

    let o2 = filter_and_popularize(&vs, width, o2fn);

    let co2 = filter_and_popularize(&vs, width, co2fn);

    println!("{}", o2 * co2);

    return Ok(());
}
