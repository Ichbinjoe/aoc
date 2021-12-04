use crate::futil::read_lines;
use std::cell::RefCell;
use std::path::PathBuf;

#[derive(Clone)]
struct MarkedBoardRef<'a> {
    b: &'a Board,
    v: u32,
}

struct BoardState {
    called: u32,
    has_won: bool,
}

struct Board {
    spots: Vec<u32>,
    state: RefCell<BoardState>,
}

impl BoardState {
    fn score(&self, board: &Board) -> u32 {
        let mut sum = 0;
        for (i, spot) in board.spots.iter().enumerate() {
            if self.called & 0b1 << i == 0 {
                sum += spot;
            }
        }
        sum
    }
}

fn horizontal_win(row: u32) -> u32 {
    0b11111 << (row * 5)
}

fn vertical_win(col: u32) -> u32 {
    let mut v = 0;
    for i in 0..5 {
        v |= 0b1 << (i * 5);
    }

    v << col
}

pub fn y2021p4(input: &PathBuf) -> Result<(), anyhow::Error> {
    let mut lines = read_lines(input)?;
    let called_numbers = lines
        .next()
        .unwrap()?
        .split(",")
        .map(|v| v.parse().unwrap())
        .collect::<Vec<u32>>();

    let mut boards = vec![];

    // Ensures there is a 'blank line header' for a new board
    while !lines.next().is_none() {
        let mut board = Board {
            spots: vec![],
            state: RefCell::new(BoardState {
                called: 0,
                has_won: false,
            }),
        };

        for _ in 0..5 {
            let l = lines.next().unwrap()?;
            for entry in l.split(" ") {
                if entry.is_empty() {
                    continue;
                }
                let val = entry.parse().unwrap();
                board.spots.push(val);
            }
        }

        boards.push(board);
    }

    let mut markers = vec![vec! {}; (*called_numbers.iter().max().unwrap() as usize) + 1];

    for b in &boards {
        for (i, v) in b.spots.iter().enumerate() {
            let board_marker = MarkedBoardRef { b, v: i as u32 };

            markers[*v as usize].push(board_marker);
        }
    }

    let mut wins = 0;
    for n in called_numbers {
        for m in &markers[n as usize] {
            let mut state = m.b.state.borrow_mut();
            if state.has_won {
                continue;
            }

            let v = m.v;
            state.called |= 0x1 << v;
            let h_w = horizontal_win(v / 5);
            let v_w = vertical_win(v % 5);

            if state.called & h_w == h_w || state.called & v_w == v_w {
                println!("win {}: {}", wins, state.score(m.b) * n);
                state.has_won = true;
                wins += 1;
            }
        }
    }

    Ok(())
}
