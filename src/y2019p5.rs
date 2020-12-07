use std::path::PathBuf;

use anyhow::anyhow;
use anyhow::Context;

use crate::intcode::{IntcodeVM, InterruptReason};

pub fn y2019p5(input: &PathBuf) -> Result<(), anyhow::Error> {
    let diagnostic_program =
        crate::futil::read_csints(input).with_context(|| "Failed to read input program")?;

    let mut vm = IntcodeVM::new(diagnostic_program.clone());

    assert_eq!(vm.run()?, InterruptReason::WaitingForInput);
    vm.input(1)?;

    loop {
        match vm.run()? {
            InterruptReason::WaitingForOutput => {
                let output = vm.output()?;
                println!("Output: {}", output);
            }
            InterruptReason::Terminate => {
                println!("Terminate");
                break;
            }
            InterruptReason::WaitingForInput => {
                return Err(anyhow!("Waiting for input with none to give"));
            }
        }
    }

    let mut vm2 = IntcodeVM::new(diagnostic_program);
    assert_eq!(vm2.run()?, InterruptReason::WaitingForInput);
    vm2.input(5)?;
    assert_eq!(vm2.run()?, InterruptReason::WaitingForOutput);
    println!("Output: {}", vm2.output()?);
    assert_eq!(vm2.run()?, InterruptReason::Terminate);
    Ok(())
}
