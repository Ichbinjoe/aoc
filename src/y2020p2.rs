extern crate regex;

use regex::Regex;

use std::path::PathBuf;
use crate::futil::read_lines;

pub fn y2020p2(input: &PathBuf) -> Result<(), anyhow::Error> {
    let re = Regex::new("(\\d+)-(\\d+) (\\w): (\\w+)").unwrap();
    let mut passes1 = 0;
    let mut passes2 = 0;
    for maybe_line in read_lines(input)? {
        let line = maybe_line?;
        
        let captures = re.captures(&line).unwrap();

        let a = captures.get(1).unwrap().as_str().parse::<usize>()?;
        let b = captures.get(2).unwrap().as_str().parse::<usize>()?;
        let ch = captures.get(3).unwrap().as_str().chars().next().unwrap();
        let p = captures.get(4).unwrap().as_str();
        let mut i = 0;
        let chrz: Vec<char> = p.chars().collect();
        for c in &chrz {
            if *c == ch {
                i += 1;
            }
        }

        let ais = *chrz.get(a - 1).unwrap() == ch; 
        let bis = *chrz.get(b - 1).unwrap() == ch; 

        if ais ^ bis {
            passes2 += 1;
        }


        if a <= i && i <= b {
            passes1 += 1;
        }
    }

    println!("a: {}, b: {}", passes1, passes2);
    Ok(())
}
