use crate::futil::read_lines;
use anyhow::anyhow;
use std::path::PathBuf;

struct Cipher {
    c: Vec<usize>,
    window: usize,
    i: usize
}

impl Cipher {
    fn step(&mut self) -> bool {
        let v = self.c.get(self.i).unwrap();
        for i in (self.i - self.window)..self.i {
            let i_v = self.c.get(i).unwrap();
            for j in i..self.i {
                let j_v = self.c.get(j).unwrap();
                if i_v + j_v == *v {
                    self.i += 1;
                    return true;
                }
            }
        }
        return false;
    }

    fn find_v(&mut self) -> Option<usize>{

        loop {
            if !self.step() {
                return Some(*self.c.get(self.i).unwrap());
            }
        }
    }

    fn find_range(&self, sum: usize) -> Option<(usize, usize)> {
        for i in 0..self.c.len() {
            let mut s = 0;
            for j in i..self.c.len() {
                s += self.c.get(j).unwrap();

                if s == sum {
                    return Some((i, j + 1))
                } else if s > sum {
                    break
                }
            }
        }
        None
    }

    fn find_minmax(&self, from: usize, to: usize) -> (usize, usize) {
        let mut min = usize::MAX;
        let mut max = usize::MIN;
        for i in from..to {
            let v = *self.c.get(i).unwrap();
            if v < min {
                min = v;
            }
            if v > max {
                max = v;
            }
        }

        (min, max)
    }
}

pub fn y2020p9(input: &PathBuf) -> Result<(), anyhow::Error> {
    let mut v = Vec::new();
    for maybe_line in read_lines(input)? {
        let line = maybe_line?;

        v.push(line.parse::<usize>()?);
    }

    let mut cipher = Cipher{
        c: v,
        window: 25,
        i: 25,
    };


    let a = cipher.find_v().unwrap();

    let (from, to) = cipher.find_range(a).unwrap();
    let (min, max) = cipher.find_minmax(from, to);


    println!("{} {}", a, min + max);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
    }
}
