use crate::ui::*;
use sqlx::{MySql, Pool, Row};

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
