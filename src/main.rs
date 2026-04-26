mod os;
mod service;
mod core;

#[tokio::main]
async fn main() {
    service::server::start_server().await;
}