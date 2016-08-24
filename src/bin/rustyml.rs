extern crate rustyml;

use std::env;
use std::path::{PathBuf, Path};
use std::io::Write;
use std::process::exit;
use rustyml::compiler::{Compiler, CompileError};

#[derive(Debug,PartialEq)]
enum Mode {
    Nop,
    Build,
    Run,
}

#[derive(Debug)]
struct Cli {
    mode: Mode,
    files: Vec<PathBuf>,
    help: bool,
    // Note: Flags will be here (e.g. optimization level)
}

macro_rules! errorln(
    ($($arg:tt)*) => { {
        writeln!(&mut ::std::io::stderr(), $($arg)*).expect("Failed to write to STDERR!")
    } }
);

fn parse_argv(argv: Vec<String>) -> Result<Cli, String> {
    let mode = match argv.get(1).map(|s| s.as_str()) {
        Some("run")   => Mode::Run,
        Some("build") => Mode::Build,
        _             => return Ok(Cli { mode: Mode::Nop, files: vec![], help: true }),
    };

    let mut cli = Cli { mode: mode, files: vec![], help: false };
    // let cwd = env::current_dir().expect("Failed to get current working directory!");

    for arg in argv.iter().skip(2) {
        match arg.as_str() {
            "--help" => {
                cli.help = true;
                continue;
            },
            _ => {}
            // Note: More options should come here.
        };

        let mut path = PathBuf::new();
        path.push(Path::new(arg));

        if path.is_absolute() {
            if !path.exists() {
                return Err(format!("File does not exist: {}", arg));
            }
            cli.files.push(path);
            continue;
        }

        let path = env::current_dir()
                .expect("Failed to get current working directory!")
                .join(path);
        if !path.exists() {
            return Err(format!("File does not exist: {}", arg));
        }
        cli.files.push(path);
    }

    if !cli.help && cli.files.len() == 0 {
        return Err("No file target is specified.".to_string());
    }

    return Ok(cli);
}

#[cfg(test)]
macro_rules! test_parse {
    ($($str:expr),*) => {
        parse_argv(vec!["rustyml".to_string(), $($str.to_string()),*])
    }
}

#[test]
fn test_parse_help() {
    let cli = test_parse!("help").unwrap();
    assert!(cli.help);

    let cli = test_parse!("unknown").unwrap();
    assert!(cli.help);

    let cli = test_parse!("run", "--help").unwrap();
    assert!(cli.help);
}

#[test]
fn test_parse_mode() {
    let cli = test_parse!("run", file!()).unwrap();
    assert_eq!(cli.mode, Mode::Run);

    let cli = test_parse!("build", file!()).unwrap();
    assert_eq!(cli.mode, Mode::Build);

    let cli = test_parse!("help").unwrap();
    assert_eq!(cli.mode, Mode::Nop);

    let cli = test_parse!("unknown").unwrap();
    assert_eq!(cli.mode, Mode::Nop);

    let cli = test_parse!("unknown", file!()).unwrap();
    assert_eq!(cli.mode, Mode::Nop);
}

#[test]
fn test_parse_path() {
    let cli = test_parse!("run", file!(), file!()).unwrap();
    assert_eq!(cli.files.len(), 2);
    assert!(cli.files.first().unwrap().to_str().unwrap().ends_with(file!()));
}

#[test]
fn test_not_found_error() {
    let ret = test_parse!("run", "unknown-file-path");
    assert!(ret.is_err());
    let ret = test_parse!("run", "/absolute/path/to/unknown-file-path");
    assert!(ret.is_err());
}

fn help() {
    println!(r#"Usage: rustyml SUBCOMMANDS [OPTIONS] FILES

Subcommand:
    run:   Compile and run.
    build: Only compile.
    help:  Show this help.

Options:
    --help: Show this help."#);
}

fn main() {
    let cli = match parse_argv(env::args().collect()) {
        Ok(c) => c,
        Err(msg) => {
            errorln!("Error on parsing command line arguments: {}", msg);
            exit(1);
        },
    };

    if cli.help {
        help();
        return;
    }

    let compiler = Compiler { files: cli.files };
    let code = match compiler.compile() {
        Ok(compiled) => {
            println!("Success: {:?}", compiled);
            0
        },
        Err(CompileError::ParseError(e)) => {
            println!("Syntax error:{}:{}: Expected one of {:?}", e.line, e.column, e.expected);
            2
        },
        Err(e) => {
            println!("Error: {:?}", e);
            255
        },
    };
    exit(code);
}
