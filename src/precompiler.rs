use crate::*;
use Instruction::*;

pub fn precompile(instructions: String) -> InstructionList {
    let mut nest = 0;
    let mut row = 0usize;
    let mut col = 1usize;
    let mut ins: Vec<Instruction> = Vec::new();
    for c in instructions.bytes() {
        row += 1;
        match c as char {
            '[' => {
                nest += 1;
                ins.push(LOOP);
            }
            ']' => {
                assert!(nest > 0, "unmatched ] at {}:{}", row, col);
                nest -= 1;
                ins.push(ELOOP);
            }
            '\n' | '\r' => {
                row = 0;
                col += 1;
            }
            '>' => ins.push(RIGHT),
            '<' => ins.push(LEFT),
            '+' => ins.push(INC),
            '-' => ins.push(DEC),
            '.' => ins.push(OUT),
            ',' => ins.push(IN),
            _ => (),
        }
    }

    InstructionList { ins }
}
