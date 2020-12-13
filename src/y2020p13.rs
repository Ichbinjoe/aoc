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

    // Okay, here is some cheese.
    //
    let multiplicatinator = 17671u64;
    for i in 1..u64::MAX {
        let v = multiplicatinator * i;

        if i % 100000 == 0{
            println!("FUCK {}", i);
        }
        if fuck(v - 41, &busses) {
            println!("{}", v);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
    }
}
