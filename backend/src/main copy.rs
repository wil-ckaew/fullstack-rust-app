use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

mod db;
mod handlers;
mod models;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut request = String::new();

    match stream.read(&mut buffer) {
        Ok(size) => {
            request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());

            let (status_line, content) = match &*request {
                r if r.starts_with("OPTIONS") => (OK_RESPONSE.to_string(), "".to_string()),
                r if r.starts_with("POST /api/rust/users") => handlers::users::handle_post_request(r),
                r if r.starts_with("GET /api/rust/users/") => handlers::users::handle_get_request(r),
                r if r.starts_with("GET /api/rust/users") => handlers::users::handle_get_all_request(r),
                r if r.starts_with("PUT /api/rust/users/") => handlers::users::handle_put_request(r),
                r if r.starts_with("DELETE /api/rust/users/") => handlers::users::handle_delete_request(r),
                r if r.starts_with("POST /api/rust/documents") => handlers::documents::handle_post_request(r),
                r if r.starts_with("GET /api/rust/documents/") => handlers::documents::handle_get_request(r),
                r if r.starts_with("GET /api/rust/documents") => handlers::documents::handle_get_all_request(r),
                r if r.starts_with("DELETE /api/rust/documents/") => handlers::documents::handle_delete_request(r),
                _ => (NOT_FOUND.to_string(), "404 not found".to_string()),
            };

            stream.write_all(format!("{}{}", status_line, content).as_bytes()).unwrap();
        }
        Err(e) => eprintln!("Unable to read stream: {}", e),
    }
}

const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET, POST, PUT, DELETE\r\nAccess-Control-Allow-Headers: Content-Type\r\n\r\n";
const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const INTERNAL_ERROR: &str = "HTTP/1.1 500 INTERNAL ERROR\r\n\r\n";

fn main() {
    // Initialize Database
    if let Err(_) = db::init_database() {
        println!("Error setting up database");
        return;
    }

    // Start server and print port
    let listener = TcpListener::bind(format!("0.0.0.0:8080")).unwrap();
    println!("Server listening on port 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}
