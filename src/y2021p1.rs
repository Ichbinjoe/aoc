use crate::futil::read_lines;
use anyhow::Result;
use std::path::PathBuf;

pub fn y2021p1(input: &PathBuf) -> Result<(), anyhow::Error> {
    let nums: Vec<i64> = read_lines(input)?
        .map(|maybe_line| {
            maybe_line
                .map_err(anyhow::Error::from)
                .and_then(|l| l.parse::<i64>().map_err(anyhow::Error::from))
        })
        .collect::<Result<Vec<i64>, anyhow::Error>>()?;

    let mut i = 0;
    let mut depths = nums.iter();
    let mut last_depth = depths.next().expect("must have at least one value");
    for next_depth in depths {
        if next_depth > last_depth {
            i += 1;
        }
        last_depth = next_depth;
    }
    println!("i: {}", i);

    let mut j = 0;
    let mut depths2 = nums.windows(3).map(|w| w.iter().sum());
    let mut last_depth2: i64 = depths2.next().unwrap();
    for next_depth2 in depths2 {
        if next_depth2 > last_depth2 {
            j += 1;
        }
        last_depth2 = next_depth2;
    }
    println!("j: {}", j);

    return Ok(());
}
