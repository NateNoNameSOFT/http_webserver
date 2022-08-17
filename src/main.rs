use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    {process, fs, thread},
    time::Duration,
};

use http_webserver::ThreadPool;

fn main() {
    // Create a new TcpListener, the bind method will return a new TcpListener 
    // instance that will be bound to the port 7878
    let listener = match TcpListener::bind("127.0.0.1:7878") {
        Ok(v) => v,
        Err(e) => {
            println!("Port could not be bound: {e}");
            process::exit(1);
        }
    };

    let pool = ThreadPool::build(12).unwrap();

    // Iterate through each each stream between the client and the server, this
    // could also be seen as iterating between each connection attempt.
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

/// Read data from the TCP stream and print it.
fn handle_connection(mut stream: TcpStream) {
    // Create a new BufRead instance that wraps a mutable reference to the stream
    let buf_reader = BufReader::new(&mut stream);
    // Read the first line of the HTTP request
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // Check if the request
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => {
            // Status line part of a response that uses HTTP version 1.1, has a 
            // status code of 200, and an OK reason phrase, no headers, and no body
            ("HTTP/1.1 200 OK", "hello.html")
        }
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    // Read the HTML document into a String
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    
    // Format the files contents as the body of the success response
    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );

    // Send the response data as bytes directly down the connection.
    stream.write_all(response.as_bytes()).unwrap();
}
