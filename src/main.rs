use structopt::StructOpt;
use rust_organizer::{get_file_names, format_extensions};

#[derive(StructOpt, Debug)]
struct Cli {
    #[structopt(short = "o", long = "origin", default_value = "./", parse(from_os_str))]
    root: std::path::PathBuf,
    #[structopt(short = "d", long = "destination", default_value = "Rust_Organizer", parse(from_os_str))] //TODO: change this to include the current date
    destination: std::path::PathBuf,
    #[structopt(short = "e", long = "extension")]
    extension: Vec<String>,
    #[structopt(short = "m", long = "move")]
    move_files: bool,
}

fn main() {
    let mut args = Cli::from_args();
    println!("Args: {:?}", args);
    let mut files = Vec::new();
    let paths = get_file_names(&args.root, &mut args.extension, &mut files);
    // for file in paths {
    //     println!("{}", file.display());
    // }
    println!("num of elements: {}", paths.len());
}
