use std::io::{self, BufRead};

#[derive(Copy, Clone, Debug)]
struct Instruction {
    opcode: usize,
    a: usize,
    b: usize,
    c: usize,
}

impl Instruction {
    fn new(s: &str) -> Instruction {
        let mut iter = s
            .split(|c: char| !c.is_ascii_digit())
            .filter(|x| !x.is_empty())
            .map(|x| x.parse().unwrap());

        Instruction {
            opcode: iter.next().unwrap(),
            a: iter.next().unwrap(),
            b: iter.next().unwrap(),
            c: iter.next().unwrap(),
        }
    }
}

fn to_registers(s: &str) -> [usize; 4] {
    let mut registers = [0; 4];
    s
        .split(|c: char| !c.is_ascii_digit())
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .enumerate()
        .for_each(|(i, v)| registers[i] = v);

    registers

}

fn matches_three(ins: Instruction, input: [usize; 4], output: [usize; 4]) -> bool {
    let fns = [addr, addi, mulr, muli, banr, bani, bonr, boni,
        setr, seti, gtir, gtri, gtrr, eqir, eqri, eqrr];

    let mut count = 0;
    for f in fns.into_iter() {
        let mut input = input.clone();
        f(&ins, &mut input);
        if input == output {
            count += 1;
            if count >= 3 {
                return true;
            }
        }
    }

    false
}

fn addr(inst: &Instruction, reg: &mut [usize; 4]) {
    reg[inst.c] = reg[inst.a] + reg[inst.b];
}

fn addi(inst: &Instruction, reg: &mut [usize; 4]) {
    reg[inst.c] = reg[inst.a] + inst.b;
}

fn mulr(inst: &Instruction, reg: &mut [usize; 4]) {
    reg[inst.c] = reg[inst.a] * reg[inst.b];
}

fn muli(inst: &Instruction, reg: &mut [usize; 4]) {
    reg[inst.c] = reg[inst.a] * inst.b;
}

fn banr(inst: &Instruction, reg: &mut [usize; 4]) {
    reg[inst.c] = reg[inst.a] & reg[inst.b];
}

fn bani(inst: &Instruction, reg: &mut [usize; 4]) {
    reg[inst.c] = reg[inst.a] & inst.b;
}

fn bonr(inst: &Instruction, reg: &mut [usize; 4]) {
    reg[inst.c] = reg[inst.a] | reg[inst.b];
}

fn boni(inst: &Instruction, reg: &mut [usize; 4]) {
    reg[inst.c] = reg[inst.a] | inst.b;
}

fn setr(inst: &Instruction, reg: &mut [usize; 4]) {
    reg[inst.c] = reg[inst.a];
}

fn seti(inst: &Instruction, reg: &mut [usize; 4]) {
    reg[inst.c] = inst.a
}

fn gtir(inst: &Instruction, reg: &mut [usize; 4]) {
    reg[inst.c] = (inst.a > reg[inst.b]) as usize;
}

fn gtri(inst: &Instruction, reg: &mut [usize; 4]) {
    reg[inst.c] = (reg[inst.a] > inst.b) as usize;
}

fn gtrr(inst: &Instruction, reg: &mut [usize; 4]) {
    reg[inst.c] = (reg[inst.a] > reg[inst.b]) as usize;
}

fn eqir(inst: &Instruction, reg: &mut [usize; 4]) {
    reg[inst.c] = (inst.a == reg[inst.b]) as usize;
}

fn eqri(inst: &Instruction, reg: &mut [usize; 4]) {
    reg[inst.c] = (reg[inst.a] == inst.b) as usize;
}

fn eqrr(inst: &Instruction, reg: &mut [usize; 4]) {
    reg[inst.c] = (reg[inst.a] == reg[inst.b]) as usize;
}


fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().map(|x| x.unwrap()).collect::<Vec<_>>();
    let mut matches = 0;

    for chunk in input.chunks(4) {
        if chunk[0].is_empty() {
            break;
        }

        let mut before = to_registers(&chunk[0]);
        let instruction = Instruction::new(&chunk[1]);
        let mut after = to_registers(&chunk[2]);

        if matches_three(instruction, before, after) {
            matches += 1;
        }
    }

    println!("{:?}", matches);
}
