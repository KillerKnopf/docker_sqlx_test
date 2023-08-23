use dotenv::dotenv;
use sqlx::mysql::MySqlPoolOptions;
use std::env;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub mod db_stuff;
pub mod ui;
enum MainMenuAction {
    ListDatabases,
    CreateDatabase,
    DeleteDatabase,
    ConnectToDatabase,
    Exit,
}

#[tokio::main]
async fn main() {
    // setup logging crates
    initialize_logging(Level::DEBUG);

    // load in .env file
    dotenv().ok();

    // initialize connection
    let dbms_url = env::var("DBMS_URL").expect("Did not find DBMS_URL in envs");
    let pool = MySqlPoolOptions::new()
        .max_connections(1)
        .connect(&dbms_url)
        .await
        .expect("Failed to establish database connection during program initialization.");

    // programm loop
    loop {
        // display main menu
        ui::write_main_menu();

        // get user input for choosing what to do
        let input = get_main_menu_action();

        // match user input
        match input {
            // This arm lists all databases
            MainMenuAction::ListDatabases => {
                db_stuff::list_databases(&pool).await;
            }
            // This arm creates a database
            MainMenuAction::CreateDatabase => {
                db_stuff::create_database(&pool).await;
            }
            // This arm creates a database
            MainMenuAction::DeleteDatabase => {
                db_stuff::delete_database(&pool).await;
            }
            // This arm connects to a database
            MainMenuAction::ConnectToDatabase => {
                println!("\n     Connecting to database:");
                ui::pause_console();
            }
            // This arm exists the program
            MainMenuAction::Exit => {
                println!("\n\n Exiting CLI tool.\n Have a nice day.\n\n");
                break;
            }
        }
    }
}

fn get_main_menu_action() -> MainMenuAction {
    loop {
        // get user input from console
        let input = ui::get_input();

        // create MainMenuAction from slice of input
        match input.to_uppercase().as_str() {
            "A" => return MainMenuAction::ListDatabases,
            "B" => return MainMenuAction::CreateDatabase,
            "C" => return MainMenuAction::DeleteDatabase,
            "D" => return MainMenuAction::ConnectToDatabase,
            "E" => return MainMenuAction::Exit,
            _ => {
                println!("       Unrecognized input. Please try again.\n       -----");
                print!("     Please type in the letter corresponding to your choice --> ");
                ui::flush();
            }
        }
    }
}

fn initialize_logging(level: Level) {
    // Includes Backtrace feature when running the programm
    // Backtrace shows call stack when panic!
    // 0 = disabled (no backtrace)
    // 1 = partial call stack
    // full = full call stack
    env::set_var("RUST_BACKTRACE", "0");
    // env::set_var("RUST_BACKTRACE", "1");
    // env::set_var("RUST_BACKTRACE", "full");

    // Sets-up eyre to generate colorful reports on any panic
    color_eyre::install().unwrap();

    // Setting up Tracing
    // Builder for creating a Subscriber instance
    // A Subscriber is used by Tracing to collect data and log it (e.g. to standard output)
    let subscriber = FmtSubscriber::builder().with_max_level(level).finish();

    // Set defaults for the subscriber
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set up default subscriber");
}
