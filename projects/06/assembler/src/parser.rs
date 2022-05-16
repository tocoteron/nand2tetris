use crate::symbol::SymbolTable;

pub type Program = Vec<Instruction>;

#[derive(Debug)]
pub enum Instruction {
    AInstruction(AInstruction),
    BInstruction(BInstruction),
}

#[derive(Debug)]
pub struct AInstruction {
    pub value: u16,
}

#[derive(Debug)]
pub struct BInstruction {
    pub dest: BInstructionDest,
    pub comp: BInstructionComp,
    pub jump: BInstructionJump,
}

#[derive(Debug)]
pub enum BInstructionDest {
    Null,
    M,
    D,
    MD,
    A,
    AM,
    AD,
    AMD
}

#[derive(Debug)]
pub enum BInstructionComp {
    Zero,
    One,
    MinusOne,
    D,
    A,
    NotD,
    NotA,
    MinusD,
    MinusA,
    DPlusOne,
    APlusOne,
    DMinusOne,
    AMinusOne,
    DPlusA,
    DMinusA,
    AMinusD,
    DAndA,
    DOrA,
    M,
    NotM,
    MinusM,
    MPlusOne,
    MMinusOne,
    DPlusM,
    DMinusM,
    MMinusD,
    DAndM,
    DOrM,
}

#[derive(Debug)]
pub enum BInstructionJump {
    Null,
    JGT,
    JEQ,
    JGE,
    JLT,
    JNE,
    JLE,
    JMP,
}

pub fn parse_program(program: &Vec<String>, symbol_table: &SymbolTable) -> Program {
    program
        .iter()
        .map(clean_line)
        .filter(valid_line)
        .map(|line| parse_instruction(&line, symbol_table))
        .collect()
}

fn clean_line(line: &String) -> String {
    let mut cleaned_line = line.clone();
    let comment_offset = line.find("//").unwrap_or(cleaned_line.len());
    cleaned_line.drain(comment_offset..);
    cleaned_line.trim().to_string()
}

fn valid_line(line: &String) -> bool {
    let is_empty = line.is_empty();
    let is_label = line.starts_with("(") && line.ends_with(")");
    !(is_empty || is_label)
}

fn parse_instruction(line: &String, symbol_table: &SymbolTable) -> Instruction {
    if line.starts_with("@") {
        Instruction::AInstruction(parse_a_instruction(line, symbol_table))
    } else {
        Instruction::BInstruction(parse_b_instruction(line))
    }
}

fn parse_a_instruction(line: &String, symbol_table: &SymbolTable) -> AInstruction {
    let mut cloned = line.clone();
    let target = cloned.drain(1..).collect::<String>();
    let value = match target.parse::<u16>() {
        Ok(val) => val,
        Err(_) => match symbol_table.get(&target) {
            Some(val) => *val,
            None => panic!("Variable not found {}", target),
        },
    };
    AInstruction { value: value }
}

fn parse_b_instruction(line: &String) -> BInstruction {
    let (dest, line) = parse_b_instruction_dest(line);
    let (comp, line) = parse_b_instruction_comp(&line);
    let jump = parse_b_instruction_jump(&line);

    BInstruction{
        dest: dest,
        comp: comp,
        jump: jump,
    }
}

fn parse_b_instruction_dest(line: &String) -> (BInstructionDest, String) {
    match line.split_once("=") {
        None => (BInstructionDest::Null, line.to_string()),
        Some((a, b)) => {
            let dest = match a {
                "M" => BInstructionDest::M,
                "D" => BInstructionDest::D,
                "MD" => BInstructionDest::MD,
                "A" => BInstructionDest::A,
                "AM" => BInstructionDest::AM,
                "AD" => BInstructionDest::AD,
                "AMD" => BInstructionDest::AMD,
                _ => BInstructionDest::Null,
            };
            (dest, b.to_string())
        }
    }
}

fn parse_b_instruction_comp(line: &String) -> (BInstructionComp, String) {
    let (a, b): (&str, &str) = match line.split_once(";") {
        None => (line, ""),
        Some((a, b)) => (a, b),
    };

    let comp = match a {
        "0"     => BInstructionComp::Zero,
        "1"     => BInstructionComp::One,
        "-1"    => BInstructionComp::MinusOne,
        "D"     => BInstructionComp::D,
        "A"     => BInstructionComp::A,
        "!D"    => BInstructionComp::NotD,
        "!A"    => BInstructionComp::NotA,
        "-D"    => BInstructionComp::MinusD,
        "-A"    => BInstructionComp::MinusA,
        "D+1"   => BInstructionComp::DPlusOne,
        "A+1"   => BInstructionComp::APlusOne,
        "D-1"   => BInstructionComp::DMinusOne,
        "A-1"   => BInstructionComp::AMinusOne,
        "D+A"   => BInstructionComp::DPlusA,
        "D-A"   => BInstructionComp::DMinusA,
        "A-D"   => BInstructionComp::AMinusD,
        "D&A"   => BInstructionComp::DAndA,
        "D|A"   => BInstructionComp::DOrA,
        "M"     => BInstructionComp::M,
        "!M"    => BInstructionComp::NotM,
        "-M"    => BInstructionComp::MinusM,
        "M+1"   => BInstructionComp::MPlusOne,
        "M-1"   => BInstructionComp::MMinusOne,
        "D+M"   => BInstructionComp::DPlusM,
        "D-M"   => BInstructionComp::DMinusM,
        "M-D"   => BInstructionComp::MMinusD,
        "D&M"   => BInstructionComp::DAndM,
        "D|M"   => BInstructionComp::DOrM,
        _ => BInstructionComp::Zero,
    };
    (comp, b.to_string())
}

fn parse_b_instruction_jump(line: &String) -> BInstructionJump {
    match &**line {
        "JGT" => BInstructionJump::JGT,
        "JEQ" => BInstructionJump::JEQ,
        "JGE" => BInstructionJump::JGE,
        "JLT" => BInstructionJump::JLT,
        "JNE" => BInstructionJump::JNE,
        "JLE" => BInstructionJump::JLE,
        "JMP" => BInstructionJump::JMP,
        _ => BInstructionJump::Null,
    }
}