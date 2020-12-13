use crate::futil::read_lines;
use std::path::PathBuf;

fn lookup(v: u32) -> usize {
    match v {
        0 => 1, /* 3, 3*/
        1 => 1, /* 3, 1, 3*/
        2 => 2, /* 3, 1, 1, 3*/
        3 => 4, /* 3, 1, 1, 1, 3*/
        4 => 7,
        _ => panic!("idk"),
    }
}

pub fn y2020p10(input: &PathBuf) -> Result<(), anyhow::Error> {
    let mut adapters = Vec::new();
    adapters.push(0);

    for maybe_line in read_lines(input)? {
        let line = maybe_line?;
        adapters.push(line.parse::<u32>()?);
    }

    adapters.sort_unstable();
    let adapter_jumps = adapters.windows(2).map(|a| a[1] - a[0]);
    let mut ones = 0;
    let mut threes = 0;
    let mut q = 1;
    let mut running_ones = 0;
    for jump in adapter_jumps {
        if jump == 1 {
            running_ones += 1;
            ones += 1;
        } else {
            q *= lookup(running_ones);
            running_ones = 0;
            threes += 1;
        }
    }
    q *= lookup(running_ones);

    println!(
        "Ones {} Threes {} P1 {} Combs {}",
        ones,
        threes,
        ones * (threes + 1),
        q
    );
    Ok(())
}
