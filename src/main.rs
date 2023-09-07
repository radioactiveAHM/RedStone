mod client;
mod server;

use client::client;
use server::server;
use redstone::read;

#[tokio::main]
async fn main(){
    match read("Choose mod: Server(1), Client(2)").replace("\r\n", "").as_str() {
        "1" => {server(read("Enter ip:port")).await},
        "2" => {client(read("Enter ip:port")).await},
        _ => {panic!("X")}
    }
}