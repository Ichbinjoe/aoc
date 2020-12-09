use crate::futil::read_lines;
use anyhow::anyhow;
use std::path::PathBuf;
use std::collections::HashSet;

pub fn y2020p8(input: &PathBuf) -> Result<(), anyhow::Error> {
    let mut instructions = Vec::new();
    for maybe_line in read_lines(input)? {
        let line = maybe_line?;
        let instruction = Instruction::from_line(&line)?;
        instructions.push(instruction);
    }

    let mut vm = VM::new(&instructions);

    loop {
        if let Terminate::Loop = vm.step() {
            println!("{}", vm.accum);
            break;
        }
    }

    for i in 0..instructions.len() {
        let mut modified = instructions.clone();
        let i = modified.get_mut(i).unwrap();

        *i = match i {
            Instruction::Accumulator(_) => continue,
            Instruction::Jump(i) => Instruction::Nop(*i),
            Instruction::Nop(i) => Instruction::Jump(*i),
        };

        let mut vm = VM::new(&modified);
        loop {
            match vm.step() {
                Terminate::Normal => continue,
                Terminate::Loop => break,
                Terminate::Halt => {
                    println!("{}", vm.accum);
                    return Ok(());
                }
            }
        }
        drop(vm);
    }

    Ok(())
}

#[derive(Copy, Clone)]
enum Instruction {
    Accumulator(i32),
    Jump(i32),
    Nop(i32),
}

impl Instruction {
    fn from_line(s: &str) -> Result<Instruction, anyhow::Error> {
        let mut args = s.split(" ");

        let op = args.next().unwrap();
        let arg = args.next().unwrap();

        match op {
            "nop" => Ok(Instruction::Nop(arg.parse::<i32>()?)),
            "acc" => Ok(Instruction::Accumulator(arg.parse::<i32>()?)),
            "jmp" => Ok(Instruction::Jump(arg.parse::<i32>()?)),
            _ => Err(anyhow!("Not a valid op: {}", op)),
        }
    }
}

struct VM<'a> {
    accum: i32,
    ip: i32,
    instructions: &'a Vec<Instruction>,
    hits: HashSet<i32>,
}

enum Terminate {
    Normal,
    Loop,
    Halt,
}

impl <'a> VM<'a> {
    fn new(v: &'a Vec<Instruction>) -> VM<'a> {
        VM {
            accum: 0,
            ip: 0,
            instructions: v,
            hits: HashSet::new(),
        }
    }

    fn step(&mut self) -> Terminate {
        if self.hits.contains(&self.ip) {
            return Terminate::Loop;
        }

        let instruction = match self.instructions.get(self.ip as usize) {
            Some(i) => i,
            None => return Terminate::Halt,
        };

        self.hits.insert(self.ip);

        match instruction {
            Instruction::Accumulator(a) => {
                self.accum += *a;
                self.ip += 1
            }
            Instruction::Jump(j) => self.ip += *j,
            Instruction::Nop(_) => self.ip += 1,
        }
        return Terminate::Normal;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let instructions = vec![
            Instruction::Nop(0),
            Instruction::Accumulator(1),
            Instruction::Jump(4),
            Instruction::Accumulator(3),
            Instruction::Jump(-3),
            Instruction::Accumulator(-99),
            Instruction::Accumulator(1),
            Instruction::Jump(-4),
            Instruction::Accumulator(6),
        ];

        let mut vm = VM::new(&instructions);
        for _ in 0..100 {
            if let Terminate::Loop = vm.step() {
                assert_eq!(5, vm.accum);
                return;
            }
        }
        assert!(false);
    }
}
