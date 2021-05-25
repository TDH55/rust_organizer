use std::{sync::{Mutex, mpsc, Arc}, thread, usize};
use std::fs;
use std::ffi::OsStr;
use std::path::PathBuf;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool
    ///
    /// # Panics
    ///
    /// The 'new function will panic if the size is zero.
    pub  fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, reciever) = mpsc::channel();

        let reciever = Arc::new(Mutex::new(reciever));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&reciever)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, reciever: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = reciever.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);

                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);

                    break;
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}

enum Message {
    NewJob(Job),
    Terminate,
}

//TODO: function to check format of extensions -> remove period from beginning
pub fn format_extensions(exts: &mut Vec<String>) {
    for i in 0..exts.len() {
        if exts[i].chars().nth(0).unwrap() == '.' {
            println!("{}", exts[i]);
            let mut chars = exts[i].chars();
            chars.next();
            exts[i] = chars.as_str().to_owned();
        }
    }
}

//DONE: function to get file paths
pub fn get_file_names<'a>(origin: &'a std::path::PathBuf, extensions: &'a mut Vec<String>, paths: &'a mut Vec<PathBuf>) -> &'a mut Vec<std::path::PathBuf>{
    assert!(origin.is_dir()); //TODO: clean up error handling
    format_extensions(extensions);
    for item in fs::read_dir(origin).unwrap() {
        let file = item.unwrap();
        let path = file.path();
        if path.is_dir() {
            get_file_names(&path, extensions, paths);

        } else {
            let ext = path.extension();
            match ext {
                Some(ext) => {
                    if extensions.contains(&ext.to_str().unwrap().to_string()) {
                        paths.push(path);
                    }
                }
                None => {}
            }
        }
    }
    paths
}

fn get_file_extension(path: &std::path::PathBuf) -> &OsStr {
    path.extension().unwrap() //TODO: error handling
}