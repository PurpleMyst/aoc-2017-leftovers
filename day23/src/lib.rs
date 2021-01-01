use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum Argument {
    Immediate(i64),
    Register(usize),
}

impl Display for Argument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Argument::Immediate(x) => x.fmt(f),
            Argument::Register(x) => ((x as u8 + b'a') as char).fmt(f),
        }
    }
}

impl Argument {
    pub fn from_input(arg: &str) -> Self {
        if let Ok(im) = arg.parse() {
            Self::Immediate(im)
        } else {
            debug_assert!(matches!(arg.chars().next(), Some('a'..='h')));
            Self::Register((arg.as_bytes()[0] - b'a') as usize)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    Set,
    Sub,
    Mul,
    Jnz,
}

#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    pub opcode: Opcode,
    pub x: Argument,
    pub y: Argument,
}

impl Instruction {
    pub fn from_input(line: &str) -> Self {
        let mut parts = line.splitn(3, ' ');

        let opcode = match parts.next().unwrap() {
            "set" => Opcode::Set,
            "sub" => Opcode::Sub,
            "mul" => Opcode::Mul,
            "jnz" => Opcode::Jnz,
            _ => unreachable!(),
        };

        let mut args = parts.map(Argument::from_input);

        Self {
            opcode,
            x: args.next().unwrap(),
            y: args.next().unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Coprocessor {
    registers: [i64; 8],
    pc: usize,
    program: Vec<Instruction>,

    muls_executed: usize,
}

impl Coprocessor {
    pub fn from_input(input: &str) -> Self {
        Self {
            registers: [0; 8],
            pc: 0,
            program: input.lines().map(Instruction::from_input).collect(),

            muls_executed: 0,
        }
    }

    fn get(&self, arg: Argument) -> i64 {
        match arg {
            Argument::Immediate(val) => val,
            Argument::Register(r) => self.registers[r],
        }
    }

    fn get_mut(&mut self, arg: Argument) -> &mut i64 {
        match arg {
            Argument::Immediate(_) => unreachable!(),
            Argument::Register(r) => &mut self.registers[r],
        }
    }

    fn exec_one(&mut self) -> bool {
        let Instruction { opcode, x, y } = match self.program.get(self.pc) {
            Some(&instr) => instr,
            None => return false,
        };

        let y = self.get(y);

        match opcode {
            Opcode::Set => {
                *self.get_mut(x) = y;
                self.pc += 1;
            }

            Opcode::Sub => {
                *self.get_mut(x) -= y;
                self.pc += 1;
            }

            Opcode::Mul => {
                self.muls_executed += 1;
                *self.get_mut(x) *= y;
                self.pc += 1;
            }

            Opcode::Jnz => {
                if self.get(x) != 0 {
                    self.pc = (self.pc as i64 + y) as usize;
                } else {
                    self.pc += 1;
                }
            }
        }

        true
    }

    fn exec(&mut self) {
        while self.exec_one() {}
    }

    pub fn reset(&mut self) {
        self.pc = 0;
        self.muls_executed = 0;
        self.registers = [0; 8];
    }
}

#[inline]
pub fn solve_part1(coprocessor: &mut Coprocessor) -> usize {
    coprocessor.exec();
    coprocessor.muls_executed
}

#[inline]
pub fn solve_part2(coprocessor: &mut Coprocessor) -> usize {
    // Remove everything but the initialization sequenc
    let init_sz = coprocessor
        .program
        .iter()
        .position(|instr| {
            matches!(
                instr,
                Instruction {
                    opcode: Opcode::Set,
                    x: Argument::Register(5),
                    y: Argument::Immediate(1),
                }
            )
        })
        .unwrap();
    coprocessor.program.drain(init_sz..);

    // Initialize the registers and get the start/end
    coprocessor.reset();
    coprocessor.registers[0] = 1;
    coprocessor.exec();
    let start = coprocessor.registers[1] as usize;
    let end = coprocessor.registers[2] as usize;

    // Count composite numbers between start and end
    let primes = concurrent_prime_sieve::filter::prime_filter(end + 1);
    (start..=end).step_by(17).filter(|&n| !primes[n]).count()
}

#[inline]
pub fn load_input() -> Coprocessor {
    Coprocessor::from_input(include_str!("input.txt"))
}

#[inline]
pub fn solve() -> (usize, usize) {
    let mut coprocessor = Coprocessor::from_input(include_str!("input.txt"));

    (solve_part1(&mut coprocessor), solve_part2(&mut coprocessor))
}
