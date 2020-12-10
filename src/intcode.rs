use std::error::Error;
use std::fmt;
use std::ops::{Deref, DerefMut};

extern crate memmap;
extern crate proc;

enum VMState {
    Ready,
    Fault,
    WaitingForInput(SomeOutOpArg),
    WaitingForOutput(SomeInOpArg),
}

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

#[derive(Clone, Copy)]
enum SomeInOpArg {
    Position(PositionInput),
    Immediate(ImmediateInput),
    Relative(RelativeInput),
}

trait InOpArg {
    fn read<T>(&self, vm: &IntcodeVM<T>) -> Result<i64, IntcodeError>
    where
        T: DerefMut<Target = [i64]>;
    fn to_enum(self) -> SomeInOpArg;
    /*fn from(i: i64) -> Self; This is just for proc...*/
}

#[derive(Clone, Copy)]
struct PositionInput {
    i: i64,
}

impl InOpArg for PositionInput {
    fn read<T>(&self, vm: &IntcodeVM<T>) -> Result<i64, IntcodeError>
    where
        T: DerefMut<Target = [i64]>,
    {
        read_index(&vm.data, self.i)
    }

    fn to_enum(self) -> SomeInOpArg {
        SomeInOpArg::Position(self)
    }
}

impl PositionInput {
    fn from(i: i64) -> Self {
        Self { i }
    }
}

#[derive(Clone, Copy)]
struct ImmediateInput {
    i: i64,
}

impl InOpArg for ImmediateInput {
    fn read<T>(&self, _: &IntcodeVM<T>) -> Result<i64, IntcodeError>
    where
        T: DerefMut<Target = [i64]>,
    {
        Ok(self.i)
    }

    fn to_enum(self) -> SomeInOpArg {
        SomeInOpArg::Immediate(self)
    }
}

impl ImmediateInput {
    fn from(i: i64) -> Self {
        Self { i }
    }
}

#[derive(Clone, Copy)]
struct RelativeInput {
    i: i64,
}

impl InOpArg for RelativeInput {
    fn read<T>(&self, vm: &IntcodeVM<T>) -> Result<i64, IntcodeError>
    where
        T: DerefMut<Target = [i64]>,
    {
        read_index(&vm.data, vm.relative_base + self.i)
    }

    fn to_enum(self) -> SomeInOpArg {
        SomeInOpArg::Relative(self)
    }
}

impl RelativeInput {
    fn from(i: i64) -> Self {
        Self { i }
    }
}

#[derive(Clone, Copy)]
enum SomeOutOpArg {
    Position(PositionOutput),
    Relative(RelativeOutput),
}

trait OutOpArg {
    fn write<'a, T>(&self, vm: &'a mut IntcodeVM<T>) -> Result<&'a mut i64, IntcodeError>
    where
        T: DerefMut<Target = [i64]>;
    fn to_enum(self) -> SomeOutOpArg;
    /*fn from(i: i64) -> Self;*/
}

#[derive(Clone, Copy)]
struct PositionOutput {
    i: i64,
}

impl OutOpArg for PositionOutput {
    fn write<'a, T>(&self, vm: &'a mut IntcodeVM<T>) -> Result<&'a mut i64, IntcodeError>
    where
        T: DerefMut<Target = [i64]>,
    {
        read_index_mut(&mut vm.data, self.i)
    }

    fn to_enum(self) -> SomeOutOpArg {
        SomeOutOpArg::Position(self)
    }
}

impl PositionOutput {
    fn from(i: i64) -> Self {
        Self { i }
    }
}

#[derive(Clone, Copy)]
struct RelativeOutput {
    i: i64,
}

impl OutOpArg for RelativeOutput {
    fn write<'a, T>(&self, vm: &'a mut IntcodeVM<T>) -> Result<&'a mut i64, IntcodeError>
    where
        T: DerefMut<Target = [i64]>,
    {
        read_index_mut(&mut vm.data, vm.relative_base + self.i)
    }

    fn to_enum(self) -> SomeOutOpArg {
        SomeOutOpArg::Relative(self)
    }
}
impl RelativeOutput {
    fn from(i: i64) -> Self {
        Self { i }
    }
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

fn intcode_op_add<D: DerefMut<Target = [i64]>, I1: InOpArg, I2: InOpArg, O: OutOpArg>(
    vm: &mut IntcodeVM<D>,
    i1: I1,
    i2: I2,
    o: O,
) -> Result<StepResult, IntcodeError> {
    let op_1 = i1.read(vm)?;
    let op_2 = i2.read(vm)?;

    *o.write(vm)? = op_1 + op_2;

    Ok(StepResult::Continue)
}

fn intcode_op_mul<D: DerefMut<Target = [i64]>, I1: InOpArg, I2: InOpArg, O: OutOpArg>(
    vm: &mut IntcodeVM<D>,
    i1: I1,
    i2: I2,
    o: O,
) -> Result<StepResult, IntcodeError> {
    let op_1 = i1.read(vm)?;
    let op_2 = i2.read(vm)?;

    *o.write(vm)? = op_1 * op_2;

    Ok(StepResult::Continue)
}

fn intcode_op_input<D: DerefMut<Target = [i64]>, O: OutOpArg>(
    vm: &mut IntcodeVM<D>,
    store_idx: O,
) -> Result<StepResult, IntcodeError> {
    vm.state = VMState::WaitingForInput(store_idx.to_enum());
    Ok(StepResult::Interrupt(InterruptReason::WaitingForInput))
}

fn intcode_op_output<D: DerefMut<Target = [i64]>, I: InOpArg>(
    vm: &mut IntcodeVM<D>,
    read_idx: I,
) -> Result<StepResult, IntcodeError> {
    vm.state = VMState::WaitingForOutput(read_idx.to_enum());
    Ok(StepResult::Interrupt(InterruptReason::WaitingForOutput))
}

fn intcode_op_jump_test<
    D: DerefMut<Target = [i64]>,
    I1: InOpArg,
    I2: InOpArg,
    F: Fn(i64) -> bool,
>(
    vm: &mut IntcodeVM<D>,
    val: I1,
    jump: I2,
    test: F,
) -> Result<StepResult, IntcodeError> {
    let v = val.read(&vm)?;

    if test(v) {
        let new_ip = jump.read(vm)?;
        if new_ip < 0 {
            return Err(IntcodeError::OutOfBoundsIp(new_ip));
        }

        vm.ip = new_ip as usize;
        Ok(StepResult::Jump)
    } else {
        Ok(StepResult::Continue)
    }
}

fn intcode_op_jump_if_true<D: DerefMut<Target = [i64]>, I1: InOpArg, I2: InOpArg>(
    vm: &mut IntcodeVM<D>,
    test: I1,
    jump: I2,
) -> Result<StepResult, IntcodeError> {
    intcode_op_jump_test(vm, test, jump, |v| v != 0)
}

fn intcode_op_jump_if_false<D: DerefMut<Target = [i64]>, I1: InOpArg, I2: InOpArg>(
    vm: &mut IntcodeVM<D>,
    test: I1,
    jump: I2,
) -> Result<StepResult, IntcodeError> {
    intcode_op_jump_test(vm, test, jump, |v| v == 0)
}

fn intcode_op_conditional_store<
    D: DerefMut<Target = [i64]>,
    I1: InOpArg,
    I2: InOpArg,
    O: OutOpArg,
    F: Fn(i64, i64) -> bool,
>(
    vm: &mut IntcodeVM<D>,
    val_a: I1,
    val_b: I2,
    store: O,
    test: F,
) -> Result<StepResult, IntcodeError> {
    let v_a = val_a.read(vm)?;
    let v_b = val_b.read(vm)?;

    *store.write(vm)? = if test(v_a, v_b) { 1 } else { 0 };

    Ok(StepResult::Continue)
}

fn intcode_op_less_than<D: DerefMut<Target = [i64]>, I1: InOpArg, I2: InOpArg, O: OutOpArg>(
    vm: &mut IntcodeVM<D>,
    val_a: I1,
    val_b: I2,
    store: O,
) -> Result<StepResult, IntcodeError> {
    intcode_op_conditional_store(vm, val_a, val_b, store, |a, b| a < b)
}

fn intcode_op_equals<D: DerefMut<Target = [i64]>, I1: InOpArg, I2: InOpArg, O: OutOpArg>(
    vm: &mut IntcodeVM<D>,
    val_a: I1,
    val_b: I2,
    store: O,
) -> Result<StepResult, IntcodeError> {
    intcode_op_conditional_store(vm, val_a, val_b, store, |a, b| a == b)
}

fn intcode_op_adjust_relative_base<D: DerefMut<Target = [i64]>, I: InOpArg>(
    vm: &mut IntcodeVM<D>,
    adj: I,
) -> Result<StepResult, IntcodeError> {
    let adj_amount = adj.read(vm)?;

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
            *match index {
                SomeOutOpArg::Position(p) => p.write(self),
                SomeOutOpArg::Relative(p) => p.write(self),
            }? = i;
            self.ip += 2;
            self.state = VMState::Ready;
            Ok(())
        } else {
            Err(IntcodeError::IllegalState)
        }
    }

    pub fn output(&mut self) -> Result<i64, IntcodeError> {
        if let VMState::WaitingForOutput(index) = &self.state {
            let v = match index {
                SomeInOpArg::Position(p) => p.read(self),
                SomeInOpArg::Immediate(p) => p.read(self),
                SomeInOpArg::Relative(p) => p.read(self),
            }?;
            self.ip += 2;
            self.state = VMState::Ready;
            Ok(v)
        } else {
            Err(IntcodeError::IllegalState)
        }
    }

    pub fn step(&mut self) -> Result<StepResult, IntcodeError> {
        let instruction = match self.data.get(self.ip) {
            Some(instruction) => instruction,
            None => return Err(IntcodeError::OutOfBoundsIp(self.ip as i64)),
        };

        proc::intcode_op!(
            10,
            instruction,
            1,
            intcode_op_add,
            2,
            1,
            2,
            intcode_op_mul,
            2,
            1,
            3,
            intcode_op_input,
            0,
            1,
            4,
            intcode_op_output,
            1,
            0,
            5,
            intcode_op_jump_if_true,
            2,
            0,
            6,
            intcode_op_jump_if_false,
            2,
            0,
            7,
            intcode_op_less_than,
            2,
            1,
            8,
            intcode_op_equals,
            2,
            1,
            9,
            intcode_op_adjust_relative_base,
            1,
            0,
            99,
            intcode_op_terminate,
            0,
            0
        )
    }

    pub fn run(&mut self) -> Result<InterruptReason, IntcodeError> {
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
