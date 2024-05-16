use dotenvy::dotenv;
use sea_orm::Database;
use spendr_api::{app_state::AppState, App};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = if let Ok(database_url) = dotenvy::var("DATABASE_URL") {
        database_url
    } else {
        panic!("\x1b[31m database url not specified \x1b[0m")
    };
    let db = match Database::connect(database_url).await {
        Ok(db) => db,
        Err(error) => {
            eprintln!(
                "\x1b[31m Error connecting to the database: {:?} \x1b[0m",
                error
            );
            panic!()
        }
    };

    let app_state = AppState { db };

    let port = if let Ok(port) = dotenvy::var("PORT") {
        port.parse::<u16>().expect("Port should be an integer")
    } else {
        eprintln!("\x1b[38;5;11m port has not been specified \x1b[0m");
        8080u16
    };
    let app = App::new(port);
    app.run(app_state).await
}
