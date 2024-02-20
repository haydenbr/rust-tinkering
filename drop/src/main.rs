use std::{thread::{self, JoinHandle}, time::Duration, sync::{mpsc::{channel, Sender}, Arc}};

struct GrandChild;

struct Child(GrandChild);

struct ThreadChild {
    join_handle: Option<JoinHandle<()>>,
    tx: Sender<()>,
}

impl ThreadChild {
    pub fn new() -> Self {
        let (tx, rx) = channel::<()>();

        let join_handle = thread::spawn(move || {
            let mut count = 0;
            loop {
                println!("{}", count);
                count += 1;
                thread::sleep(Duration::from_millis(1000));

                if rx.try_recv().is_ok() {
                    println!("exiting ThreadChild work loop");
                    break;
                }
            }
        });

        Self {
            join_handle: Some(join_handle),
            tx,
        }
    }
}

impl Drop for ThreadChild {
    fn drop(&mut self) {
        println!("dropping child");
        self.tx.send(()).ok();

        if let Some(join_handle) = self.join_handle.take() {
            join_handle.join().ok();
        }

        println!("done dropping child");
    }
}

struct Parent(ThreadChild);

fn main() {
    let parent = Parent(ThreadChild::new());

    thread::spawn(move || {
        thread::sleep(Duration::from_secs(2));
        println!("dropping parent");
        drop(parent);
    });

    println!("never ending main thread. Doing other work ...");
    thread::spawn(move || loop {}).join().ok();
}
