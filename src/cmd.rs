use std::borrow::Borrow;
use std::io::{Error, ErrorKind, Result};
use std::process::{Child, Command, ExitStatus, Stdio};

// pub type FunResult = Result<String>;
pub type CmdResult = Result<()>;

#[macro_export]
macro_rules! macro_str {
    ($macro:ident) => {{
        let macro_name = stringify!($macro);
        let mut macro_str = String::new();
        let src = String::from(format!("{}/{}", env!("CARGO_MANIFEST_DIR"), file!()));
        let target_line = line!() as usize;
        let file: Vec<char> = std::fs::read_to_string(src)
            .expect("error reading file")
            .chars()
            .collect();
        let len = file.len();
        let mut i: usize = 0;
        let mut line = 1;
        let mut level = 0;
        while i < len {
            if file[i] == '\n' {
                line += 1;
            }
            if line == target_line {
                let cmp_str: String = file[i..i + macro_name.len()].iter().collect();
                if cmp_str == macro_name {
                    i += macro_name.len() + 1;
                    while file[i] != '{' && file[i] != '(' {
                        i += 1;
                    }
                    i += 1;
                    level += 1;

                    let with_quote = file[i] == '"';
                    let mut in_single_quote = false;
                    let mut in_double_quote = false;
                    if with_quote {
                        in_double_quote = true;
                        i += 1;
                    }
                    loop {
                        if !in_single_quote && !in_double_quote {
                            if file[i] == '}' || file[i] == ')' {
                                level -= 1;
                            } else if file[i] == '{' || file[i] == '(' {
                                level += 1;
                            }

                            if level == 0 {
                                break;
                            }
                        }

                        if file[i] == '"' && !in_single_quote {
                            in_double_quote = !in_double_quote;
                        } else if file[i] == '\'' && !in_double_quote {
                            in_single_quote = !in_single_quote;
                        }

                        macro_str.push(file[i]);
                        i += 1;
                    }
                    if with_quote {
                        macro_str.pop();
                    }
                    break;
                }
            }
            i += 1;
        }
        macro_str
    }};
}

#[macro_export]
macro_rules! run_cmd {
    (use $($arg:tt)*) => {{
        let mut sym_table = ::std::collections::HashMap::new();
        run_cmd!(&sym_table; $($arg)*)
    }};
    (&$st:expr; $var:ident, $($arg:tt)*) => {{
        $st.insert(stringify!($var).into(), format!("{}", $var));
        run_cmd!(&$st; $($arg)*)
    }};
    (&$st:expr; $var:ident; $($arg:tt)*) => {{
        $st.insert(stringify!($var).into(), format!("{}", $var));
        let src = $crate::macro_str!(run_cmd);
        $crate::run_cmd(&$crate::resolve_name(&src, &$st, &file!(), line!()))
    }};
    ($cmd:ident $($arg:tt)*) => {{
        $crate::run_cmd(&$crate::macro_str!(run_cmd))
    }};
    ($($arg:tt)*) => {{
        $crate::run_cmd(&format!($($arg)*))
    }};
}

pub trait ProcessResult {
    fn get_result(process: &mut Process) -> Self;
}

pub struct Process {
    cur_dir: Option<String>,
    full_cmd: Vec<Vec<String>>,
}

impl Process {
    pub fn new<S: Borrow<str>>(pipe_cmd: S) -> Self {
        let args = parse_args(pipe_cmd.borrow());
        let argv = parse_argv(args);

        Self {
            cur_dir: None,
            full_cmd: vec![argv],
        }
    }

    pub fn current_dir<S: Borrow<str>>(&mut self, dir: S) -> &mut Self {
        self.cur_dir = Some(dir.borrow().to_string());
        self
    }

    pub fn pipe<S: Borrow<str>>(&mut self, pipe_cmd: S) -> &mut Self {
        let args = parse_args(pipe_cmd.borrow());
        let argv = parse_argv(args);

        self.full_cmd.push(argv);
        self
    }

    pub fn wait<T: ProcessResult>(&mut self) -> T {
        T::get_result(self)
    }
}

impl ProcessResult for CmdResult {
    fn get_result(process: &mut Process) -> Self {
        let (mut last_proc, full_cmd_str) = run_full_cmd(process, false)?;
        let status = last_proc.wait()?;
        if !status.success() {
            Err(to_io_error(&full_cmd_str, status))
        } else {
            Ok(())
        }
    }
}

fn format_full_cmd(full_cmd: &Vec<Vec<String>>) -> String {
    let mut full_cmd_str = String::from(full_cmd[0].join(" "));
    for cmd in full_cmd.iter().skip(1) {
        full_cmd_str += " | ";
        full_cmd_str += &cmd.join(" ");
    }
    full_cmd_str
}

fn run_full_cmd(process: &mut Process, pipe_last: bool) -> Result<(Child, String)> {
    let mut full_cmd_str = format_full_cmd(&process.full_cmd);
    let first_cmd = &process.full_cmd[0];
    let mut cmd = Command::new(&first_cmd[0]);
    if let Some(dir) = &process.cur_dir {
        full_cmd_str += &format!(" (cd: {})", dir);
        cmd.current_dir(dir);
    }

    let mut last_proc = cmd
        .args(&first_cmd[1..])
        .stdout(if pipe_last || process.full_cmd.len() > 1 {
            Stdio::piped()
        } else {
            Stdio::inherit()
        })
        .spawn()?;
    for (i, cmd) in process.full_cmd.iter().skip(1).enumerate() {
        let new_proc = Command::new(&cmd[0])
            .args(&cmd[1..])
            .stdin(last_proc.stdout.take().unwrap())
            .stdout(if !pipe_last && i == process.full_cmd.len() - 2 {
                Stdio::inherit()
            } else {
                Stdio::piped()
            })
            .spawn()?;
        last_proc.wait();
        last_proc = new_proc;
    }

    Ok((last_proc, full_cmd_str))
}

fn run_pipe_cmd(full_command: &str, cd_opt: &mut Option<String>) -> CmdResult {
    let pipe_args = parse_pipes(full_command.trim());
    let pipe_argv = parse_argv(pipe_args);

    let mut pipe_iter = pipe_argv[0].split_whitespace();
    let cmd = pipe_iter.next().unwrap();
    if cmd == "cd" || cmd == "lcd" {
        let dir = pipe_iter.next().unwrap().trim_matches('"');
        if pipe_iter.next() != None {
            let err = Error::new(
                ErrorKind::Other,
                format!("{} format wrong: {}", cmd, full_command),
            );
            return Err(err);
        } else {
            if cmd == "cd" {
                // info!("Set env current_dir: \"{}\"", dir);
                return std::env::set_current_dir(dir);
            } else {
                // info!("Set local current_dir: \"{}\"", dir);
                *cd_opt = Some(dir.into());
                return Ok(());
            }
        }
    } else if cmd == "pwd" {
        let pwd = std::env::current_dir()?;
        // info!("Running \"pwd\" ...");
        println!("{}", pwd.display());
        return Ok(());
    }

    let mut last_proc = Process::new(pipe_argv[0].clone());
    if let Some(dir) = cd_opt {
        last_proc.current_dir(dir.clone());
    }
    for pipe_cmd in pipe_argv.iter().skip(1) {
        last_proc.pipe(pipe_cmd.clone());
    }

    last_proc.wait::<CmdResult>()
}

pub fn run_cmd(cmds: &str) -> CmdResult {
    let cmd_args = parse_cmds(cmds);
    let cmd_argv = parse_argv(cmd_args);
    let mut cd_opt: Option<String> = None;
    for cmd in cmd_argv {
        if let Err(e) = run_pipe_cmd(&cmd, &mut cd_opt) {
            return Err(e);
        }
    }
    Ok(())
}

fn to_io_error(command: &str, status: ExitStatus) -> Error {
    if let Some(code) = status.code() {
        Error::new(ErrorKind::Other, format!("{} exit with {}", command, code))
    } else {
        Error::new(ErrorKind::Other, "Unknown error")
    }
}

fn parse_args(s: &str) -> String {
    let mut in_single_quote = false;
    let mut in_double_quote = false;
    s.chars()
        .map(|c| {
            if c == '"' && !in_single_quote {
                in_double_quote = !in_double_quote;
                '\n'
            } else if c == '\'' && !in_double_quote {
                in_single_quote = !in_single_quote;
                '\n'
            } else if !in_single_quote && !in_double_quote && char::is_whitespace(c) {
                '\n'
            } else {
                c
            }
        })
        .collect()
}

fn parse_cmds(s: &str) -> String {
    parse_seps(s, ';')
}

fn parse_pipes(s: &str) -> String {
    parse_seps(s, '|')
}

fn parse_seps(s: &str, sep: char) -> String {
    let mut in_single_quote = false;
    let mut in_double_quote = false;
    s.chars()
        .map(|c| {
            if c == '"' && !in_single_quote {
                in_double_quote = !in_double_quote;
            } else if c == '\'' && !in_double_quote {
                in_single_quote = !in_single_quote;
            }

            if c == sep && !in_single_quote && !in_double_quote {
                '\n'
            } else {
                c
            }
        })
        .collect()
}

fn parse_argv(s: String) -> Vec<String> {
    s.split("\n")
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}
