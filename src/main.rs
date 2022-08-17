use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    process,
};

fn main() {
    // Create a new TcpListener
    let listener = 
        // Returns a new TcpListener instance that will be bound to the port 7878
        match TcpListener::bind("127.0.0.1:7878") {
            Ok(v) => v,
            Err(e) => {
                println!("Port could not be bound: {e}");
                process::exit(1);
            }
        };

    // Iterate through each each stream between the client and the server, this
    // could also be seen as iterating between each connection attempt.
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

/// Read data from the TCP stream and print it.
fn handle_connection(mut stream: TcpStream) {
    // Create a new BufRead instance that wraps a mutable reference to the stream
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        // An iterator that splits the stream of data whenever it sees a newline
        // byte. Returns a result containing a String
        .lines()
        // Unwrap each map to get each String contained
        .map(|result| result.unwrap())
        // Take lines until we get a line that is an empty String, since the 
        // browser signals the end of an HTTP request by sending two newline
        // character in a row.
        .take_while(|line| !line.is_empty())
        // Collect the lines into a vector
        .collect();

    println!("Request: {:#?}", http_request);
}
