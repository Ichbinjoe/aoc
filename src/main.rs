extern crate anyhow;
extern crate nom;
extern crate structopt;

use std::path::PathBuf;
use structopt::*;
mod futil;
mod intcode;
mod y2019p1;
mod y2019p2;
mod y2019p3;
mod y2019p5;

mod y2020p1;
mod y2020p2;
mod y2020p3;
mod y2020p4;
mod y2020p5;
mod y2020p6;
mod y2020p7;

#[derive(StructOpt)]
enum Y2019 {
    P1 { input: PathBuf },
    P2 { input: PathBuf },
    P3 { input: PathBuf },
    P5 { input: PathBuf },
    //   P4 { min: u32, max: u32 },
}

#[derive(StructOpt)]
enum Y2020 {
    P1 { input: PathBuf },
    P2 { input: PathBuf },
    P3 { input: PathBuf },
    P4 { input: PathBuf },
    P5 { input: PathBuf },
    P6 { input: PathBuf },
    P7 { input: PathBuf },
}

#[derive(StructOpt)]
enum Year {
    Y2019(Y2019),
    Y2020(Y2020),
}

fn run2019(y: &Y2019) -> Result<(), anyhow::Error> {
    match y {
        Y2019::P1 { input } => {
            y2019p1::y2019p1(&input)?;
        }
        Y2019::P2 { input } => {
            y2019p2::y2019p2(&input)?;
        }
        Y2019::P3 { input } => {
            y2019p3::y2019p3(&input)?;
        }
        Y2019::P5 { input } => {
            y2019p5::y2019p5(input)?;
        }
    };
    Ok(())
}

fn run2020(y: &Y2020) -> Result<(), anyhow::Error> {
    match y {
        Y2020::P1 { input } => {
            y2020p1::y2020p1(&input)?;
        }
        Y2020::P2 { input } => {
            y2020p2::y2020p2(&input)?;
        }
        Y2020::P3 { input } => {
            y2020p3::y2020p3(&input)?;
        }
        Y2020::P4 { input } => {
            y2020p4::y2020p4(&input)?;
        }
        Y2020::P5 { input } => {
            y2020p5::y2020p5(&input)?;
        }
        Y2020::P6 { input } => {
            y2020p6::y2020p6(&input)?;
        }
        Y2020::P7 { input } => {
            y2020p7::y2020p7(&input)?;
        }
    }
    Ok(())
}

fn main() {
    let opt = Year::from_args();
    let r = match opt {
        Year::Y2019(y) => run2019(&y),
        Year::Y2020(y) => run2020(&y),
    };

    if let Err(err) = r {
        println!("Error occurred while executing: {:?}", err);
    }
}
