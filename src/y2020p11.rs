use crate::futil::read_lines;
use std::path::PathBuf;

#[derive(Copy, Clone, Debug, PartialEq)]
enum State {
    Occupied,
    Seat,
    Floor,
}

fn state(m: &Vec<Vec<State>>, x: i64, y: i64) -> State {
    *m.get(x as usize).unwrap().get(y as usize).unwrap()
}

fn occupied(m: &Vec<Vec<State>>, x: i64, y: i64) -> bool {
    if x < 0 || y < 0 {
        false
    } else {
        m.get(x as usize)
            .and_then(|a| a.get(y as usize))
            .map(|s| *s == State::Occupied)
            .or_else(|| Some(false))
            .unwrap()
    }
}

fn raytrace(m: &Vec<Vec<State>>, mut x: i64, mut y: i64, d: (i64, i64)) -> bool {
    let (xd, yd) = d;
    loop {
        x += xd;
        y += yd;
        if x < 0 || y < 0 {
            return false;
        } else {
            let state = m.get(x as usize).and_then(|a| a.get(y as usize));
            if let Some(s) = state {
                match s {
                    State::Floor => continue,
                    State::Seat => return false,
                    State::Occupied => return true,
                }
            } else {
                return false;
            }
        }
    }
}

fn step(x: i64, y: i64, c: &mut i64, old: &Vec<Vec<State>>, new: &mut Vec<Vec<State>>) {
    for x in 0..x {
        for y in 0..y {
            let me = state(old, x, y);

            let mut surrounding_occupied = 0;
            for x1 in x - 1..x + 2 {
                for y1 in y - 1..y + 2 {
                    if x1 == x && y1 == y {
                        continue;
                    }

                    if occupied(old, x1, y1) {
                        surrounding_occupied += 1;
                    }
                }
            }

            *new.get_mut(x as usize)
                .unwrap()
                .get_mut(y as usize)
                .unwrap() = match me {
                State::Seat => {
                    if surrounding_occupied == 0 {
                        *c += 1;
                        State::Occupied
                    } else {
                        State::Seat
                    }
                }
                State::Floor => State::Floor,
                State::Occupied => {
                    if surrounding_occupied >= 4 {
                        *c += 1;
                        State::Seat
                    } else {
                        State::Occupied
                    }
                }
            }
        }
    }
}

fn step2(x: i64, y: i64, c: &mut i64, old: &Vec<Vec<State>>, new: &mut Vec<Vec<State>>) {
    for x in 0..x {
        for y in 0..y {
            let me = state(old, x, y);

            let mut surrounding_occupied = 0;
            let directions = vec![
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, 1),
                (0, -1),
                (1, -1),
                (1, 0),
                (1, 1),
            ];
            for direction in directions {
                if raytrace(old, x, y, direction) {
                    surrounding_occupied += 1;
                }
            }

            *new.get_mut(x as usize)
                .unwrap()
                .get_mut(y as usize)
                .unwrap() = match me {
                State::Seat => {
                    if surrounding_occupied == 0 {
                        *c += 1;
                        State::Occupied
                    } else {
                        State::Seat
                    }
                }
                State::Floor => State::Floor,
                State::Occupied => {
                    if surrounding_occupied >= 5 {
                        *c += 1;
                        State::Seat
                    } else {
                        State::Occupied
                    }
                }
            }
        }
    }
}

fn count(x: i64, y: i64, n: &Vec<Vec<State>>) -> usize {
    let mut sum = 0;
    for x1 in 0..x {
        for y1 in 0..y {
            if occupied(&n, x1 as i64, y1 as i64) {
                sum += 1;
            }
        }
    }

    sum
}

pub fn y2020p11(input: &PathBuf) -> Result<(), anyhow::Error> {
    let mut m: Vec<Vec<State>> = vec![];

    for maybe_line in read_lines(input)? {
        let line = maybe_line?;
        m.push(
            line.chars()
                .map(|c| match c {
                    'L' => State::Seat,
                    '.' => State::Floor,
                    _ => panic!("FuCK"),
                })
                .collect(),
        );
    }

    let l = m.len();
    let w = m[0].len();
    let mut last = m.clone();
    loop {
        let mut next = last.clone();
        let mut c = 0;
        step(l as i64, w as i64, &mut c, &last, &mut next);

        if last == next {
            let old = count(l as i64, w as i64, &last);
            let new = count(l as i64, w as i64, &next);

            println!("{} {}", old, new);
            break;
        } else {
            last = next;
        }
    }
    let mut cnt = 0;
    let mut last2 = m;
    loop {
        let mut next = last2.clone();
        let mut c = 0;
        step2(l as i64, w as i64, &mut c, &last2, &mut next);

        println!("{}", c);
        cnt += 1;
        if last2 == next {
            let old = count(l as i64, w as i64, &last2);
            let new = count(l as i64, w as i64, &next);

            println!("{} {} {}", cnt, old, new);
            break;
        } else {
            last2 = next;
        }
    }

    Ok(())
}
