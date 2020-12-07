use crate::futil::read_lines;
use anyhow::anyhow;
use std::path::PathBuf;

const DEFAULT_ALL_ANSWERS: u32 = 0x3ffffff;

pub fn y2020p6(input: &PathBuf) -> Result<(), anyhow::Error> {
    let mut aggregate_any_answers = 0;
    let mut aggregate_all_answers = 0;

    let mut any_answers = 0_u32;
    let mut all_answers = DEFAULT_ALL_ANSWERS;

    for maybe_line in read_lines(input)? {
        let line = maybe_line?;

        if line.len() == 0 {
            aggregate_any_answers += any_answers.count_ones();
            aggregate_all_answers += all_answers.count_ones();
            any_answers = 0;
            all_answers = DEFAULT_ALL_ANSWERS;
            continue;
        }

        let mut row_answers = 0_u32;

        for ch in line.chars() {
            let i = ch as usize - 'a' as usize;
            row_answers |= 1 << i;
        }

        any_answers |= row_answers;
        all_answers &= row_answers;
    }

    aggregate_any_answers += any_answers.count_ones();
    aggregate_all_answers += all_answers.count_ones();
    println!("{} {}", aggregate_any_answers, aggregate_all_answers);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default_all_answers() {
        assert_eq!(DEFAULT_ALL_ANSWERS.count_ones(), 26);
    }
}
