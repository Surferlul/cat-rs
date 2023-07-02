use std::{
    fs::File,
    io::{self, Read, Write},
    path::{Path, PathBuf},
    process::exit,
};

use clap::Parser;

#[derive(Parser)]
#[command(
    author,
    version,
    about = "Concatenate file(s) to standard output",
    long_about = "Concatenate file(s) to standard output

Examples:
  cat-rs f - g  Output f's contents, then standard input, then g's contents.
  cat-rs        Copy standard input to standard output."
)]
struct Cli {
    #[arg(help = "equivalent to -vET")]
    #[arg(short = 'A', long = "show-all")]
    show_all: bool,
    #[arg(help = "number nonempty output lines, overrides -n")]
    #[arg(short = 'b', long = "number-nonblank")]
    number_nonblank: bool,
    #[arg(help = "equivalent to -vE")]
    #[arg(short = 'e')]
    nonprinting_ends: bool,
    #[arg(help = "display $ at end of each line")]
    #[arg(short = 'E', long = "show-ends")]
    show_ends: bool,
    #[arg(help = "number all output lines")]
    #[arg(short = 'n', long = "number")]
    number: bool,
    #[arg(help = "suppress repeated empty output lines")]
    #[arg(short = 's', long = "squeeze-blanks")]
    squeeze_blanks: bool,
    #[arg(help = "equivalent to -vT")]
    #[arg(short = 't')]
    nonprinting_tabs: bool,
    #[arg(help = "display TAB character as ^I")]
    #[arg(short = 'T', long = "show-tabs")]
    show_tabs: bool,
    #[arg(help = "(ignored)")]
    #[arg(short = 'u')]
    ignored_value: bool,
    #[arg(help = "use ^ and M- notation, except for LFD and TAB")]
    #[arg(short = 'v', long = "show-nonprinting")]
    show_nonprinting: bool,
    #[arg(help = "With no file, or when file is -, read standard input")]
    files: Vec<String>,
}

#[derive(Clone)]
enum Input {
    File(PathBuf),
    Stdio,
}

impl Input {
    fn new(input: String) -> Self {
        if &input == "-" {
            Self::Stdio
        } else {
            Self::File(Path::new(&input).to_path_buf())
        }
    }
}

struct Settings {
    number_nonblank: bool,
    number: bool,
    ends: bool,
    tabs: bool,
    nonprinting: bool,
    inputs: Vec<Input>,
}

impl Settings {
    fn new(cli: Cli) -> Self {
        Self {
            number_nonblank: cli.number_nonblank,
            number: cli.number || cli.number_nonblank,
            ends: cli.show_ends || cli.nonprinting_ends || cli.show_all,
            tabs: cli.show_tabs || cli.nonprinting_tabs || cli.show_all,
            nonprinting: cli.show_nonprinting
                || cli.nonprinting_ends
                || cli.nonprinting_tabs
                || cli.show_all,
            inputs: cli.files.into_iter().map(Input::new).collect(),
        }
    }
}

fn read_stdin() -> Vec<u8> {
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();
    input
}

fn read_file(path: PathBuf) -> Vec<u8> {
    let mut content = Vec::new();
    let mut file = File::open(path).unwrap();
    file.read_to_end(&mut content).unwrap();
    content
}

fn parse_number_nonblank(contents: Vec<u8>) -> Vec<u8> {
    contents
}

fn parse_number(contents: Vec<u8>) -> Vec<u8> {
    contents
}

fn parse_ends(contents: Vec<u8>) -> Vec<u8> {
    contents
}

fn parse_tabs(contents: Vec<u8>) -> Vec<u8> {
    contents
}

fn parse_nonprinting(contents: Vec<u8>) -> Vec<u8> {
    contents
}

fn parse_to_stdout(mut contents: Vec<u8>, settings: &Settings) {
    let stdout = io::stdout();
    if settings.number_nonblank {
        contents = parse_number_nonblank(contents)
    } else if settings.number {
        contents = parse_number(contents)
    }
    if settings.ends {
        contents = parse_ends(contents)
    }
    if settings.tabs {
        contents = parse_tabs(contents)
    }
    if settings.nonprinting {
        contents = parse_nonprinting(contents)
    }
    if let Err(e) = stdout.lock().write_all(&contents) {
        eprintln!("Error writing to stdout: {e}");
        exit(1)
    };
}

fn main() {
    let settings = Settings::new(Cli::parse());
    for input in settings.inputs.clone() {
        parse_to_stdout(
            match input {
                Input::File(path) => read_file(path),
                Input::Stdio => read_stdin(),
            },
            &settings,
        )
    }
}
