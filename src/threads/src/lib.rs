use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};
use uuid::Uuid;
pub struct ThreadPool {
    thread_ids: Vec<Uuid>,
    workers: Vec<Worker>,
    sender: Option<Sender<Process>>,
}

impl ThreadPool {
    pub fn new(number_of_threads: usize) -> Result<Self, &'static str> {
        if number_of_threads <= 0 {
            return Err("Thread Error: Request 1 or more threads.");
        }

        let mut workers: Vec<Worker> = Vec::with_capacity(number_of_threads);
        let mut thread_ids: Vec<Uuid> = Vec::with_capacity(number_of_threads);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        for _ in 0..number_of_threads {
            let id = Uuid::new_v4();
            workers.push(Worker::new(id, Arc::clone(&receiver)));
            thread_ids.push(id);
        }
        Ok(ThreadPool {
            thread_ids,
            workers,
            sender: Some(sender),
        })
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let process = Box::new(f);

        self.sender.as_ref().unwrap().send(process).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

type Process = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: Uuid,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id: Uuid, receiver: Arc<Mutex<Receiver<Process>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let process = receiver.lock().unwrap().recv();

            match process {
                Ok(value) => {
                    println!("Worker {id} got a job; executing.");

                    value()
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
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

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
