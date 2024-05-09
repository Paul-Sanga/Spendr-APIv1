pub mod app_state;
pub mod database;
mod middleware;
mod queries;
mod routes;
mod utilities;

use app_state::AppState;
use routes::create_routes;
use std::net::Ipv4Addr;

pub struct App {
    port: u16,
    address: Ipv4Addr,
}

impl App {
    pub fn new(port: u16) -> Self {
        let address = Ipv4Addr::new(127, 0, 0, 1);
        Self { port, address }
    }

    pub async fn run(&self, app_state: AppState) {
        let tcp_listener = tokio::net::TcpListener::bind((self.address, self.port))
            .await
            .unwrap();
        let app = create_routes(app_state);

        axum::serve(tcp_listener, app.into_make_service())
            .await
            .unwrap();
    }
}
