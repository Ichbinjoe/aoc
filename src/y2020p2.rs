extern crate regex;

use anyhow::anyhow;
use regex::Regex;
use std::path::PathBuf;
use crate::futil::read_lines;

pub fn y2020p2(input: &PathBuf) -> Result<(), anyhow::Error> {
    let re = Regex::new("(\\d+)-(\\d+) (\\w): (\\w+)")?;
    let mut passes1 = 0;
    let mut passes2 = 0;
    for maybe_line in read_lines(input)? {
        let line = maybe_line?;
        
        let captures = re.captures(&line).ok_or(anyhow!("line did not match expected syntax"))?;

        // This is all safe and won't panic because of the fixed regex above
        let a = captures.get(1).unwrap().as_str().parse::<usize>()?;
        let b = captures.get(2).unwrap().as_str().parse::<usize>()?;
        let ch = captures.get(3).unwrap().as_str().chars().next().unwrap();
        let p = captures.get(4).unwrap().as_str();
        let mut i = 0;
        let mut j = 0;
        
        for (z, c) in p.chars().enumerate() {
            if c == ch {
                i += 1;
                let z1 = z + 1;
                if z1 == a || z1 == b {
                    j += 1;
                }
            }
        }

        if j == 1 {
            passes2 += 1;
        }


        if a <= i && i <= b {
            passes1 += 1;
        }
    }

    println!("a: {}, b: {}", passes1, passes2);
    Ok(())
}
