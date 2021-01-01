use std::collections::HashSet;

use day23::{Argument, Instruction, Opcode};

fn main() {
    println!("#include <stdio.h>");
    println!("int main(void) {{");

    println!("long a = 0, b = 0, c = 0, d = 0, e = 0, f = 0, g = 0, h = 0;");

    let program: Box<[Instruction]> = include_str!("../input.txt")
        .lines()
        .map(Instruction::from_input)
        .collect();

    let jump_points: HashSet<_> = program
        .iter()
        .enumerate()
        .filter(|(_, instr)| instr.opcode == Opcode::Jnz)
        .map(|(pc, instr)| {
            (pc as i64
                + match instr.y {
                    Argument::Immediate(off) => off,
                    Argument::Register(_) => unreachable!(),
                }) as usize
        })
        .collect();

    for (pc, Instruction { opcode, x, y }) in program.iter().enumerate() {
        if jump_points.contains(&pc) {
            print!("pc{}: ", pc);
        }

        match opcode {
            Opcode::Set => print!("{} = {}", x, y),
            Opcode::Sub => print!("{} -= {}", x, y),
            Opcode::Mul => print!("{} *= {}", x, y),
            Opcode::Jnz => {
                let dest = (pc as i64
                    + match y {
                        Argument::Immediate(off) => off,
                        Argument::Register(_) => unreachable!(),
                    }) as usize;

                if dest < program.len() {
                    print!("if ({}) goto pc{}", x, dest)
                } else {
                    print!("if ({}) goto out", x);
                }
            }
        }

        println!(";");
    }

    println!("out: printf(\"%ld\\n\", h);");

    println!("}}");
}
