use crate::db::get_db_client;
use crate::models::document::Document;
use serde_json;
use postgres::{Client, NoTls};

pub fn handle_post_request(request: &str) -> (String, String) {
    match (get_document_request_body(request), get_db_client()) {
        (Ok(document), Ok(mut client)) => {
            match client.query_one(
                "INSERT INTO documents (user_id, filename, data) VALUES ($1, $2, $3) RETURNING id",
                &[&document.user_id, &document.filename, &document.data],
            ) {
                Ok(row) => {
                    let document_id: i32 = row.get(0);
                    match client.query_one("SELECT id, user_id, filename, data FROM documents WHERE id = $1", &[&document_id]) {
                        Ok(row) => {
                            let document = Document {
                                id: Some(row.get(0)),
                                user_id: row.get(1),
                                filename: row.get(2),
                                data: row.get(3),
                            };
                            (OK_RESPONSE.to_string(), serde_json::to_string(&document).unwrap())
                        }
                        Err(_) => (INTERNAL_ERROR.to_string(), "Failed to retrieve created document".to_string()),
                    }
                }
                Err(_) => (INTERNAL_ERROR.to_string(), "Failed to insert document".to_string()),
            }
        }
        _ => (INTERNAL_ERROR.to_string(), "Internal error".to_string()),
    }
}

pub fn handle_get_request(request: &str) -> (String, String) {
    match (get_id(&request).parse::<i32>(), get_db_client()) {
        (Ok(id), Ok(mut client)) =>
            match client.query_one("SELECT * FROM documents WHERE id = $1", &[&id]) {
                Ok(row) => {
                    let document = Document {
                        id: row.get(0),
                        user_id: row.get(1),
                        filename: row.get(2),
                        data: row.get(3),
                    };
                    (OK_RESPONSE.to_string(), serde_json::to_string(&document).unwrap())
                }
                _ => (NOT_FOUND.to_string(), "Document not found".to_string()),
            }
        _ => (INTERNAL_ERROR.to_string(), "Internal error".to_string()),
    }
}

pub fn handle_get_all_request(_request: &str) -> (String, String) {
    match get_db_client() {
        Ok(mut client) => {
            let mut documents = Vec::new(); // Vector to store the documents

            for row in client.query("SELECT id, user_id, filename, data FROM documents", &[]).unwrap() {
                documents.push(Document {
                    id: row.get(0),
                    user_id: row.get(1),
                    filename: row.get(2),
                    data: row.get(3),
                });
            }

            (OK_RESPONSE.to_string(), serde_json::to_string(&documents).unwrap())
        }
        _ => (INTERNAL_ERROR.to_string(), "Internal error".to_string()),
    }
}

pub fn handle_delete_request(request: &str) -> (String, String) {
    match (get_id(&request).parse::<i32>(), get_db_client()) {
        (Ok(id), Ok(mut client)) => {
            let rows_affected = client.execute("DELETE FROM documents WHERE id = $1", &[&id]).unwrap();

            // If rows affected is 0, document not found
            if rows_affected == 0 {
                return (NOT_FOUND.to_string(), "Document not found".to_string());
            }

            (OK_RESPONSE.to_string(), "Document deleted".to_string())
        }
        _ => (INTERNAL_ERROR.to_string(), "Internal error".to_string()),
    }
}

// Get id from request URL
fn get_id(request: &str) -> &str {
    request.split("/").nth(4).unwrap_or_default().split_whitespace().next().unwrap_or_default()
}

// Deserialize document from request body without id
fn get_document_request_body(request: &str) -> Result<Document, serde_json::Error> {
    serde_json::from_str(request.split("\r\n\r\n").last().unwrap_or_default())
}

const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET, POST, PUT, DELETE\r\nAccess-Control-Allow-Headers: Content-Type\r\n\r\n";
const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const INTERNAL_ERROR: &str = "HTTP/1.1 500 INTERNAL ERROR\r\n\r\n";
