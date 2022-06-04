use std::fs;
use std::thread;
use std::time::Duration;
use std::path::Path;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use mtws::ThreadPool;


fn main() {
    let host = "127.0.0.1";
    let port =      "7878";
    let addr = format!("{}:{}", host, port);

    let listener = match TcpListener::bind(addr) {
        Ok (l) => l,
        Err(_) => panic!("Could not establish connection.")
    };

    println!("Connection established!");

    let mut pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        println!("{:?}", stream);
        pool.execute(|| handle_connection(stream));

    }
}


fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let http_version   =  "1.1";

    let (status_code, status_message, filename) = handle_request(&buffer[..]);

    let body     = fs::read_to_string(filename).unwrap();
    let headers  = format!("Content-Length: {}", body.len());
    let response = format!( "HTTP/{} {} {}\r\n{}\r\n\r\n{}"
                          , http_version
                          , status_code
                          , status_message
                          , headers
                          , body
                          );

    println!("Response: {}", response);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}

fn is_get_request(buffer : &[u8]) -> bool {
    return buffer.starts_with(b"GET ");
}

fn get_path_from_request(buffer : &[u8]) -> &[u8] {
    return buffer.split(|c| b' ' == *c).nth(1).unwrap();
}

fn handle_request(buffer : &[u8]) -> (i16, String, String) {
    let not_found = (404, "NOT FOUND".to_string(), "html/404.html".to_string());

    if !is_get_request(buffer) {
        return not_found;
    }
    else {
        let path = get_path_from_request(buffer);
        if let Some((b'/', rest)) = path.split_first() {
            let mut rest = String::from_utf8_lossy(&rest[..]).to_string();
            if rest == "" {
                rest = "homepage".to_string();
            }
            if rest == "sleep" {
                thread::sleep(Duration::from_secs(5));
                rest = "hello".to_string();
            }

            let filename = format!("html/{}.html", rest);
            if !Path::new(&filename).exists() {
                return not_found;
            }
            return (200, rest, filename);
        }
        return not_found;
    }
}
