use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Message>,
}

#[allow(dead_code)]
impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        let mut worker_pool = Vec::with_capacity(size);

        let (sender, receiver) = channel::<Message>();

        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            worker_pool.push(Worker::new(id, receiver.clone()));
        }

        Self {
            workers: worker_pool,
            sender,
        }
    }

    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        for worker in &mut self.workers {
            println!("Sending shutdown to worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

#[allow(dead_code)]
enum Message {
    NewJob(Job),
    Terminate,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            let receiver_mutex = receiver.lock().unwrap();
            let msg = receiver_mutex.recv().unwrap();
            match msg {
                Message::NewJob(job) => {
                    println!("Worker {} received a job!", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} received Termination Request!", id);
                    return;
                }
            }
        });
        Self {
            id,
            thread: Some(thread),
        }
    }
}
