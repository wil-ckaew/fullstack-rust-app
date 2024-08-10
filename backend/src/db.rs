use postgres::{Client, NoTls, Error as PostgresError};
use std::env;

fn get_database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub fn init_database() -> Result<(), PostgresError> {
    let mut client = Client::connect(&get_database_url(), NoTls)?;

    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            email VARCHAR NOT NULL
        );
        CREATE TABLE IF NOT EXISTS documents (
            id SERIAL PRIMARY KEY,
            user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
            filename VARCHAR NOT NULL,
            data_ent VARCHAR NOT NULL
        );
        "
    )?;

    Ok(())
}

pub fn get_db_client() -> Result<Client, PostgresError> {
    Client::connect(&get_database_url(), NoTls)
}