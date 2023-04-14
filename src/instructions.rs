pub enum Instruction {
    LEFT,
    RIGHT,
    INC,
    DEC,
    IN,
    OUT,
    LOOP,
    ELOOP,
}

pub struct InstructionList {
    pub ins: Vec<Instruction>,
}

use Instruction::*;

impl std::fmt::Display for InstructionList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in &self.ins {
            write!(
                f,
                "{}",
                match i {
                    LEFT => "if(++dp>29999)dp=0;",
                    RIGHT => "if(--dp<0)dp=29999;",
                    INC => "d[dp]++;",
                    DEC => "d[dp]--",
                    OUT => "putc((int)d[dp],stdout);fflush(stdout);",
                    IN => r#"if((d[dp]=getc(stdin))<0){exit(1);}"#,
                    LOOP => "while(d[dp]){",
                    ELOOP => "}",
                }
            )?;
        }
        Ok(())
    }
}
