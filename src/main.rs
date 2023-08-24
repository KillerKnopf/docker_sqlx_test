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
    ReloadMainMenu,
    Exit,
}

#[tokio::main]
async fn main() {
    // setup logging crates
    initialize_logging(Level::DEBUG);

    // Load .env file into environment variables
    dotenvy::dotenv().ok();

    // initialize first connection
    let connection = db_stuff::Connection::new(db_stuff::ConnectionDetails::from_env()).await;

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
                db_stuff::list_databases(&connection.pool).await;
            }
            // This arm creates a database
            MainMenuAction::CreateDatabase => {
                db_stuff::create_database(&connection.pool).await;
            }
            // This arm creates a database
            MainMenuAction::DeleteDatabase => {
                db_stuff::delete_database(&connection.pool).await;
            }
            // This arm connects to a database
            MainMenuAction::ConnectToDatabase => {
                println!("\n     Connecting to database:");
                ui::pause_console();
            }
            // This arm rewrites the main menu if problems arised
            // This is the case for example when running the program in a docker container and attaching later
            MainMenuAction::ReloadMainMenu => {
                ui::write_main_menu();
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
            "E" => return MainMenuAction::ReloadMainMenu,
            "F" => return MainMenuAction::Exit,
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
