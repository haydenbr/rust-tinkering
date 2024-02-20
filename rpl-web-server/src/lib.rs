use std::{thread::{self, JoinHandle}, sync::{mpsc::{Sender, Receiver, self}, Arc, Mutex}};

type Job = Box<dyn FnOnce() + Send + 'static>;
type JobReceiver = Arc<Mutex<Receiver<Job>>>;

struct Worker {
	id: usize,
	join_handle: Option<JoinHandle<()>>,
}

impl Worker {
	fn new(id: usize, job_receiver: JobReceiver) -> Self {
		let join_handle = thread::spawn(move || loop {
			// with `let`,` any temporary value used in the rhs expression is dropped, so the mutex guard is dropped as soon
			// as we're done reading from the mutex
			// `while let`, `if let`, and `match` do not drop values until the whole block is done
			let message = job_receiver.lock().unwrap().recv();

			match message {
				Ok(job) => {
					println!("worker {} executing a job", id);
					job();
				}
				Err(_) => {
					println!("worker {} disconnecting", id);
					break;
				}
			}
		});

		Self {
			id,
			join_handle: Some(join_handle),
		}
	}
}

pub struct ThreadPool {
	workers: Vec<Worker>,
	job_sender: Option<Sender<Job>>,
}
pub struct PoolCreationError;

impl ThreadPool {
	pub fn new(size: usize) -> Self {
		assert!(size > 0);

		// with capacity allocates memory up-front while new and adding elements resizes the vec as it goes, which is
		// slightly less efficient. We can use with_capacity since we know the number of elements we need to store
		let mut workers: Vec<Worker> = Vec::with_capacity(size);
		let (job_sender, job_receiver) = mpsc::channel();
		let job_receiver = Arc::new(Mutex::new(job_receiver));

		for i in 0 .. size {
			workers.push(Worker::new(i, job_receiver.clone()));
		}

		Self {
			workers,
			job_sender: Some(job_sender),
		}
	}

	pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
		todo!()
	}

	pub fn execute<F>(&self, f: F)
		where
			F: FnOnce() + Send + 'static
	{
		let job = Box::new(f);

		self.job_sender.as_ref().unwrap().send(job).unwrap();
	}
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
			// we don't need to send any particular close message to the channels. This closes the sender, which closes the
			// channel, which causes the worker threads to exit their worker loop and then the thread exists
			drop(self.job_sender.take());

			for worker in &mut self.workers {
				if let Some(join_handle) = worker.join_handle.take() {
					println!("shutting down worker {}", worker.id);
					join_handle.join().unwrap();
				}
			}
    }
}