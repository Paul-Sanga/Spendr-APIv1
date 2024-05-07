use dotenvy::dotenv;
use dotenvy_macro::{self, dotenv};
use sea_orm::Database;
use spendr_api::{app_state::AppState, App};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = dotenv!("DATABASE_URL");
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

    let port = dotenv!("PORT")
        .parse::<u16>()
        .expect("Port should be an integer");

    let app = App::new(port);
    app.run(app_state).await
}
