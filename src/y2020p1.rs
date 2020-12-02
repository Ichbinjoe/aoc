
use std::path::PathBuf;
use crate::futil::read_lines;

pub fn y2020p1(input: &PathBuf) -> Result<(), anyhow::Error> {
    let mut nums = Vec::new();

    for maybe_line in read_lines(input)? {
        let line = maybe_line?;
        let v = line.parse::<usize>()?;
        nums.push(v);
    }

    for (i, v) in nums.iter().enumerate() {
        for v2 in nums.iter().skip(i + 1) {
            if v + v2 == 2020 {
                println!("a: {} {} {}", v, v2, v * v2);
            }
        }
    }
    
    for (i, v) in nums.iter().enumerate() {
        for (i2, v2) in nums.iter().skip(i + 1).enumerate() {
            for v3 in nums.iter().skip(i2 + 1) {
                if v + v2 + v3 == 2020 {
                    println!("b: {} {} {} {}", v, v2, v3, v * v2 * v3);
                    return Ok(())
                }
            }
        }
    }

    return Err(anyhow::anyhow!("No answer :("));
}
