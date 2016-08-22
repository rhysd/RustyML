use std::env;
use std::path::{PathBuf, Path};
use std::io::Write;

#[derive(Debug)]
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
                return Err("File does not exist: ".to_string() + arg);
            }
            cli.files.push(path);
            continue;
        }

        let path = env::current_dir()
                .expect("Failed to get current working directory!")
                .join(path);
        if !path.exists() {
            return Err("File does not exist: ".to_string() + arg);
        }
        cli.files.push(path);
    }

    return Ok(cli);
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
            return;
        },
    };

    if cli.help {
        help();
        return;
    }

    println!("{:?}", cli);
}
