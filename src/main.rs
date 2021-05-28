use structopt::StructOpt;
use rust_organizer::{get_file_names, organize_file, verify_directories};
use threadpool::ThreadPool;
use std::sync::mpsc;

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
    let move_file = args.move_files;
    let destination = args.destination;
    let origin = args.root;
    verify_directories(&destination);
    let mut files = Vec::new();
    let paths = get_file_names(&origin, &mut args.extension, &mut files).to_owned();

    let (tx, rc) = mpsc::channel();
    let n_workers = 30;
    let n_jobs = paths.len();

    let pool = ThreadPool::new(n_workers);
    for file in paths {
        let tx = tx.clone();
        let destination = destination.clone();
        let move_file = move_file.clone();
        pool.execute(move || {
            organize_file(&file, &destination, move_file).unwrap();
            tx.send(()).unwrap();
        })
    }

    for _ in 0..n_jobs {
        rc.recv().unwrap();
    }
}
