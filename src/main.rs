use nom::{
    bytes::complete::{tag, take_while1},
    character::{complete::char, is_alphabetic, is_digit},
    combinator::opt,
    sequence::tuple,
};
use std::num::Wrapping;
use std::{env, fmt::Display, fs};
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
* fwd a, a
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
    Fwd {
        count: Value,
        check: Value,
    },
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Set { src, tgt } => write!(f, "set {} -> {}", src, tgt),
            Instruction::Add { left, right, tgt } => {
                write!(f, "add {}, {} -> {}", left, right, tgt)
            }
            Instruction::Sub { left, right, tgt } => {
                write!(f, "sub {}, {} -> {}", left, right, tgt)
            }
            Instruction::Out { src } => write!(f, "out {}", src),
            Instruction::Num { src } => write!(f, "num {}", src),
            Instruction::Cin { tgt } => write!(f, "cin -> {}", tgt),
            Instruction::Nin { tgt } => write!(f, "nin -> {}", tgt),
            Instruction::Bak { count, check } => write!(f, "bak {}, {}", count, check),
            Instruction::Fwd { count, check } => write!(f, "fwd {}, {}", count, check),
        }
    }
}

/// Little convenience function for converting strs into u8s and being able to use a ?
#[inline]
fn str_to_u8(s: &str) -> Result<u8, &'static str> {
    match s.parse::<u8>() {
        Ok(val) => Ok(val),
        Err(_) => Err("Error parsing integer"),
    }
}

fn parse_instruction(input: &str) -> Result<Instruction, String> {
    // Matches any alpha word
    let word = take_while1(|c: char| is_alphabetic(c as u8));
    // Matches any series of numbers
    let num = take_while1(|c: char| is_digit(c as u8));
    // Matches just 'm'
    let mem = char::<&str, nom::error::Error<&str>>('m');
    // Matches any number of spaces
    let space = take_while1(|c| c == ' ');
    // Matches a comma with any number of spaces on either side
    let sep = tuple((opt(&space), char(','), opt(&space)));
    // Matches an arrow with any number of spaces on either side
    let arrow = tuple((opt(&space), tag("->"), opt(&space)));

    let (input, (name, _)) = match tuple((word, &space))(input) {
        Ok(val) => val,
        Err(nom::Err::Error(nom::error::Error { input, .. }))
        | Err(nom::Err::Failure(nom::error::Error { input, .. })) => {
            return Err(format!("Not a valid instruction name: `{}`", input))
        }
        Err(nom::Err::Incomplete(_)) => {
            return Err("Error while parsing instruction name, incomplete data.".to_owned())
        }
    };

    let inst = match name {
        "set" => {
            let (input, (is_mem, src, _, _, tgt, _)) =
                match tuple((opt(&mem), &num, arrow, &mem, &num, opt(&space)))(input) {
                    Ok(val) => val,
                    Err(nom::Err::Error(nom::error::Error { input, .. }))
                    | Err(nom::Err::Failure(nom::error::Error { input, .. })) => {
                        return Err(format!(
                            "Error while parsing set instruction near `{}`",
                            input
                        ))
                    }
                    Err(nom::Err::Incomplete(_)) => {
                        return Err("Error while parsing `set`, incomplete data.".to_owned())
                    }
                };
            if !input.is_empty() {
                return Err(format!("Unexpected characters: `{}`", input))
            }
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
            let (input, (is_mem_l, src_l, _, is_mem_r, src_r, _, _, tgt, _)) =
                match tuple((opt(&mem), &num, sep, opt(&mem), &num, arrow, &mem, &num, opt(&space)))(input) {
                    Ok(val) => val,
                    Err(nom::Err::Error(nom::error::Error { input, .. }))
                    | Err(nom::Err::Failure(nom::error::Error { input, .. })) => {
                        return Err(format!(
                            "Error while parsing `add` instruction near `{}`",
                            input
                        ))
                    }
                    Err(nom::Err::Incomplete(_)) => {
                        return Err("Error while parsing `add`, incomplete data.".to_owned())
                    }
                };
            if !input.is_empty() {
                return Err(format!("Unexpected characters: `{}`", input))
            }
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
            let (input, (is_mem_l, src_l, _, is_mem_r, src_r, _, _, tgt, _)) =
                match tuple((opt(&mem), &num, sep, opt(&mem), &num, arrow, &mem, &num, opt(&space)))(input) {
                    Ok(val) => val,
                    Err(nom::Err::Error(nom::error::Error { input, .. }))
                    | Err(nom::Err::Failure(nom::error::Error { input, .. })) => {
                        return Err(format!(
                            "Error while parsing `sub` instruction near `{}`",
                            input
                        ))
                    }
                    Err(nom::Err::Incomplete(_)) => {
                        return Err("Error while parsing `sub`, incomplete data.".to_owned())
                    }
                };
            if !input.is_empty() {
                return Err(format!("Unexpected characters: `{}`", input))
            }
            let src_l = str_to_u8(src_l)?;
            let src_r = str_to_u8(src_r)?;
            let tgt = str_to_u8(tgt)?;

            Instruction::Sub {
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
        "out" => {
            let (input, (is_mem, src, _)) = match tuple((opt(&mem), &num, opt(&space)))(input) {
                Ok(val) => val,
                Err(nom::Err::Error(nom::error::Error { input, .. }))
                | Err(nom::Err::Failure(nom::error::Error { input, .. })) => {
                    return Err(format!(
                        "Error while parsing `out` instruction near `{}`",
                        input
                    ))
                }
                Err(nom::Err::Incomplete(_)) => {
                    return Err("Error while parsing `out`, incomplete data.".to_owned())
                }
            };
            if !input.is_empty() {
                return Err(format!("Unexpected characters: `{}`", input))
            }

            let src = str_to_u8(src)?;

            Instruction::Out {
                src: match is_mem {
                    Some(_) => Value::Memory { addr: src },
                    None => Value::Literal { val: src },
                },
            }
        }
        "num" => {
            let (input, (is_mem, src, _)) = match tuple((opt(&mem), &num, opt(&space)))(input) {
                Ok(val) => val,
                Err(nom::Err::Error(nom::error::Error { input, .. }))
                | Err(nom::Err::Failure(nom::error::Error { input, .. })) => {
                    return Err(format!(
                        "Error while parsing `num` instruction near `{}`",
                        input
                    ))
                }
                Err(nom::Err::Incomplete(_)) => {
                    return Err("Error while parsing `num`, incomplete data.".to_owned())
                }
            };
            if !input.is_empty() {
                return Err(format!("Unexpected characters: `{}`", input))
            }

            let src = str_to_u8(src)?;

            Instruction::Num {
                src: match is_mem {
                    Some(_) => Value::Memory { addr: src },
                    None => Value::Literal { val: src },
                },
            }
        }
        "cin" => {
            let (input, (_, _, addr, _)) = match tuple((arrow, &mem, &num, opt(&space)))(input) {
                Ok(val) => val,
                Err(nom::Err::Error(nom::error::Error { input, .. }))
                | Err(nom::Err::Failure(nom::error::Error { input, .. })) => {
                    return Err(format!(
                        "Error while parsing `cin` instruction near `{}`",
                        input
                    ))
                }
                Err(nom::Err::Incomplete(_)) => {
                    return Err("Error while parsing `cin`, incomplete data.".to_owned())
                }
            };
            if !input.is_empty() {
                return Err(format!("Unexpected characters: `{}`", input))
            }

            let addr = str_to_u8(addr)?;

            Instruction::Cin {
                tgt: Value::Memory { addr },
            }
        }
        "nin" => {
            let (input, (_, _, addr, _)) = match tuple((arrow, &mem, &num, opt(&space)))(input) {
                Ok(val) => val,
                Err(nom::Err::Error(nom::error::Error { input, .. }))
                | Err(nom::Err::Failure(nom::error::Error { input, .. })) => {
                    return Err(format!(
                        "Error while parsing `nin` instruction near `{}`",
                        input
                    ))
                }
                Err(nom::Err::Incomplete(_)) => {
                    return Err("Error while parsing `nin`, incomplete data.".to_owned())
                }
            };
            if !input.is_empty() {
                return Err(format!("Unexpected characters: `{}`", input))
            }

            let addr = str_to_u8(addr)?;

            Instruction::Nin {
                tgt: Value::Memory { addr },
            }
        }
        "bak" => {
            let (input, (is_mem_count, count, _, is_mem_check, check, _)) =
                match tuple((opt(&mem), &num, sep, opt(&mem), &num, opt(&space)))(input) {
                    Ok(val) => val,
                    Err(nom::Err::Error(nom::error::Error { input, .. }))
                    | Err(nom::Err::Failure(nom::error::Error { input, .. })) => {
                        return Err(format!(
                            "Error while parsing `bak` instruction near `{}`",
                            input
                        ))
                    }
                    Err(nom::Err::Incomplete(_)) => {
                        return Err("Error while parsing `bak`, incomplete data.".to_owned())
                    }
                };
            if !input.is_empty() {
                return Err(format!("Unexpected characters: `{}`", input))
            }

            let count = str_to_u8(count)?;
            let check = str_to_u8(check)?;

            Instruction::Bak {
                count: match is_mem_count {
                    Some(_) => Value::Memory { addr: count },
                    None => Value::Literal { val: count },
                },
                check: match is_mem_check {
                    Some(_) => Value::Memory { addr: check },
                    None => Value::Literal { val: check },
                },
            }
        }

        "fwd" => {
            let (input, (is_mem_count, count, _, is_mem_check, check, _)) =
                match tuple((opt(&mem), &num, sep, opt(&mem), &num, opt(&space)))(input) {
                    Ok(val) => val,
                    Err(nom::Err::Error(nom::error::Error { input, .. }))
                    | Err(nom::Err::Failure(nom::error::Error { input, .. })) => {
                        return Err(format!(
                            "Error while parsing `fwd` instruction near `{}`",
                            input
                        ))
                    }
                    Err(nom::Err::Incomplete(_)) => {
                        return Err("Error while parsing `fwd`, incomplete data.".to_owned())
                    }
                };
            if !input.is_empty() {
                return Err(format!("Unexpected characters: `{}`", input))
            }

            let count = str_to_u8(count)?;
            let check = str_to_u8(check)?;

            Instruction::Fwd {
                count: match is_mem_count {
                    Some(_) => Value::Memory { addr: count },
                    None => Value::Literal { val: count },
                },
                check: match is_mem_check {
                    Some(_) => Value::Memory { addr: check },
                    None => Value::Literal { val: check },
                },
            }
        }

        _ => return Err(format!("Unknown instruction `{name}`")),
    };

    Ok(inst)
}

fn get_instructions(path: &str) -> Result<Vec<Instruction>, String> {
    let mut instructions = Vec::new();

    if let Ok(content) = fs::read_to_string(path) {
        for (idx, line) in content.split('\n').enumerate() {
            // Let's ignore comments
            if let Some('#') = line.chars().next() {
                continue;
            }
            if line.is_empty() {
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
    let instructions = match env::args().nth(1) {
        Some(file) => match get_instructions(&file) {
            Ok(insts) => insts,
            Err(msg) => return Err(msg),
        },
        None => return Err("Please provide a filename as an argument".to_owned()),
    };

    for i in instructions {
        println!("{}", i);
    }

    let mut memory: [Wrapping<u8>; 255] = [Wrapping(0u8); 255];

    Ok(())
}
