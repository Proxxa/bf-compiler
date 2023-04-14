#[allow(unused_imports)]
use std::process::*;

pub fn check_exists(cmd: &str) -> bool {
    Command::new(cmd)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .is_ok()
}

pub fn spawn_pipe_out<I, S>(cmd: &str, args: I) -> std::io::Result<Child>
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    Command::new(cmd).args(args).stdout(Stdio::piped()).spawn()
}

pub fn spawn_pipe_in<I, S>(cmd: &str, args: I, piper: Child) -> std::io::Result<Child>
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    Command::new(cmd)
        .args(args)
        .stdout(Stdio::null())
        .stdin(piper.stdout.unwrap())
        .spawn()
}
