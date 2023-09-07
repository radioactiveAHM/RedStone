pub mod client;
pub mod server;
mod client_btn;
mod server_btn;

use cursive::views::{Dialog, TextView};

use client_btn::client_btn;
use server_btn::server_btn;

#[tokio::main]
async fn main() {
    // Creates the cursive root
    let mut siv = cursive::default();

    siv.add_layer(
        Dialog::around(TextView::new("RedStone"))
            .title("Choose mode")
            .button("Server", server_btn)
            .button("Client", client_btn),
    );

    // Starts the event loop.
    siv.run();
}
