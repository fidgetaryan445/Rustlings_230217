use std::{
    fs,
    io::{prelude::*,BufReader, ErrorKind} ,
    net::{TcpListener ,TcpStream} 
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7848").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream:TcpStream){
    let buf_read = BufReader::new(&mut stream) ;
    let http_request : Vec<_> = buf_read
    .lines()
    .map(|result|result.unwrap())
    .take_while(|line|!line.is_empty())
    .collect();
    
    if check_400(&http_request) {
        let status_line = "HTTP/1.1 400 BAD REQUEST";
        let content = "The request made is invalid";
        let length = content.len() ;
        let response = format!("{status_line}\r\nContent-Length={length}\r\n\r\n{content}");
        stream.write_all(response.as_bytes()).unwrap();
        return ; 
    }
    if http_request[0]=="GET / HTTP/1.1" {
     let status_line = "HTTP/1.1 200 OK" ;
     let content =fs::read_to_string("./hello.html").unwrap();
     let length = content.len() ;
     let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");
     stream.write_all(response.as_bytes()).unwrap() ;
   } 
   else if http_request[0]== "GET /read HTTP/1.1" {
       let content = match fs::read_to_string("./permission") {
            Ok(contents) => {
                let status_line = "HTTP/1.1 200 OK";
                let length = contents.len();
                format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}")
            }
            Err(error) => {
                let status_line = if error.kind() == ErrorKind::NotFound {
                    "HTTP/1.1 404 NOT FOUND"
                } else if error.kind() == ErrorKind::PermissionDenied {
                    "HTTP/1.1 403 FORBIDDEN"
                } else {
                    "HTTP/1.1 500 INTERNAL SERVER ERROR"
                };
                let content =  if error.kind() == ErrorKind::NotFound {
                    "PAGE NOT FOUND".to_string()
                } else if error.kind() == ErrorKind::PermissionDenied {
                    "Permission Denied".to_string()
                } else {
                    "Internal Server Error".to_string()
                };
                let length = content.len();
                format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}")
            }
        };
        stream.write_all(content.as_bytes()).unwrap();
   } else if http_request[0].starts_with("GET") {
    let status_line= "HTTP/1.1 404 NOT FOUND";
    let content = fs::read_to_string("./404.html").unwrap() ;
    let length = content.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}") ;
    stream.write_all(response.as_bytes()).unwrap();
   } else {
   
        let status_line = "HTTP/1.1 405 NOT ALLOWED";
        let content = "The method used is not allowed.";
        let length = content.len() ;
        let response = format!("{status_line}\r\nContent-Length={length}\r\n\r\n{content}");
        stream.write_all(response.as_bytes()).unwrap();
        
   }



}

fn check_400(http_request : &Vec<String>) -> bool {
    if http_request.is_empty() {
        return true ; 
    }
    let  req_line = http_request[0].clone() ;
    let req_line_parts : Vec<&str> = req_line.split_whitespace().collect() ;

    if !["GET", "POST", "PUT", "DELETE", "HEAD", "OPTIONS", "PATCH"].contains(&req_line_parts[0]) {
       return true ; 
    } 
    if !req_line_parts[2].starts_with("HTTP/1."){
        return true; 
    }
    false 
}