use std::env::args;
use std::fs::File;
use std::io::{Read};
use std::process::*;

enum Instruction {
  LEFT, RIGHT, INC, DEC, IN, OUT, LOOP, ELOOP
}

struct InstructionList {
    pub ins: Vec<Instruction>,
}

use Instruction::*;

impl std::fmt::Display for InstructionList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in &self.ins {
          write!(f, "{}", match i {
              LEFT => "if(++dp>29999)dp=0;",
              RIGHT => "if(--dp<0)dp=29999;",
              INC => "d[dp]++;",
              DEC => "d[dp]--",
              OUT => "putc((int)d[dp],stdout);fflush(stdout);",
              IN => r#"if((d[dp]=getc(stdin))<0){exit(1);}"#,
              LOOP => "while(d[dp]){",
              ELOOP => "}",
          })?;
      }
      Ok(())
    }
}

fn main() {
  let argv: Vec<String> = args().collect();
  
  if argv.len() < 1 {
    panic!("failed to collect args.");
  }
  
  if argv.len() < 2 {
    eprintln!("invalid usage.\nusage: {0} [options] <file>\nuse `{0} -h` for more info.", argv[0]);
  }
  
  let opts = argv.iter().filter(|s| s.starts_with("-"));
  
  if opts.clone().any(|s| s == "-h" || s == "--help") {
    println!(r#"BF COMPILER

USAGE
{0} [options] <file>
{0} -h

OPTIONS     ALIASES     DESCRIPTION
  --help    -h          prints this dialog and exits
  --output  -o          sets the desired output file"#, argv[0]);
    return;
  }
  
  let mut output_path = String::from("a.out");
  let modified_out = opts.clone().any(|s| s == "-o" || s == "--output");
  
  if modified_out {
    let output_path_position = argv
      .iter()
      .position(|s| s == "-o" || s == "--output")
      .expect("tried searching for an output name when there was no such output") + 1;
      assert!(output_path_position < argv.len(), "output flag is present, but no path specified");
      output_path = argv[output_path_position].clone();
  }
  
  let mut last = String::with_capacity(0);
  
  let mut input_name = String::with_capacity(0);
  let mut found = false;
  
  for arg in argv.iter().skip(1) {
    if arg.starts_with("-") || last == "-o" || last == "--output" {
      last = arg.clone();
      continue;
    }
    found = true;
    input_name = arg.to_string();
    break;
  }
  
  assert!(found, "no input file provided.");
  
  let mut f = File::open(input_name.clone()).unwrap_or_else(|e| {
    eprintln!("Could not open file `{input_name}`: {e}");
    exit(1);
  });
  
  let mut instructions = String::new();
  let _ = f.read_to_string(&mut instructions);
  
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
      },
      ']' => {
        assert!(nest > 0, "unmatched ] at {}:{}", row, col);
        nest -= 1;
        ins.push(ELOOP);
      },
      '\n' | '\r' => {
        row = 0;
        col += 1;
      },
      '>' => ins.push(RIGHT),
      '<' => ins.push(LEFT),
      '+' => ins.push(INC),
      '-' => ins.push(DEC),
      '.' => ins.push(OUT),
      ',' => ins.push(IN),
      _ => (),
    }
  }
  
  let ins = InstructionList { ins };
  
  // let mut cpath = output_path.clone();
  // cpath.push_str("_C.c");
  
  let compiler_cmd: &str = match Command::new("gcc")
    .stdin(Stdio::null())
    .stdout(Stdio::null())
    .stderr(Stdio::null())
    .spawn() {
    Ok(_) => "gcc",
    Err(e) => match e.kind() {
      std::io::ErrorKind::NotFound => {
        match Command::new("clang")
          .stdin(Stdio::null())
          .stdout(Stdio::null())
          .stderr(Stdio::null())
          .spawn() {
          Ok(_) => "clang",
          Err(_) => {
            eprintln!("could not find acceptable compiler.");
            exit(1);
          }
        }
      },
      _ => {
        eprintln!("error compiling program: {e}");
        exit(1);
      }
    }
  };
  let echo_child = Command::new("echo")
    .arg(format!(r#"extern void *stdin;extern void *stdout;void exit(int);int getc(void*);int putc(int,void*);void fflush(void*);char d[30000];int dp=0;int main(){{{ins}}}"#))
    .stdout(Stdio::piped())
    .spawn()
    .unwrap();
  
  let mut cmd = Command::new(compiler_cmd);
  cmd.args([if compiler_cmd == "gcc" {"-xc"} else {""}, "-", "-o", format!("{output_path}").as_str()])
    .stdin(Stdio::from(echo_child.stdout.unwrap()))
    .stdout(Stdio::null())
    .stderr(Stdio::null());
  
  let out = cmd.output()
    .unwrap_or_else(|e| {
      eprintln!("error compiling program: {e}");
      exit(1);
    });
  
  if !out.status.success() {
    println!("error compiling program: {}", out.status);
  }
}