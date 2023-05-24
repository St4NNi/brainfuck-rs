use anyhow::Result;
use std::{
    collections::HashMap,
    fs,
    io::{self, Read},
    num::Wrapping,
};

struct Brainfuck {
    tape: Vec<Wrapping<u8>>,
    instruction_pointer: usize,
    data_pointer: usize,
    code: Vec<Instruction>,
    matched_loops: HashMap<usize, usize>,
}

#[derive(Clone)]
enum Instruction {
    IncrementDatapointer, // Increment Datapointer >
    DecrementDatapointer, // Decrement Datapointer <
    IncrementValue,       // Increment value +
    DecrementValue,       // Decrement value -
    Output,               // Output current value as ascii to stdout
    Input,                // Read 1 byte from stdin
    BeginLoop,            // Begin Loop
    EndLoop,              // End Loop
}

impl Brainfuck {
    fn new(instructions: Vec<Instruction>, matched_loops: HashMap<usize, usize>) -> Self {
        Brainfuck {
            tape: vec![Wrapping(0u8); 30000],
            instruction_pointer: 0,
            data_pointer: 0,
            code: instructions,
            matched_loops,
        }
    }
    fn run(&mut self) {
        loop {
            self.process_instruction();
            if self.instruction_pointer >= self.code.len() {
                break;
            }
        }
    }
    fn process_instruction(&mut self) {
        let instruction = &self.code[self.instruction_pointer];
        match instruction {
            Instruction::IncrementDatapointer => self.data_pointer += 1,
            Instruction::DecrementDatapointer => self.data_pointer -= 1,
            Instruction::IncrementValue => self.tape[self.data_pointer] += 1,
            Instruction::DecrementValue => self.tape[self.data_pointer] -= 1,
            Instruction::Output => print!(
                "{}",
                std::str::from_utf8(&[self.tape[self.data_pointer].0]).unwrap_or_default()
            ),
            Instruction::Input => {
                self.tape[self.data_pointer] =
                    Wrapping(io::stdin().bytes().next().unwrap().unwrap());
            }
            Instruction::BeginLoop => {
                if self.tape[self.data_pointer].0 == 0u8 {
                    self.instruction_pointer =
                        *self.matched_loops.get(&self.instruction_pointer).unwrap();
                }
            }
            Instruction::EndLoop => {
                if self.tape[self.data_pointer].0 != 0u8 {
                    self.instruction_pointer =
                        *self.matched_loops.get(&self.instruction_pointer).unwrap();
                }
            }
        }
        self.instruction_pointer += 1;
    }
}

fn build_matched_loops(code: Vec<Instruction>) -> HashMap<usize, usize> {
    let mut tempstack = Vec::new();
    let mut retmap = HashMap::new();
    for (index, instruction) in code.iter().enumerate() {
        match instruction {
            Instruction::BeginLoop => tempstack.push(index),
            Instruction::EndLoop => {
                let matching = tempstack.pop().unwrap();
                retmap.insert(matching, index);
                retmap.insert(index, matching);
            }
            _ => (),
        }
    }
    retmap
}

fn lex_file(filename: &str) -> Result<(Vec<Instruction>, HashMap<usize, usize>)> {
    let contents = fs::read_to_string(filename).unwrap();
    let mut codes = Vec::new();
    for char in contents.chars() {
        match char {
            '>' => codes.push(Instruction::IncrementDatapointer),
            '<' => codes.push(Instruction::DecrementDatapointer),
            '+' => codes.push(Instruction::IncrementValue),
            '-' => codes.push(Instruction::DecrementValue),
            '.' => codes.push(Instruction::Output),
            ',' => codes.push(Instruction::Input),
            '[' => codes.push(Instruction::BeginLoop),
            ']' => codes.push(Instruction::EndLoop),
            _ => (),
        }
    }
    let matched_lps = build_matched_loops(codes.clone());
    Ok((codes, matched_lps))
}

fn main() {
    let (codes, matched_lps) = lex_file("test.bf").unwrap();
    let mut bf = Brainfuck::new(codes, matched_lps);
    bf.run();
}
