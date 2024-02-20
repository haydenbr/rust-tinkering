use std::{net::{TcpListener, TcpStream}, io::{prelude::*, BufReader}, fs, thread, time::Duration, sync::mpsc::channel};

use rpl_web_server::ThreadPool;

fn main() {
    let (tx, rx) = channel::<()>();

    ctrlc::set_handler(move || {
        println!("requesting exit");
        tx.send(()).ok();
    }).ok();

    let listener = TcpListener::bind(("127.0.0.1", 7878)).unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });

        // this doesn't actually work quite as expected because we only reach here on the next request. Ideally, we
        // would have a channel of tcp requests and an exit notification channel that we can select between.
        if rx.try_recv().is_ok() {
            println!("exiting main loop");
            break;
        }
    }

    println!("shutting down");
}

// even though the original stream variable in main is not mutable, notice that it can be mutable when passed to a
// function that expects a mutable input parameter
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // match doesn't automatically ref and deref so we have to call as_str to compare to static string literals
    let (status, file_name) = match request_line.as_str() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index.html")
        },
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(file_name).unwrap();
    let content_length = format!("Content-Length: {}", contents.len());
    let response = format!("{status}\r\n{content_length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
