use petgraph::{algo::has_path_connecting, prelude::*};
use std::{convert::TryFrom, fs};

#[derive(Copy, Clone)]
enum Instruction {
    Nop,
    Acc,
    Jmp,
}

fn main() {
    let s = fs::read_to_string("day8/input").unwrap();
    let instructions = s
        .lines()
        .map(|l| {
            let mut parts = l.split(" ");
            let (instruction, value) = (
                parts.next().unwrap(),
                parts.next().unwrap().parse::<isize>().unwrap(),
            );
            (
                match instruction {
                    "nop" => Instruction::Nop,
                    "acc" => Instruction::Acc,
                    "jmp" => Instruction::Jmp,
                    _ => unreachable!(),
                },
                value,
            )
        })
        .collect::<Vec<_>>();
    let modified = (0..instructions.len())
        .find_map(|i| {
            let mut modified = instructions.clone();
            modified[i].0 = match modified[i].0 {
                Instruction::Nop => Instruction::Jmp,
                Instruction::Jmp => Instruction::Nop,
                x => x,
            };
            if has_path_connecting(
                &to_graph(&modified),
                0.into(),
                instructions.len().into(),
                None,
            ) {
                Some(modified)
            } else {
                None
            }
        })
        .unwrap();
    let (mut location, mut accumulator) = (0, 0);
    while location != modified.len() {
        match modified[location] {
            (Instruction::Nop, _) => {
                location += 1;
            }
            (Instruction::Acc, v) => {
                location += 1;
                accumulator += v;
            }
            (Instruction::Jmp, v) => {
                location = signed_add(location, v);
            }
        }
    }
    dbg!(accumulator);
}

fn to_graph(instructions: &[(Instruction, isize)]) -> DiGraph<(), (), usize> {
    DiGraph::from_edges(
        instructions
            .iter()
            .enumerate()
            .map(|(location, &(instruction, v))| match instruction {
                Instruction::Nop => (location, instructions.len().min(location + 1)),
                Instruction::Acc => (location, instructions.len().min(location + 1)),
                Instruction::Jmp => (location, instructions.len().min(signed_add(location, v))),
            }),
    )
}

fn signed_add(a: usize, b: isize) -> usize {
    if b > 0 {
        a + usize::try_from(b).unwrap()
    } else {
        a - usize::try_from(b.abs()).unwrap()
    }
}
