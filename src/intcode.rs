use std::error::Error;
use std::fmt;
use std::ops::{Deref, DerefMut};

extern crate proc;

#[derive(Debug, PartialEq)]
enum VMState {
    Ready,
    Fault,
    WaitingForInput(OpArg),
    WaitingForOutput(OpArg),
}

const PAGE_ELEMENT_COUNT: usize = std::mem::size_of::<i64>();

struct VMMMU {}

#[derive(Debug)]
pub struct IntcodeVM<T>
where
    T: DerefMut<Target = [i64]>,
{
    ip: usize,
    data: T,
    state: VMState,
    relative_base: i64,
}

#[derive(Debug)]
pub enum IntcodeError {
    OutOfBoundsDereference(i64),
    IllegalOpcode(i64),
    OutOfBoundsArguments(i64),
    OutOfBoundsIp(i64),
    IllegalState,
    IllegalStore,
}

impl fmt::Display for IntcodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IntcodeError::OutOfBoundsDereference(a) => {
                write!(f, "IntcodeError: Out of bounds dereference at {}", a)
            }
            IntcodeError::IllegalOpcode(a) => write!(f, "IntcodeError: Hit illegal opcode ({})", a),
            IntcodeError::OutOfBoundsArguments(a) => {
                write!(f, "IntcodeError: Out of bounds arguments at {}", a)
            }
            IntcodeError::OutOfBoundsIp(a) => write!(f, "IntcodeError: Out of bounds ip at {}", a),
            IntcodeError::IllegalState => write!(f, "IntcodeError: Illegal state"),
            IntcodeError::IllegalStore => write!(f, "IntcodeError: Illegal store"),
        }
    }
}

impl Error for IntcodeError {}

#[derive(Debug, PartialEq)]
pub enum InterruptReason {
    Terminate,
    WaitingForInput,
    WaitingForOutput,
}

#[derive(Debug, PartialEq)]
pub enum StepResult {
    Continue,
    Jump,
    Interrupt(InterruptReason),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum OpArg {
    Position(i64),
    Immediate(i64),
    Relative(i64),
}

fn read_index<D: Deref<Target = [i64]>>(data: &D, index: i64) -> Result<i64, IntcodeError> {
    if index < 0 {
        return Err(IntcodeError::OutOfBoundsDereference(index));
    }
    match data.get(index as usize) {
        Some(x) => Ok(*x),
        None => Err(IntcodeError::OutOfBoundsDereference(index)),
    }
}

fn read_index_mut<D: DerefMut<Target = [i64]>>(
    data: &mut D,
    index: i64,
) -> Result<&mut i64, IntcodeError> {
    if index < 0 {
        return Err(IntcodeError::OutOfBoundsDereference(index));
    }
    match data.get_mut(index as usize) {
        Some(x) => Ok(x),
        None => Err(IntcodeError::OutOfBoundsDereference(index)),
    }
}

fn intcode_op_add<D: DerefMut<Target = [i64]>>(
    vm: &mut IntcodeVM<D>,
    left_idx_1: OpArg,
    left_idx_2: OpArg,
    right_idx: OpArg,
) -> Result<StepResult, IntcodeError> {
    let op_1 = vm.read(left_idx_1)?;
    let op_2 = vm.read(left_idx_2)?;

    *vm.write(right_idx)? = op_1 + op_2;

    Ok(StepResult::Continue)
}

fn intcode_op_mul<D: DerefMut<Target = [i64]>>(
    vm: &mut IntcodeVM<D>,
    left_idx_1: OpArg,
    left_idx_2: OpArg,
    right_idx: OpArg,
) -> Result<StepResult, IntcodeError> {
    let op_1 = vm.read(left_idx_1)?;
    let op_2 = vm.read(left_idx_2)?;

    *vm.write(right_idx)? = op_1 * op_2;

    Ok(StepResult::Continue)
}

fn intcode_op_input<D: DerefMut<Target = [i64]>>(
    vm: &mut IntcodeVM<D>,
    store_idx: OpArg,
) -> Result<StepResult, IntcodeError> {
    // Ensure that the value can be stored to to begin with
    vm.write(store_idx)?;

    // Set OpArg state so that
    vm.state = VMState::WaitingForInput(store_idx);
    Ok(StepResult::Interrupt(InterruptReason::WaitingForInput))
}

fn intcode_op_output<D: DerefMut<Target = [i64]>>(
    vm: &mut IntcodeVM<D>,
    read_idx: OpArg,
) -> Result<StepResult, IntcodeError> {
    // Ensure that the value can be accessed to to begin with
    vm.read(read_idx)?;

    // Set VM state so that
    vm.state = VMState::WaitingForOutput(read_idx);
    Ok(StepResult::Interrupt(InterruptReason::WaitingForOutput))
}

fn intcode_op_jump_test<D: DerefMut<Target = [i64]>, F: Fn(i64) -> bool>(
    vm: &mut IntcodeVM<D>,
    val: OpArg,
    jump: OpArg,
    test: F,
) -> Result<StepResult, IntcodeError> {
    let v = vm.read(val)?;

    if test(v) {
        let new_ip = vm.read(jump)?;
        if new_ip < 0 {
            return Err(IntcodeError::OutOfBoundsIp(new_ip));
        }

        vm.ip = new_ip as usize;
        Ok(StepResult::Jump)
    } else {
        Ok(StepResult::Continue)
    }
}

fn intcode_op_jump_if_true<D: DerefMut<Target = [i64]>>(
    vm: &mut IntcodeVM<D>,
    test: OpArg,
    jump: OpArg,
) -> Result<StepResult, IntcodeError> {
    intcode_op_jump_test(vm, test, jump, |v| v != 0)
}

fn intcode_op_jump_if_false<D: DerefMut<Target = [i64]>>(
    vm: &mut IntcodeVM<D>,
    test: OpArg,
    jump: OpArg,
) -> Result<StepResult, IntcodeError> {
    intcode_op_jump_test(vm, test, jump, |v| v == 0)
}

fn intcode_op_conditional_store<D: DerefMut<Target = [i64]>, F: Fn(i64, i64) -> bool>(
    vm: &mut IntcodeVM<D>,
    val_a: OpArg,
    val_b: OpArg,
    store: OpArg,
    test: F,
) -> Result<StepResult, IntcodeError> {
    let v_a = vm.read(val_a)?;
    let v_b = vm.read(val_b)?;

    *vm.write(store)? = if test(v_a, v_b) { 1 } else { 0 };

    Ok(StepResult::Continue)
}

fn intcode_op_less_than<D: DerefMut<Target = [i64]>>(
    vm: &mut IntcodeVM<D>,
    val_a: OpArg,
    val_b: OpArg,
    store: OpArg,
) -> Result<StepResult, IntcodeError> {
    intcode_op_conditional_store(vm, val_a, val_b, store, |a, b| a < b)
}

fn intcode_op_equals<D: DerefMut<Target = [i64]>>(
    vm: &mut IntcodeVM<D>,
    val_a: OpArg,
    val_b: OpArg,
    store: OpArg,
) -> Result<StepResult, IntcodeError> {
    intcode_op_conditional_store(vm, val_a, val_b, store, |a, b| a == b)
}

fn intcode_op_adjust_relative_base<D: DerefMut<Target = [i64]>>(
    vm: &mut IntcodeVM<D>,
    adj: OpArg,
) -> Result<StepResult, IntcodeError> {
    let adj_amount = vm.read(adj)?;

    vm.relative_base += adj_amount;

    Ok(StepResult::Continue)
}

fn intcode_op_terminate<D: DerefMut<Target = [i64]>>(
    _vm: &IntcodeVM<D>,
) -> Result<StepResult, IntcodeError> {
    Ok(StepResult::Interrupt(InterruptReason::Terminate))
}

impl<D: DerefMut<Target = [i64]>> IntcodeVM<D> {
    pub fn new(d: D) -> IntcodeVM<D> {
        IntcodeVM {
            ip: 0,
            data: d,
            state: VMState::Ready,
            relative_base: 0,
        }
    }

    pub fn input(&mut self, i: i64) -> Result<(), IntcodeError> {
        if let VMState::WaitingForInput(index) = self.state {
            *(self.write(index)?) = i;
            self.ip += 2;
            self.state = VMState::Ready;
            Ok(())
        } else {
            Err(IntcodeError::IllegalState)
        }
    }

    pub fn output(&mut self) -> Result<i64, IntcodeError> {
        if let VMState::WaitingForOutput(index) = self.state {
            let v = self.read(index)?;
            self.ip += 2;
            self.state = VMState::Ready;
            Ok(v)
        } else {
            Err(IntcodeError::IllegalState)
        }
    }

    pub fn is_ready(&self) -> bool {
        self.state == VMState::Ready
    }

    pub fn step(&mut self) -> Result<StepResult, IntcodeError> {
        let instruction = match self.data.get(self.ip) {
            Some(instruction) => instruction,
            None => return Err(IntcodeError::OutOfBoundsIp(self.ip as i64)),
        };

        let opcode = instruction % 100;
        let mut modecode = instruction / 100;

        match opcode {
            1 => proc::intcode_op!(modecode, intcode_op_add, 3),
            2 => proc::intcode_op!(modecode, intcode_op_mul, 3),
            3 => proc::intcode_op!(modecode, intcode_op_input, 1),
            4 => proc::intcode_op!(modecode, intcode_op_output, 1),
            5 => proc::intcode_op!(modecode, intcode_op_jump_if_true, 2),
            6 => proc::intcode_op!(modecode, intcode_op_jump_if_false, 2),
            7 => proc::intcode_op!(modecode, intcode_op_less_than, 3),
            8 => proc::intcode_op!(modecode, intcode_op_equals, 3),
            9 => proc::intcode_op!(modecode, intcode_op_adjust_relative_base, 1),
            99 => proc::intcode_op!(modecode, intcode_op_terminate, 0),
            _ => Err(IntcodeError::IllegalOpcode(*instruction)),
        }
    }

    pub fn run(&mut self) -> Result<InterruptReason, IntcodeError> {
        if self.state != VMState::Ready {
            return Err(IntcodeError::IllegalState);
        }

        loop {
            let step_result = match self.step() {
                Ok(r) => r,
                Err(err) => {
                    // The VM needs to be permanently bricked
                    self.state = VMState::Fault;
                    return Err(err);
                }
            };

            if let StepResult::Interrupt(reason) = step_result {
                return Ok(reason);
            }
        }
    }

    pub fn data<'a>(&'a self) -> &'a D {
        return &self.data;
    }

    pub fn data_mut<'a>(&'a mut self) -> &'a mut D {
        return &mut self.data;
    }

    fn read(&self, arg: OpArg) -> Result<i64, IntcodeError> {
        match arg {
            OpArg::Position(index) => read_index(&self.data, index),
            OpArg::Relative(index) => read_index(&self.data, index + self.relative_base),
            OpArg::Immediate(value) => Ok(value),
        }
    }

    fn write(&mut self, arg: OpArg) -> Result<&mut i64, IntcodeError> {
        match arg {
            OpArg::Position(index) => read_index_mut(&mut self.data, index),
            OpArg::Relative(index) => read_index_mut(&mut self.data, index + self.relative_base),
            OpArg::Immediate(_) => Err(IntcodeError::IllegalStore),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intcode_case1() {
        let d: Vec<i64> = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];

        let mut vm = IntcodeVM::new(d);
        vm.run().unwrap();
        assert_eq!(vm.data()[3], 70);
        assert_eq!(vm.data()[0], 3500);
    }

    #[test]
    fn test_intcode_case2() {
        let d: Vec<i64> = vec![1, 0, 0, 0, 99];
        let mut vm = IntcodeVM::new(d);
        vm.run().unwrap();
        assert_eq!(vm.data()[0], 2);
    }

    #[test]
    fn test_intcode_case3() {
        let d: Vec<i64> = vec![2, 3, 0, 3, 99];
        let mut vm = IntcodeVM::new(d);
        vm.run().unwrap();
        assert_eq!(vm.data()[3], 6);
    }

    #[test]
    fn test_intcode_case4() {
        let d: Vec<i64> = vec![2, 4, 4, 5, 99, 0];
        let mut vm = IntcodeVM::new(d);
        vm.run().unwrap();
        assert_eq!(vm.data()[5], 9801);
    }

    #[test]
    fn test_intcode_case5() {
        let d: Vec<i64> = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let mut vm = IntcodeVM::new(d);
        vm.run().unwrap();
        assert_eq!(vm.data()[0], 30);
    }

    struct IOProg {
        p: Vec<i64>,
        i: i64,
        o: i64,
    }

    fn run_prog(p: Vec<i64>, i: i64, o: i64) {
        let mut vm = IntcodeVM::new(p);
        assert_eq!(vm.run().unwrap(), InterruptReason::WaitingForInput);
        vm.input(i).unwrap();
        let a = vm.run();
        assert_eq!(a.unwrap(), InterruptReason::WaitingForOutput);
        assert_eq!(vm.output().unwrap(), o);
        assert_eq!(vm.run().unwrap(), InterruptReason::Terminate);
    }

    #[test]
    fn test_intcode_inputoutput() {
        run_prog(vec![3, 0, 4, 0, 99], 5, 5);
    }

    #[test]
    fn test_intcode_intermediate_mode() {
        let d: Vec<i64> = vec![1002, 4, 3, 4, 33];
        let mut vm = IntcodeVM::new(d);
        assert_eq!(vm.run().unwrap(), InterruptReason::Terminate);
    }

    #[test]
    fn test_intcode_jump_and_cond_store() {
        run_prog(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], 8, 1);
        run_prog(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], 7, 0);

        run_prog(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], 7, 1);
        run_prog(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], 8, 0);

        run_prog(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], 8, 1);
        run_prog(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], 8, 1);

        run_prog(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], 7, 1);
        run_prog(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], 8, 0);

        run_prog(
            vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            0,
            0,
        );
        run_prog(
            vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            -1,
            1,
        );

        run_prog(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1], 0, 0);
        run_prog(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1], -1, 1);

        run_prog(
            vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ],
            7,
            999,
        );
        run_prog(
            vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ],
            8,
            1000,
        );
        run_prog(
            vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ],
            9,
            1001,
        );
    }
}
