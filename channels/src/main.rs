use std::{time::Duration, thread, fmt::Debug, sync::{self, Once, Arc}};
use crossbeam::{channel::{tick, bounded, unbounded, Sender, Receiver, after, Select}, select};

#[derive(Debug)]
pub enum DownloadOutput {
    Buf(Vec<u8>),
    Ended,
    Error(String),
}

impl From<Vec<u8>> for DownloadOutput {
    fn from(value: Vec<u8>) -> Self {
        Self::Buf(value)
    }
}

impl Into<Vec<u8>> for DownloadOutput {
    fn into(self) -> Vec<u8> {
        match self {
            DownloadOutput::Buf(buf) => buf,
            _ => vec![]
        }
    }
}

fn main() {
    channel_select_execution();
}

fn channel_select_execution() {
    let mut selector = Select::new();
    // select! {
    //     recv(after(Duration::from_secs(1))) -> _ => println!("got message after 1 sec"),
    //     recv(after(Duration::from_secs(10))) -> _ => println!("got message after 10 sec"),
    // }

    let after_1 = after(Duration::from_secs(1));
    let index_1 = selector.recv(&after_1);
    let after_10 = after(Duration::from_secs(10));
    let index_10 = selector.recv(&after_10);

    let result_index = selector.ready();

    if result_index == index_1 {
        println!("after 1 sec");
    } else if result_index == index_10 {
        println!("after 10 sec");
    } else {
        println!("nothin");
    }
}

#[derive(Clone)]
struct Context {
    close_internal_tx: Sender<()>,
    is_closed_rx: Receiver<()>,
}

impl Context {
    pub fn new() -> Self {
        let (is_closed_tx, is_closed_rx) = bounded::<()>(0);
        let (close_internal_tx, close_internal_rx) = bounded::<()>(0);

        thread::spawn(move || {
            close_internal_rx.recv().ok();
            drop(is_closed_tx);
        });

        Self {
            is_closed_rx,
            close_internal_tx,
        }
    }

    pub fn close(&self) {
        self.close_internal_tx.try_send(()).ok();
    }

    pub fn is_closed(&self) -> Receiver<()> {
        self.is_closed_rx.clone()
    }
}

fn with_context() {
    let (numbers_tx, numbers_rx) = bounded::<i32>(0);
    let (letters_tx, letters_rx) = bounded::<&str>(0);
    let context = Context::new();

    let number_context = context.clone();
    let number_consumer = thread::spawn(move || loop {
        select! {
            recv(numbers_rx) -> msg => {
                if let Ok(msg) = msg {
                    println!("number: {}", msg)
                }
            },
            recv(number_context.is_closed()) -> _ => {
                println!("number_consumer: context is closed");
                break;
            }
        }
    });

    thread::spawn(move || {
        for i in 0..100 {
            thread::sleep(Duration::from_millis(500));
            numbers_tx.send(i).ok();
        }
    });

    let letter_consumer = thread::spawn(move || {
        for msg in letters_rx {
            println!("letter: {}", msg);
        }

        println!("exiting letter consumer");
    });

    let letter_context = context.clone();
    let letter_producer = thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(1000));

        select! {
            send(letters_tx, "a") -> msg => {
                if msg.is_ok() {
                    println!("sent letter");
                }
                // else {
                //     println!("letters channel is closed");
                //     break;
                // }
            },
            recv(letter_context.is_closed()) -> _ => {
                println!("letter_producer: context is closed");
                break;
            }
        }
    });

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(5 * 1000));
        println!("closing context");
        context.close();
    });

    number_consumer.join().ok();
    letter_consumer.join().ok();
    letter_producer.join().ok();

    println!("sleep before done");
    thread::sleep(Duration::from_secs(2));
}

fn exit_channel() {
    let (tx_numbers, rx_numbers) = bounded::<i32>(0);
    let (tx_exit, rx_exit) = bounded::<()>(0);

    let t1 = thread::spawn(move || loop {
        select! {
            recv(rx_numbers) -> msg => println!("msg: {:?}", msg),
            recv(rx_exit) -> _ => {
                println!("someone wants to exit");
                break;
            },
        }
    });

    let t2 = thread::spawn(move || {
        for i in 0..5 {
            tx_numbers.send(i).ok();
        }

        drop(tx_exit);

        thread::sleep(Duration::from_millis(2000));
    });

    t1.join().ok();
    println!("t1 exited");
    t2.join().ok();
}

fn when_do_channels_close() {
    let (tx, rx) = bounded(0);

    let rx_inner = rx.clone();
    let rx_thread = thread::spawn(move || {
        loop {
            if let Ok(msg) = rx_inner.recv() {
                println!("received {msg}");
            } else {
                println!("channel closed");
                break;
            }
        }

        println!("exiting receive loop");
    });

    let tx_thread = thread::spawn(move || {
        for i in 0..6 {
            tx.send(i).ok();
            thread::sleep(Duration::from_millis(500));
        }
    });

    rx_thread.join().ok();
    tx_thread.join().ok();

    if rx.recv().is_err() {
        println!("channel is definitely closed");
    }
}

fn throttle_test() {
    // let mut throttle = Throttle::new();
}

fn select_channels() {
    let (tx1, rx1) = unbounded();
    let (tx2, rx2) = unbounded();

    let t1 = thread::spawn(move|| {
        thread::sleep(Duration::from_millis(500));
        tx1.send("t1").unwrap();
    });
    let t2 = thread::spawn(move|| {
        thread::sleep(Duration::from_millis(3000));
        tx2.send("t2").unwrap();
    });

    select! {
        recv(rx1) -> msg => println!("{}", msg.unwrap()),
        recv(rx2) -> msg => println!("{}", msg.unwrap()),
    }

    t1.join().unwrap();
    t2.join().unwrap();
}

fn bounded_zero_channels() {
    let (tx, rx) = bounded(0);

    let thread_tx = thread::spawn(move|| {
        let mut counter = 0usize;
        loop {
            thread::sleep(Duration::from_millis(500));
            if tx.try_send(counter).is_err() {
                println!("unable to send {counter}. skipping");
            }
            counter += 1;
        }
    });

    let thread_rx = thread::spawn(move|| {
        loop {
            let msg = rx.recv().unwrap();
            println!("received {msg}");
            thread::sleep(Duration::from_millis(2000));
        }
    });

    thread_tx.join().unwrap();
    thread_rx.join().unwrap();
}

fn multiple_consumers() {
    let rx = tick(Duration::from_secs(1));

    let rx_t1 = rx.clone();
    let t1 = thread::spawn(move|| {
        for msg in rx_t1.iter() {
            println!("t1: received {msg:?}");
        }
    });

    let rx_t2 = rx.clone();
    let t2 = thread::spawn(move|| {
        for msg in rx_t2.iter() {
            println!("t2: received {msg:?}");
        }
    });

    // multiple consumers from the channel, but only the first one in gets the message. So in this case, the threads
    // alternate getting the message

    t1.join();
    t2.join();
}

fn type_narrow_example() {
    let (tx, rx) = unbounded::<DownloadOutput>();

    narrow_send(tx);
    narrow_recv(rx);
}

fn narrow_send<T : From<Vec<u8>>>(tx: Sender<T>) {
    let vec: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 6, 7];
    tx.send(vec.into()).ok();
}

fn narrow_recv<T : Into<Vec<u8>> + Debug>(rx: Receiver<T>) {
    let msg: Vec<u8> = rx.recv().unwrap().into();
    println!("received {:?}", msg);
}
