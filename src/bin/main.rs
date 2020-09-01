use multithreaded_web::ThreadPool;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(5);
    for stream in listener.incoming().take(2){
        let stream = stream.unwrap();

        pool.execute(||
            handle_connection(stream));
        //println!("connection estblished");    
    }
    
} 
fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();
    //println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let (line, file) = if buffer.starts_with(get){
        ("HTTP/1.1 200 OK \r\n\r\n", "file.html")
    }
    else if buffer.starts_with(sleep){
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK \r\n\r\n", "file.html")
    }
    else{
        ("HTTP/1.1 404 NOT-FOUND\r\n\r\n", "404.html")
    };
    
    let content = fs::read_to_string(file).unwrap();
        
    //HTTP version 1.1, has a status code of 200, an OK reason phrase, no headers, and no body    
        let response = format!("{} {}", line,content);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
}