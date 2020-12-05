use crate::futil::read_lines;
use anyhow::anyhow;
use std::path::PathBuf;

use std::str::Chars;


struct Seat {
    row: usize,
    col: usize,
}

enum SplitResult {
    Range(usize, usize),
    One(usize),
}

enum Direction {
    Higher,
    Lower,
}

fn split(lower: usize, higher: usize, d: Direction) -> SplitResult {
    let spread = higher + 1 - lower;
    if spread <= 2 {
        SplitResult::One(match d {
            Direction::Higher => higher,
            Direction::Lower => lower,
        })
    } else {
        let midpoint = (spread / 2) + lower;
        match d {
            Direction::Higher => SplitResult::Range(midpoint, higher),
            Direction::Lower => SplitResult::Range(lower, midpoint - 1),
        }
    }
}

fn ingest_splits(
    c: &mut Chars,
    low_c: char,
    high_c: char,
    mut low: usize,
    mut high: usize,
) -> SplitResult {
    for ch in c {
        let r = if ch == low_c {
            Direction::Lower
        } else if ch == high_c {
            Direction::Higher
        } else {
            break;
        };

        match split(low, high, r) {
            SplitResult::Range(lw, hgh) => {
                low = lw;
                high = hgh;
            }
            SplitResult::One(v) => {
                return SplitResult::One(v);
            }
        }
    }

    return SplitResult::Range(low, high);
}

impl Seat {
    fn seat_from_instructions(mut i: Chars) -> Result<Seat, anyhow::Error> {
        if let SplitResult::One(rw) = ingest_splits(&mut i, 'F', 'B', 0, 127) {
            if let SplitResult::One(cl) = ingest_splits(&mut i, 'L', 'R', 0, 7) {
                Ok(Seat{row: rw, col: cl})
            } else {
                Err(anyhow!("invalid col syntax"))
            }
        } else {
            Err(anyhow!("invalid row syntax"))
        }
    }

    fn seat_id(&self) -> usize {
        self.row * 8 + self.col
    }
}

fn find_seat(filled_seats: &[bool;1024]) -> Option<usize> {
    for i in 1..1023 {
        if filled_seats[i-1] && !filled_seats[i] && filled_seats[i+1] {
            return Some(i)
        }
    }
    None
}

pub fn y2020p5(input: &PathBuf) -> Result<(), anyhow::Error> {
    let mut max_seat = 0;
    let mut filled_seats = [false; 1024];
    for maybe_line in read_lines(input)? {
        let line = maybe_line?;
        let seat = Seat::seat_from_instructions(line.chars())?;
        let code = seat.seat_id();

        if code > max_seat {
            max_seat = code;
        }

        filled_seats[code] = true;
    }


    let seat = find_seat(&filled_seats);

    println!("MAX: {}, my seat: {:?}", max_seat, seat);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn passes() {
        let a = |c: &str| {
            let seat = Seat::seat_from_instructions(c.chars()).unwrap();
            seat.seat_id()
        };

        assert_eq!(a("BFFFBBFRRR"), 567);
        assert_eq!(a("FFFBBBFRRR"), 119);
        assert_eq!(a("BBFFBBFRLL"), 820);
    }
}
