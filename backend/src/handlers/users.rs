use crate::db::get_db_client;
use crate::models::user::User;
use serde_json;
use postgres::{Client, NoTls};

pub fn handle_post_request(request: &str) -> (String, String) {
    match (get_user_request_body(request), get_db_client()) {
        (Ok(user), Ok(mut client)) => {
            if !is_valid_email(&user.email) {
                return (INTERNAL_ERROR.to_string(), "Invalid email format".to_string());
            }

            match client.query_one(
                "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id",
                &[&user.name, &user.email],
            ) {
                Ok(row) => {
                    let user_id: i32 = row.get(0);
                    match client.query_one("SELECT id, name, email FROM users WHERE id = $1", &[&user_id]) {
                        Ok(row) => {
                            let user = User {
                                id: Some(row.get(0)),
                                name: row.get(1),
                                email: row.get(2),
                            };
                            (OK_RESPONSE.to_string(), serde_json::to_string(&user).unwrap())
                        }
                        Err(_) => (INTERNAL_ERROR.to_string(), "Failed to retrieve created user".to_string()),
                    }
                }
                Err(_) => (INTERNAL_ERROR.to_string(), "Failed to insert user".to_string()),
            }
        }
        _ => (INTERNAL_ERROR.to_string(), "Internal error".to_string()),
    }
}

pub fn handle_get_request(request: &str) -> (String, String) {
    match (get_id(&request).parse::<i32>(), get_db_client()) {
        (Ok(id), Ok(mut client)) =>
            match client.query_one("SELECT * FROM users WHERE id = $1", &[&id]) {
                Ok(row) => {
                    let user = User {
                        id: row.get(0),
                        name: row.get(1),
                        email: row.get(2),
                    };
                    (OK_RESPONSE.to_string(), serde_json::to_string(&user).unwrap())
                }
                _ => (NOT_FOUND.to_string(), "User not found".to_string()),
            }
        _ => (INTERNAL_ERROR.to_string(), "Internal error".to_string()),
    }
}

pub fn handle_get_all_request(_request: &str) -> (String, String) {
    match get_db_client() {
        Ok(mut client) => {
            let mut users = Vec::new(); // Vector to store the users

            for row in client.query("SELECT id, name, email FROM users", &[]).unwrap() {
                users.push(User {
                    id: row.get(0),
                    name: row.get(1),
                    email: row.get(2),
                });
            }

            (OK_RESPONSE.to_string(), serde_json::to_string(&users).unwrap())
        }
        _ => (INTERNAL_ERROR.to_string(), "Internal error".to_string()),
    }
}

pub fn handle_put_request(request: &str) -> (String, String) {
    match (
        get_id(&request).parse::<i32>(),
        get_user_request_body(&request),
        get_db_client(),
    ) {
        (Ok(id), Ok(user), Ok(mut client)) => {
            client
                .execute(
                    "UPDATE users SET name = $1, email = $2 WHERE id = $3",
                    &[&user.name, &user.email, &id],
                )
                .unwrap();

            (OK_RESPONSE.to_string(), "User updated".to_string())
        }
        _ => (INTERNAL_ERROR.to_string(), "Internal error".to_string()),
    }
}

pub fn handle_delete_request(request: &str) -> (String, String) {
    match (get_id(&request).parse::<i32>(), get_db_client()) {
        (Ok(id), Ok(mut client)) => {
            let rows_affected = client.execute("DELETE FROM users WHERE id = $1", &[&id]).unwrap();

            // If rows affected is 0, user not found
            if rows_affected == 0 {
                return (NOT_FOUND.to_string(), "User not found".to_string());
            }

            (OK_RESPONSE.to_string(), "User deleted".to_string())
        }
        _ => (INTERNAL_ERROR.to_string(), "Internal error".to_string()),
    }
}

// Get id from request URL
fn get_id(request: &str) -> &str {
    request.split("/").nth(4).unwrap_or_default().split_whitespace().next().unwrap_or_default()
}

// Deserialize user from request body without id
fn get_user_request_body(request: &str) -> Result<User, serde_json::Error> {
    serde_json::from_str(request.split("\r\n\r\n").last().unwrap_or_default())
}

// Email validation
fn is_valid_email(email: &str) -> bool {
    // Implement email validation logic
    email.contains('@') && email.contains('.')
}

const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET, POST, PUT, DELETE\r\nAccess-Control-Allow-Headers: Content-Type\r\n\r\n";
const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const INTERNAL_ERROR: &str = "HTTP/1.1 500 INTERNAL ERROR\r\n\r\n";
