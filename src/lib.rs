use std::thread;
use std::thread::JoinHandle;
use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::Receiver;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    threads: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}


impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));
        let mut threads = Vec::with_capacity(size);
        for id in 0..size {
            let w = Worker::new(id, Arc::clone(&receiver));
            threads.push(w)
        }
        ThreadPool {
            threads,
            sender,
        }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}


pub struct Worker {
    id: usize,
    handle: Option<JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
        let t = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker: {} gott a new job and it's executing", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} has been told to terminate", id)
                }
            }
        });
        Worker {
            id,
            handle: Some(t),
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers!");
        for _ in &self.threads {
            self.sender.send(Message::Terminate);
        }
        println!("Shutting down  all workers.");
        for worker in &mut self.threads {
            println!("Worker {} is shutting down.", worker.id);
            if let Some(thread) = worker.handle.take() {
                thread.join().unwrap();
            }
        }
    }
}

pub enum Message {
    NewJob(Job),
    Terminate,
}