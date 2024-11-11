use std::{fmt::format, fs, io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream}};

fn main(){
    let listner = TcpListener::bind("127.0.0.1:3000").unwrap();

    for stream in listner.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}


fn handle_connection(mut stream: TcpStream){
    //HTTP request format:
    // Method Request-URI HTTP-Version CRLF
    // headers CRLF
    // message-body

    //HTTP response format:
    // HTTP-Version Status-Code Reason-Phrase CRLF
    // headers CRLF
    // message-body

    let buf_reader = BufReader::new(&mut stream);
    // let http_req: Vec<_> = buf_reader
    //     .lines()
    //     .map(|res| res.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();

    let req_line = buf_reader
        .lines()
        .next().unwrap().unwrap();

    let (status_line, filename) = if req_line == "GET / HTTP/1.1" {
            ("HTTP/1.1 200 OK", "hello.html")
        } else{
            ("HTTP/1.1 404 NOT FOUND", "not_found.html")
        };

    let contents = fs::read_to_string(filename).unwrap();
    let length= contents.len();

    let response=  format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();



}