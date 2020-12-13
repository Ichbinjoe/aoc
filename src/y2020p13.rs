use crate::futil::read_lines;
use std::path::PathBuf;

fn fuck(d: u64, i: &Vec<(usize, u64)>) -> bool {
    for (off, fuckfuckfuck) in i {
        let v = d + *off as u64;
        if v % fuckfuckfuck != 0 {
            return false
        }
    }
    return true
}

fn big_thonk(period: u64, inset: u64, period2: u64, offset: u64) -> (u64, u64) {
    let new_period = period * period2;
    for q in 0..period2 {
        let base = period * q;
        let v = base + inset;
        if (v + offset) % period2 == 0 {
            return (new_period, v)
        }
    }

    panic!("OH SHITTTTTTTT");
}

pub fn y2020p13(input: &PathBuf) -> Result<(), anyhow::Error> {
    
    let mut lines = read_lines(input)?;

    let arrival = lines.next().unwrap()?.parse::<u64>()?;
    let busses: Vec<u64> = lines.next().unwrap()?.split(",").filter(|x| *x != "x").map(|i| i.parse::<u64>().unwrap()).collect();

    let mut m: Vec<(u64, u64)> = busses.iter().map(|b| {
        let depart = b * ((arrival / b) + 1);
        let diff = depart - arrival;
        println!("{}", diff);
        (*b, diff)
    }).collect();

    m.sort_by(|(b1, diff1), (b2, diff2)| diff1.cmp(diff2));

    println!("{:?}", m);
    
    let mut lines = read_lines(input)?;

    let _ = lines.next().unwrap()?.parse::<u64>()?;
    let busses: Vec<(usize, u64)> = lines.next().unwrap()?.split(",").enumerate().filter(|(i, x)| *x != "x").map(|(v, i)| (v, i.parse::<u64>().unwrap())).collect();

    println!("{:?}", busses); 

    let mut bus_iter = busses.iter();
    let (fikds, mut period) = bus_iter.next().unwrap();
    let mut off = *fikds as u64;
    for (o2, p2) in bus_iter {
        let (np, no) = big_thonk(period, off as u64, *p2, *o2 as u64);
        println!("{} {} DDD", np, no);
        period = np;
        off = no;
    }

    println!("{} {}", off, period);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
    }
}
