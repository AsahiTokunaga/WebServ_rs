use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    fs,
    env,
};
use dotenv::dotenv;

const ALLOW_REQUESTS: [&str; 2] = ["GET / HTTP/1.1", "GET /index.html HTTP/1.1"];

enum HttpResponce {
    OK,
    NotFound,
}

impl ToString for HttpResponce {
    fn to_string(&self) -> String {
        match self {
            HttpResponce::OK => String::from("HTTP/1.1 200 OK"),
            HttpResponce::NotFound => String::from("HTTP/1.1 404 NotFound"),
        }
    }
}

fn main() {
    dotenv().ok();
    let listener = TcpListener::bind(env::var("LISTENER").unwrap()).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handler(stream);
    }
}

fn handler(mut stream: TcpStream) {
    let reader = BufReader::new(&stream);
    let request_line = reader.lines().next().unwrap().unwrap();

    println!("Request: {}", request_line);

    let (status_line, contents) = if ALLOW_REQUESTS.iter().any(|r| *r == request_line) {
        (HttpResponce::OK.to_string(), fs::read_to_string("index.html").unwrap())
    } else {
        (HttpResponce::NotFound.to_string(), fs::read_to_string("not_found.html").unwrap())
    };
    let contents_length = contents.len();

    let responce = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, contents_length, contents);

    stream.write_all(responce.as_bytes()).unwrap();
}

