use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc;

struct ThreadWorker {
    id       : usize,
    handle   : Option<thread::JoinHandle<()>>
}

pub struct ThreadPool{
    workers : Vec<ThreadWorker>,
    sender  : mpsc::Sender<Message>
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message{
    NewJob(Job),
    Terminate
}

impl ThreadWorker {
    fn new(id : usize, receiver : Arc<Mutex<mpsc::Receiver<Message>>>) -> ThreadWorker{
        let handle = thread::spawn(move || {
            loop {
                // Must use let. the match expression holds the lock!
                let message = receiver.lock().unwrap().recv().unwrap();
                
                match message {
                    Message::NewJob(job) =>  {
                        println!("Worker {} got a job; executing.", id);
                        job();
                    }
                    Message::Terminate => {
                        println!("Terminating worker {}", id);
                        break;
                    }
                }
            }
        });

        let handle = Some(handle);
        ThreadWorker{id, handle}
    }
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(nmax : usize) ->ThreadPool {
        assert!(nmax > 0);

        let mut workers = Vec::with_capacity(nmax);
        let (sender, receiver)  = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        for i in 0..nmax {
            workers.push(ThreadWorker::new(i, Arc::clone(&receiver)));
        }

        ThreadPool{workers, sender}
    }

    pub fn execute<F>(&mut self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let msg = Message::NewJob(Box::new(f));
        self.sender.send(msg).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self){
        println!("Shutting down...");
        for _worker in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(worker) = worker.handle.take() {
                worker.join().unwrap();
            }
        }
    }
}
