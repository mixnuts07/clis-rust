use std::{error::Error, fs::File, io::{self, BufRead, BufReader}};
use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    for file_name in config.files {
        match open(&file_name) {
            Err(err) => eprintln!("Failed to open {}: {}",file_name, err),
            Ok(file) => {
                let mut line_num = 0;
                for line_result in file.lines() {
                    let line = line_result?;
                    line_num += 1;

                    if config.number_lines {
                        println!("{:>6}\t{}", line_num, line)
                    } else {
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}

fn open(file_name: &str) -> MyResult<Box<dyn BufRead>> {
    match file_name {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(file_name)?)))
    }
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("mixnuts")
        .about("Rust Cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")   
                .help("Input Files(s)")
                .multiple(true)
                .default_value("-")
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .help("Number lines")
                .takes_value(false)
                .conflicts_with("number_nonblank_lines")
        )
        .arg(
            Arg::with_name("number_nonblank")
                .short("b")
                .long("number-nonblack")
                .help("Number non-blank lines")
                .takes_value(false)
        )
        .get_matches();

    Ok(Config{
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number_nonblack"),
    })
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool
}