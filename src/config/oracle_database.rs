use oracle::{Connection, Error, ConnParam};
use std::env;
use dotenv::dotenv;
pub struct OracleDatabase;

impl OracleDatabase {
    pub fn get_connection() -> Result<Connection, Error> {
        dotenv().ok();
        let username = env::var("DB_USERNAME").expect("DB_USERNAME must be set");
        let password = env::var("DB_PASSWORD").expect("DB_PASSWORD must be set");
        let connection_string = env::var("DB_CONNECTION_STRING").expect("DB_CONNECTION_STRING must be set");

        println!("Username: {}", username);
        println!("password: {}", password);
        println!("connection_string: {}", connection_string);
        Connection::connect(
            username.as_str(),
            password.as_str(),
            connection_string.as_str(),
            &[ConnParam::Sysdba],
        )
    }
}