use clap::{App, Arg};

fn main() {
    let _matches = App::new("this app name is echor")
    .version("1.0")
    .author("mixnuts. <mixnuts@gmail.com>")
    .about("Rust Echo")
    .arg(
        Arg::with_name("text")
            .value_name("TEXT")
            .help("Input Text")
            .required(true)
            .min_values(1)
    )
    .arg(
        Arg::with_name("omit_newline")
            .short("n")
            .help("Do not Print Newline")
            .takes_value(false)
    )
    .get_matches();
    println!("-----");
    println!("{:#?}", _matches);
    println!("-----");

    let text = _matches.values_of_lossy("text").unwrap();
    let omit_newline = _matches.is_present("omit_newline");
    let ending = if omit_newline { "" } else { "\n" };
    println!("{}{}", text.join(""), ending);
}
