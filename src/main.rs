use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Cli {
    #[structopt(short = "o", long = "origin", default_value = "./", parse(from_os_str))]
    root: std::path::PathBuf,
    #[structopt(short = "d", long = "destination", default_value = "Rust_Organizer", parse(from_os_str))] //TODO: change this to include the current date
    destination: std::path::PathBuf,
    #[structopt(short = "e", long = "extension")]
    extension: String,
    #[structopt(short = "m", long = "move")]
    move_files: bool,
}

fn main() {
    let args = Cli::from_args();
    println!("Args: {:?}", args);
}
