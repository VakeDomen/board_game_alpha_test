use server::wss::start_server;

mod game;
mod server;
mod storage;


#[tokio::main]
async fn main() {
    let listen_addr = "127.0.0.1:8080";
    start_server(listen_addr);
}
