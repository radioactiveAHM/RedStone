use crate::server::server;
use redstone::ipfinder;

use cursive::align::HAlign;
use cursive::traits::Scrollable;
use cursive::view::Resizable;
use cursive::views::{Dialog, SelectView, TextView};

pub fn server_btn(s: &mut cursive::Cursive) {
    // Clear layer
    s.pop_layer();
    // Create selectview
    let mut select = SelectView::new().h_align(HAlign::Center).autojump();
    // Get list of IPs and add to selectview
    select.add_all(ipfinder());
    // Add functionality to selectview on select
    select.set_on_submit(|s, c| {
        s.pop_layer();
        // Show the IP listening to
        s.add_layer(
            TextView::new(format!("Server listening on {c}:1140"))
                .h_align(HAlign::Center)
                .fixed_height(5),
        );
        // IP + Port = IP:Port
        let ip_port = format!("{}:1140", c.clone());
        // Run server thread
        tokio::spawn(async { server(ip_port).await });
    });
    s.add_layer(Dialog::around(select.scrollable()).title("Select Adaptor"));
}
