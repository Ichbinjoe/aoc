use crate::futil::read_lines;
use std::path::PathBuf;

fn tree(s: &str, i: usize, m: usize, l: usize) -> usize {
    let j = (i * m) % l;
    if s.chars().nth(j).unwrap() == '#' {
        1
    } else {
        0
    }
}

pub fn y2020p3(input: &PathBuf) -> Result<(), anyhow::Error> {
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    let mut d = 0;
    let mut e = 0;
    for (l, maybe_line) in read_lines(input)?.enumerate() {
        let line = maybe_line?;

        let w = line.len();

        a += tree(&line, l, 1, w);
        b += tree(&line, l, 3, w);
        c += tree(&line, l, 5, w);
        d += tree(&line, l, 7, w);
        if l % 2 == 0 {
            e += tree(&line, l / 2, 1, w);
        }
    }

    println!("TREE: {} {}", a, a * b * c * d * e);

    Ok(())
}
