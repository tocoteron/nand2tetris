use crate::parser::{Program, Instruction, BInstructionDest, BInstructionComp, BInstructionJump};

pub fn generate_code(program: &Program) -> Vec<String> {
    program.iter().map(generate_instruction_code).collect()
}

fn generate_instruction_code(instruction: &Instruction) -> String {
    match instruction {
        Instruction::AInstruction(ins) => format!("{:016b}", ins.value),
        Instruction::BInstruction(ins) => {
            let comp_code = generate_instruction_comp_code(&ins.comp);
            let dest_code = generate_instruction_dest_code(&ins.dest);
            let jump_code = generate_instruction_jump_code(&ins.jump);
            format!("{:16b}", (7 << 13) | (comp_code << 6) | (dest_code << 3) | jump_code)
        }
    }
}

fn generate_instruction_comp_code(comp: &BInstructionComp) -> u16 {
    match comp {
        BInstructionComp::Zero =>       0b0101010,
        BInstructionComp::One =>        0b0111111,
        BInstructionComp::MinusOne =>   0b0111010,
        BInstructionComp::D =>          0b0001100,
        BInstructionComp::A =>          0b0110000,
        BInstructionComp::NotD =>       0b0001101,
        BInstructionComp::NotA =>       0b0110001,
        BInstructionComp::MinusD =>     0b0001111,
        BInstructionComp::MinusA =>     0b0110011,
        BInstructionComp::DPlusOne =>   0b0011111,
        BInstructionComp::APlusOne =>   0b0110111,
        BInstructionComp::DMinusOne =>  0b0001110,
        BInstructionComp::AMinusOne =>  0b0110010,
        BInstructionComp::DPlusA =>     0b0000010,
        BInstructionComp::DMinusA =>    0b0010011,
        BInstructionComp::AMinusD =>    0b0000111,
        BInstructionComp::DAndA =>      0b0000000,
        BInstructionComp::DOrA =>       0b0010101,
        BInstructionComp::M =>          0b1110000,
        BInstructionComp::NotM =>       0b1110001,
        BInstructionComp::MinusM =>     0b1110011,
        BInstructionComp::MPlusOne =>   0b1110111,
        BInstructionComp::MMinusOne =>  0b1110010,
        BInstructionComp::DPlusM =>     0b1000010,
        BInstructionComp::DMinusM =>    0b1010011,
        BInstructionComp::MMinusD =>    0b1000111,
        BInstructionComp::DAndM =>      0b1000000,
        BInstructionComp::DOrM =>       0b1010101,
    }
}

fn generate_instruction_dest_code(dest: &BInstructionDest) -> u16 {
    match dest {
        BInstructionDest::Null =>   0b000,
        BInstructionDest::M =>      0b001,
        BInstructionDest::D =>      0b010,
        BInstructionDest::MD =>     0b011,
        BInstructionDest::A =>      0b100,
        BInstructionDest::AM =>     0b101,
        BInstructionDest::AD =>     0b110,
        BInstructionDest::AMD =>    0b111,
    }
}


fn generate_instruction_jump_code(jump: &BInstructionJump) -> u16 {
    match jump {
        BInstructionJump::Null =>   0b000,
        BInstructionJump::JGT =>    0b001,
        BInstructionJump::JEQ =>    0b010,
        BInstructionJump::JGE =>    0b011,
        BInstructionJump::JLT =>    0b100,
        BInstructionJump::JNE =>    0b101,
        BInstructionJump::JLE =>    0b110,
        BInstructionJump::JMP =>    0b111,
    }
}
