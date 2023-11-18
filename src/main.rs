use std::{net::{TcpListener, TcpStream, Shutdown}, io::{Read, Write, BufWriter}, time::Duration, fs::{self, File}, error::Error};

fn main() {
    let listen_address = String::from("127.0.0.1:8080");
    let listener = start_listener(listen_address);
    for stream in listener.incoming(){
        match stream { 
            Ok(ok) => handle_tcp_stream(ok),
            Err(error) => println!("Error creating tcp connection {:?}", error),
        }
    }
}

pub fn handle_tcp_stream(mut stream: TcpStream){
    let mut data:Vec<u8> = Vec::new();
    stream.set_read_timeout(Some(Duration::new(0,50_000_000))).expect("Count not set read timeout");
    
    println!("Handling request bytes read {:?}", stream.read_to_end(&mut data));
    println!("Request data {:?}\n", byte_to_string(data));

    not_found_response(&mut stream);
    
    stream.shutdown(Shutdown::Both).expect("Could not close conention");
}

pub fn start_listener(address: String) -> TcpListener{
    let tcp_listen = TcpListener::bind(&address);
    println!("Atempting to bind tcp listner to {:?}\n", &address);

    match tcp_listen {
        Ok(tcp_listener) => return tcp_listener,
        Err(error) => panic!("Failed to create tcp listener {:?}\n", error)
    }
}

pub fn byte_to_string(bytes: Vec<u8>) -> String{
    let output = String::from_utf8(bytes).expect("Found invalid UTF-8");
    output
}

pub fn parse_http_request(request: String){
    
}

pub fn not_found_response(stream: &mut TcpStream){
    let http_code = String::from("HTTP/1.1 200 OK\r\n\r\n");

    let message = String::from("");
   
    stream.write(message.as_bytes()).expect("Failed to write to stream");
}

pub fn load_resource(path: String) -> Result<String, String>{
    let file = File::open(path);
    let mut content: String;
    let result
    match file {
        Ok(temp) => temp.read_to_string(&mut content),
        Err(error) => 
    }
}

struct HttpRequest{
    method: Method,
    requested_path: String
}

enum Method {
    GET
}
