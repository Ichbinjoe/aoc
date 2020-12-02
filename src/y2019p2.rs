use std::path::PathBuf;

use anyhow::Context;

use crate::intcode::IntcodeVM;

fn run_variation(data: &Vec<i64>, noun: i64, verb: i64) -> Result<i64, anyhow::Error> {
    let mut running_data = data.clone();

    running_data[1] = noun;
    running_data[2] = verb;

    let mut vm = IntcodeVM::new(running_data);
    vm.run().with_context(|| {
        format!(
            "Failed to execute program with noun: {} verb: {}",
            noun, verb
        )
    })?;
    return Ok(vm.data()[0]);
}

pub fn y2019p2(input: &PathBuf) -> Result<(), anyhow::Error> {
    let intcode_data =
        crate::futil::read_csints(input).with_context(|| "Failed to read input program")?;

    let result = run_variation(&intcode_data, 12, 2)?;
    println!("Output: {}", result);

    for noun in 0..99 {
        for verb in 0..99 {
            let result = run_variation(&intcode_data, noun, verb)?;
            if result == 19690720 {
                println!("Found answer: {}, {} ({})", noun, verb, 100 * noun + verb);
            }
        }
    }

    Ok(())
}
