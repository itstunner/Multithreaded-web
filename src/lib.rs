use std::thread;
use std::sync::{Arc, Mutex, mpsc};

//struct Job;

enum Message{
    Newjob(Job),
    Terminate,
}

pub struct ThreadPool{
    worker: Vec<Worker>,
    sender: mpsc::Sender<Message>
}

impl Drop for ThreadPool{
    fn drop(&mut self){
        println!("Sending terminate message to all the workers");
        for _ in &self.worker{
            self.sender.send(Message::Terminate).unwrap();
        }        
            println!("shutting down all wokers");
        
        for worker in &mut self.worker{    
            println!("shutting down the woker: {}", worker.id);
            
            if let Some(thread) = worker.thread.take(){
                thread.join().unwrap();            
            }    
        }      
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool{
    pub fn new(size: usize) -> ThreadPool{
        assert!(size>0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut worker = Vec::with_capacity(size);
        
        for id in 0..size{
            worker.push(Worker::new(id, Arc::clone(&receiver)))
        }
        ThreadPool{worker,sender}
    }

    pub fn execute<F>(&self, f: F)
    where F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(Message::Newjob(job)).unwrap();
    }
}

pub struct Worker{
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker{
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>)-> Worker{
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            
            match message{
                Message::Newjob(job) => {
                    println!("worker {} got a job.executing", id);
                    job();
                }

                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);
                    break;
                }
            }
        });

        Worker{
            id,
            thread: Some(thread),
        }
    }
}