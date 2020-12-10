use crate::futil::read_lines;
use anyhow::anyhow;
use std::path::PathBuf;

use std::collections::HashSet;

fn permute(a: &Vec<u32>) -> usize {
    match a.len() {
        0 => 1, /* 3, 3*/
        1 => 1, /* 3, 1, 3*/
        2 => 2, /* 3, 1, 1, 3*/
        3 => 4, /* 3, 1, 1, 1, 3*/
        4 => 7,
        _ => panic!("idk")
    }
}

fn segment(a: &Vec<u32>) -> Vec<Vec<u32>> {
    let mut q = vec![];
    let mut v = vec![];
    for va in a {
        if *va < 3 {
            v.push(*va);
        } else {
            q.push(v);
            v = vec![];
        }
    }
    q
}

pub fn y2020p10(input: &PathBuf) -> Result<(), anyhow::Error> {
    let mut a = Vec::new();

    for maybe_line in read_lines(input)? {
        let line = maybe_line?;
        a.push(line.parse::<u32>()?);
    }

    a.push(0);
    a.sort_unstable();
    let mut b = vec![];
    println!("{:?}", a);
    let mut diff1 = 0;
    let mut diff3 = 0;
    let mut last = None;
    for i in a {
        if let Some(v) = last {
            let d = i - v;
            b.push(d);
            if d == 1 {
                diff1 += 1;
            } else if d == 3 {
                diff3 += 1;
            }
        }
        last = Some(i);
    }

    b.push(3);

    println!("{:?}", b);
    let p = segment(&b);
    println!("{:?}", p);
    let mut q = 1;
    for f in p {
        q *= permute(&f);
    }

    println!("{} {} {}", diff1, diff3, diff1 * (diff3 + 1));
    println!("{}", q);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
    }
}
