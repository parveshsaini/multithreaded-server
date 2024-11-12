use std::{sync::{mpsc, Arc, Mutex}, thread};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool{
        assert!(size >0);

        let mut workers = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();

        // Arc: Allows multiple threads to share ownership of the Receiver.
        // Mutex: Ensures that only one thread can access the Receiver at a time, 
        // making access synchronized and preventing race conditions.
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender: Some(sender) }
    }

    pub fn execute<F>(&self, f: F)
    where 
        F: FnOnce() + Send + 'static
    {
        let job= Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {

        drop(self.sender.take());

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
    thread: Option<thread::JoinHandle<()>>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) ->Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Thread {id} is running a new job 🚀");

                    job();
                }
                Err(_) => {
                    println!("Thread {id} disconnected; shutting down...");
                    break;
                }
            }
        });

        Worker {id, thread: Some(thread)}
    }
}