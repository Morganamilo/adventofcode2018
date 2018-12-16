use std::io::{self, BufRead};
use std::collections::{HashMap};

static OPS: &[fn(&Instruction, &mut [usize; 4])] =
    &[addr, addi, mulr, muli, banr, bani, bonr, boni,
    setr, seti, gtir, gtri, gtrr, eqir, eqri, eqrr];

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

fn filter(possible: &mut HashMap<usize, Vec<usize>>, ins: Instruction, input: [usize; 4], output: [usize; 4]) {
    for (i, f) in OPS.iter().enumerate() {
        let mut input = input.clone();
        f(&ins, &mut input);
        if input != output {
            possible.get_mut(&ins.opcode).unwrap().retain(|&x| x != i);
        }
    }

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
    let mut possible = HashMap::new();
    let mut part2 = 0;

    for op in 0..16 {
        possible.insert(op, (0..16).collect::<Vec<_>>());
    }

    for (i, chunk) in input.chunks(4).enumerate() {
        if chunk[0].is_empty() {
            part2 = i*4;
            break;
        }

        let mut before = to_registers(&chunk[0]);
        let instruction = Instruction::new(&chunk[1]);
        let mut after = to_registers(&chunk[2]);

        filter(&mut possible, instruction, before, after);
    }

    while possible.values().find(|v| v.len() > 1).is_some() {
        let to_filter = possible.values().filter(|v| v.len() == 1).map(|v| v[0]).collect::<Vec<_>>();

        for filter_by in to_filter {
            for v in possible.values_mut().filter(|v| v.len() > 1) {
                v.retain(|&x| x != filter_by);
            }
        }
    }

    let mut registers = [0; 4];
    for line in input[part2..].iter().filter(|l| !l.is_empty()) {
        let instruction = Instruction::new(&line);
        let x = possible.get(&instruction.opcode).unwrap()[0];
        let f = OPS[x];
        f(&instruction, &mut registers);
    }

    println!("{}", registers[0]);
}
