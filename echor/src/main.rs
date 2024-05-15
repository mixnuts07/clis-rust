use clap::App;

fn main() {
    let _matches = App::new("this app name is echor")
    .version("1.0")
    .author("mixnuts. <mixnuts@gmail.com>")
    .about("Rust Echo")
    .get_matches();
}
