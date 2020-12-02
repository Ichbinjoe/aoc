use anyhow::anyhow;

type Combo = [i8; 6];

fn combo_is_valid(c: &[i8; 6]) -> bool {
    let mut has_double = false;
    let mut last = -1i8;
    for i in c {
        if *i < last {
            return false
        }
        if *i == last {
            has_double = true;
        }
    }

    has_double
}

fn split_num(mut i: u32) -> Result<[i8; 6], anyhow::Error> {
    if i >= 1000000 {
        return Err(anyhow!("Invalid #"))
    }

    let r = [0; 6];
    
    for j in 0..6 {
        r[5-j] = i % 10;
        i /= 10;
    }
    Ok(r)
}

fn passwords_between(min: &[i8; 6], max: &[i8; 6]) {
    
}

fn y2019p4(min: u32, max: u32)
