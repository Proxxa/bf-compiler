mod arguments;
mod commands;
mod instructions;
mod precompiler;
mod reader;

pub use instructions::*;
use std::process::*;

fn main() -> std::io::Result<()> {
    let argv = arguments::get_args();

    match argv.len() {
        ..=0 => panic!("failed to collect args."),
        1 => {
            eprintln!(
                "invalid usage.\nusage: {0} [options] <file>\nuse `{0} -h` for more info.",
                argv[0]
            );
            exit(1)
        }
        _ => (),
    }

    if argv.has_any(&["-h", "--help"]) {
        println!(include_str!("text_include/help"), argv[0]);
        return Ok(());
    }

    let mut output_path = String::from("a.out");
    let modified_out = argv.has_any(&["-o", "--output"]);

    if modified_out {
        let output_path_position = argv
            .iter()
            .position(|s| s == "-o" || s == "--output")
            .expect("tried searching for an output name when there was no such output")
            + 1;
        assert!(
            output_path_position < argv.len(),
            "output flag is present, but no path specified"
        );
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

    let instructions = reader::read_file(input_name.as_str())?;

    let instructions = precompiler::precompile(instructions);

    let compiler_cmd: &str = if commands::check_exists("gcc") {
        "gcc"
    } else {
        if commands::check_exists("clang") {
            "clang"
        } else {
            eprintln!("could not find acceptable compiler.");
            exit(1);
        }
    };

    let compiler_cmd = "clang";

    let echo_child = commands::spawn_pipe_out(
        "echo",
        [include_str!("text_include/c_program_base")
            .replace("{instructions}", instructions.to_string().as_str())],
    )
    .unwrap();

    let cmd = commands::spawn_pipe_in(
        compiler_cmd,
        [
            "-xc",
            "-",
            "-o",
            format!("{output_path}").as_str(),
        ],
        echo_child,
    )
    .unwrap_or_else(|e| {
        eprintln!("error compiling program: {e}");
        exit(1);
    });

    let out = cmd.wait_with_output().unwrap_or_else(|e| {
        eprintln!("error compiling program: {e}");
        exit(1);
    });

    if !out.status.success() {
        println!("error compiling program: {}", out.status);
    }

    Ok(())
}
