use std::io::{BufRead, BufReader, Write};
#[allow(unused_imports)]
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");
                handle_connection(_stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let tokens: Vec<&str> = request_line.split_whitespace().collect();
    let response = match tokens[0] {
        "GET" => {
            if tokens[1] == "/" {
                format!("HTTP/1.1 200 OK\r\n\r\n")
            } else if &tokens[1][..6] == "/echo/" {
                format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                    tokens[1][6..].len(),
                    &tokens[1][6..]
                )
            } else {
                format!("HTTP/1.1 404 Not Found\r\n\r\n")
            }
        }
        _ => format!("HTTP/1.1 404 Not Found\r\n\r\n"),
    };

    stream.write_all(response.as_bytes()).unwrap();
}
