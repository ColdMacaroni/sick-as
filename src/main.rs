use std::error::Error;
use std::{env, fmt::Display, fs};

use nom::{
    bytes::complete::{tag, take_while1},
    character::{complete::char, is_alphabetic, is_digit},
    combinator::opt,
    sequence::tuple,
};
/*
* Instructions:
* set a, -> m
* add a, a -> m
* sub a, a -> m
* out a
* num a
* cin -> m
* nin -> m
* bak a, a
*/

#[derive(Debug)]
enum Value {
    Memory { addr: u8 },
    Literal { val: u8 },
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Memory { addr } => write!(f, "m{}", addr),
            Value::Literal { val } => write!(f, "{}", val),
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Set {
        src: Value,
        tgt: Value,
    },
    Add {
        left: Value,
        right: Value,
        tgt: Value,
    },
    Sub {
        left: Value,
        right: Value,
        tgt: Value,
    },
    Out {
        src: Value,
    },
    Num {
        src: Value,
    },
    Cin {
        tgt: Value,
    },
    Nin {
        tgt: Value,
    },
    Bak {
        count: Value,
        check: Value,
    },
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Set { src, tgt } => write!(f, "set {} -> {}", src, tgt),
            Instruction::Add { left, right, tgt } => write!(f, "add {}, {} -> {}", left, right, tgt),
            Instruction::Sub { left, right, tgt } => todo!(),
            Instruction::Out { src } => todo!(),
            Instruction::Num { src } => todo!(),
            Instruction::Cin { tgt } => todo!(),
            Instruction::Nin { tgt } => todo!(),
            Instruction::Bak { count, check } => todo!(),
        }
    }
}

/// Little convenience function for converting strs into u8s and being able to use a ?
#[inline]
fn str_to_u8(s: &str) -> Result<u8, &'static str> {
    match u8::from_str_radix(s, 10) {
        Ok(val) => Ok(val),
        Err(_) => Err("Error parsing integer"),
    }
}

fn parse_instruction(input: &str) -> Result<Instruction, String> {
    let word = take_while1(|c: char| is_alphabetic(c as u8));
    let num = take_while1(|c: char| is_digit(c as u8));
    let mem = char::<&str, nom::error::Error<&str>>('m');
    let space = take_while1(|c| c == ' ');
    let sep = tuple((opt(&space), char(','), opt(&space)));
    let arrow = tuple((opt(&space), tag("->"), opt(&space)));

    // let (input, (name, _, _, has_mem, num)) = tuple((word, space, mem, num, space, mem, num))(input)?;
    let (input, (name, _)) = match tuple((word, &space))(input) {
        Ok(val) => val,
        Err(e) => return Err(format!("Could not find valid instruction name: `{:?}`", e.source())),
    };

    let inst = match name {
        "set" => {
            let (_input, (is_mem, src, _, _, tgt)) =
                match tuple((opt(&mem), &num, arrow, &mem, &num))(input) {
                    Ok(val) => val,
                    Err(e) => return Err(format!("Error while parsing set instruction near `{:?}`", e.source())),
                };
            let src = str_to_u8(src)?;
            let tgt = str_to_u8(tgt)?;

            Instruction::Set {
                src: match is_mem {
                    Some(_) => Value::Memory { addr: src },
                    None => Value::Literal { val: src },
                },
                tgt: Value::Memory { addr: tgt },
            }
        }
        "add" => {
            let (_input, (is_mem_l, src_l, _, is_mem_r, src_r, _, _, tgt)) =
                match tuple((opt(&mem), &num, sep, opt(&mem), &num, arrow, &mem, &num ))(input) {
                    Ok(val) => val,
                    Err(e) => return Err(format!("Error while parsing add instruction near `{:?}`", e.source())),
                };
            let src_l = str_to_u8(src_l)?;
            let src_r = str_to_u8(src_r)?;
            let tgt = str_to_u8(tgt)?;

            Instruction::Add {
                left: match is_mem_l {
                    Some(_) => Value::Memory { addr: src_l },
                    None => Value::Literal { val: src_l },
                },
                right: match is_mem_r {
                    Some(_) => Value::Memory { addr: src_r },
                    None => Value::Literal { val: src_r },
                },
                tgt: Value::Memory { addr: tgt },
            }
        }
        "sub" => {
            todo!()
        }
        "out" => {
            todo!()
        }
        "num" => {
            todo!()
        }
        "cin" => {
            todo!()
        }
        "nin" => {
            todo!()
        }
        "bak" => {
            todo!()
        }

        _ => return Err(format!("Unknown instruction `{name}`")),
    };

    Ok(inst)
}

fn get_instructions(path: &str) -> Result<Vec<Instruction>, String> {
    let mut instructions = Vec::new();

    if let Ok(content) = fs::read_to_string(path) {
        for (idx, line) in content.split("\n").enumerate() {
            // Let's ignore comments
            if let Some('#') = line.chars().next() {
                continue;
            }
            if line.len() == 0 {
                continue;
            };

            match parse_instruction(line) {
                Ok(inst) => instructions.push(inst),
                Err(val) => return Err(format!("Error on line {}: {}", idx + 1, val)),
            }
        }
    } else {
        return Err("Could not read file".to_owned());
    }

    Ok(instructions)
}

fn main() -> Result<(), String> {
    // Get instructions first
    // Skip this filename
    let instructions = match env::args().skip(1).next() {
        Some(file) => match get_instructions(&file) {
            Ok(insts) => insts,
            Err(msg) => return Err(msg),
        },
        None => return Err("Please provide a filename as an argument".to_owned()),
    };

    for i in instructions {
        println!("{}", i);
    }

    let mut memory: [u8; 255] = [0; 255];

    Ok(())
}
