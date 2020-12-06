use crate::futil::read_lines;
use anyhow::anyhow;
use std::path::PathBuf;


pub fn y2020p6(input: &PathBuf) -> Result<(), anyhow::Error> {

    let mut agg_answers = 0;
    let mut agg_answers2 = 0;
    let mut answers = [false; 26];
    let mut answers2 = [true; 26];

    for maybe_line in read_lines(input)? {
        let line = maybe_line?;
   
        if line.len() == 0 {
            for i in 0..26 {
                if answers[i] {
                    agg_answers += 1;
                }
                if answers2[i] {
                    agg_answers2 += 1;
                }
                answers[i] = false;
                answers2[i] = true;
            }
            continue
        }

        let mut answers3 = [false; 26];


        for ch in line.chars() {
            let i = ch as usize - 'a' as usize;
            if i >= 26 {
                return Err(anyhow!("HELP"))
            }

            answers[i] = true;
            answers3[i] = true;
        }

        for i in 0..26 {
            if !answers3[i] {
                answers2[i] = false;
            }
        }
    }
    for i in 0..26 {
        if answers[i] {
            agg_answers += 1;
        }
        if answers2[i] {
            agg_answers2 += 1;
        }
    }

    println!("{} {}", agg_answers, agg_answers2);
    Ok(())
}

#[cfg(test)]
mod tests {
}
