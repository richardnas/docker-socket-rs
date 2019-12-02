use std::io::prelude::*;
use std::os::unix::net::UnixStream;

fn main() {
    
    let mut stream = UnixStream::connect("/var/run/docker.sock").unwrap();
    stream.write_all(b"GET /info HTTP/1.1\r\nHost: v1.40\r\nConnection: close\r\n\r\n").unwrap();
    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();
    println!("{}", response);

}
