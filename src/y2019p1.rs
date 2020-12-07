use std::path::PathBuf;

use crate::futil::read_lines;

fn base_fuel_needed(mass: i32) -> i32 {
    mass / 3 - 2
}

fn total_fuel_needed(mass: i32) -> i32 {
    let mut total = 0;
    let mut unaccounted_for_mass = mass;

    loop {
        let new_fuel = base_fuel_needed(unaccounted_for_mass);
        if new_fuel <= 0 {
            return total;
        }

        total += new_fuel;
        unaccounted_for_mass = new_fuel;
    }
}

pub fn y2019p1(input: &PathBuf) -> Result<(), anyhow::Error> {
    let mut base_fuel = 0;
    let mut total_fuel = 0;

    for maybe_line in read_lines(input)? {
        let line = maybe_line?;
        let mass = line.parse::<i32>()?;
        base_fuel += base_fuel_needed(mass);
        total_fuel += total_fuel_needed(mass);
    }

    println!("Base Fuel: {}", base_fuel);
    println!("Total Fuel: {}", total_fuel);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_fuel_needed_test() {
        assert_eq!(base_fuel_needed(1969), 654);
        assert_eq!(base_fuel_needed(100756), 33583);
    }

    #[test]
    fn total_fuel_needed_test() {
        assert_eq!(total_fuel_needed(1969), 966);
        assert_eq!(total_fuel_needed(100756), 50346);
    }
}
