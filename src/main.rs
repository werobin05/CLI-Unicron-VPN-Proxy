mod ui;
mod structs;
mod service;

#[tokio::main]
async fn main() {
    service::run_cli().await;
}
