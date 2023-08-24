use std::env;

use crate::ui::*;
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool, Row};
use tracing::debug;

pub struct ConnectionDetails {
    user: String,
    password: String,
    ip: String,
    database: Option<String>,
}

impl ConnectionDetails {
    pub fn new(&mut self, user: &str, password: &str, ip: &str, database: Option<&str>) -> Self {
        ConnectionDetails {
            user: user.to_string(),
            password: password.to_string(),
            ip: ip.to_string(),
            database: database.map(str::to_string),
        }
    }

    pub fn from_env() -> Self {
        let user = env::var("DB_USER").expect("DB_USER not found in environment variables.");
        let password =
            env::var("DB_PASSWORD").expect("DB_PASSWORD not found in environment variables.");
        let ip = env::var("DB_ADDRESS").expect("DB_ADDRESS not found in environment variables");
        let database: Option<String> = None;

        ConnectionDetails {
            user,
            password,
            ip,
            database,
        }
    }
}

pub struct Connection {
    pub details: ConnectionDetails,
    pub pool: Pool<MySql>,
}

impl Connection {
    pub async fn new(details: ConnectionDetails) -> Self {
        let dbms_url = format!(
            "mysql://{}:{}@{}/{}",
            details.user,
            details.password,
            details.ip,
            details.database.as_deref().unwrap_or("")
        );
        debug!("\n Created first dbms_url --> {}\n", dbms_url);

        let pool = MySqlPoolOptions::new()
            .max_connections(1)
            .connect(&dbms_url)
            .await
            .expect("Failed to establish first database connection.");

        Connection { details, pool }
    }

    pub async fn switch_connection(
        &mut self,
        user: &str,
        password: &str,
        ip: &str,
        database: Option<&str>,
    ) {
        self.details.new(user, password, ip, database);

        let dbms_url = format!(
            "mysql://{}:{}@{}/{}",
            self.details.user,
            self.details.password,
            self.details.ip,
            self.details.database.as_deref().unwrap_or("")
        );
        debug!("\n Switched dbms_url --> {}\n", dbms_url);

        self.pool.close().await;
        debug!("\n Closed old connection");

        self.pool = MySqlPoolOptions::new()
            .max_connections(1)
            .connect(&dbms_url)
            .await
            .expect("Failed to establish new database connection.");
        debug!("\n Created new connection");
    }
}

pub async fn list_databases(pool: &Pool<MySql>) {
    let rows = sqlx::query("SHOW DATABASES;")
        .fetch_all(pool)
        .await
        .unwrap_or_default();

    write_title();
    println!("\n Listing all existing Databases");
    println!(" -----");
    for row in rows {
        let db: String = row.try_get("Database").unwrap_or_default();
        println!("   -> {}", db);
    }

    pause_console();
}

pub async fn create_database(pool: &Pool<MySql>) {
    // Write new cli page
    write_title();
    println!("\n Creating a new Database");
    println!(" -----");
    println!("\n   Please enter the name for the new database.");
    println!("   Dashes will be replaced by underscores.");
    print!("\n     Database name --> ");
    flush();

    // get user input from console
    let db_name = get_input();
    // Change name to lowercase. This makes it easier to work with and mysql will turn it into lowercase anyway.
    let mut db_name_edited = db_name.to_lowercase();

    // Replace all "-" with "_" otherwise the query will crash
    db_name_edited = db_name_edited.replace("-", "_");
    // Remove all ";" to prevent some sql injections
    db_name_edited = db_name_edited.replace(";", "");
    // Remove all "select"
    db_name_edited = db_name_edited.replace("select", "");
    // Remove all "union"
    db_name_edited = db_name_edited.replace("union", "");

    // building the query string
    let sql = format!("CREATE DATABASE {};", db_name_edited);

    let result = sqlx::query(&sql).bind(db_name_edited).execute(pool).await;

    match result {
        Ok(_) => {
            println!("\n     Database {} successfully created.", db_name);
        }
        Err(e) => {
            println!("\n       Failed to create database.");
            println!("\n       {}", e.to_string());
        }
    }

    pause_console();
}

pub async fn delete_database(pool: &Pool<MySql>) {
    // Write new cli page
    write_title();
    println!("\n Deleting a Database");
    println!(" -----");
    println!("\n   Please enter the name of the database which you want deleted.");
    println!("   Dashes will be replaced by underscores.");
    print!("\n     Database name --> ");
    flush();

    // get user input from console
    let db_name = get_input();
    // Change name to lowercase. This makes it easier to work with and mysql will turn it into lowercase anyway.
    let mut db_name_edited = db_name.to_lowercase();

    // Replace all "-" with "_" otherwise the query will crash and no database has a dash
    db_name_edited = db_name_edited.replace("-", "_");

    // building the query string
    let sql = format!("DROP DATABASE {};", db_name_edited);

    let result = sqlx::query(&sql).bind(db_name_edited).execute(pool).await;

    match result {
        Ok(_) => {
            println!("\n     Database {} successfully deleted.", db_name);
        }
        Err(e) => {
            println!("\n       Failed to delete database.");
            println!("\n       {}", e.to_string());
        }
    }
}
