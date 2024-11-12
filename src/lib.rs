use std::{sync::{mpsc, Arc, Mutex}, thread};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
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

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where 
        F: FnOnce() + Send + 'static
    {
        let job= Box::new(f);

        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) ->Worker {
        let thread= thread::spawn(move || loop {
            let job= receiver.lock().unwrap().recv().unwrap();
            println!("Thread {id} is running a new job 🚀🚀🚀");
            job();
        });

        Worker {id, thread}
    }
}