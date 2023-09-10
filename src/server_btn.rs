use crate::server::server;

use cursive::{
    align::HAlign,
    traits::Scrollable,
    view::Resizable,
    views::{Dialog, SelectView, TextView},
};

pub fn server_btn(s: &mut cursive::Cursive) {
    // Clear layer
    s.pop_layer();
    // Create selectview
    let mut select = SelectView::new().h_align(HAlign::Center).autojump();
    // Get list of IPs and add to selectview
    select.add_all(local_ip_address::list_afinet_netifas().expect("Error getting ip"));
    // Add functionality to selectview on select
    select.set_on_submit(|s, c| {
        // IP + Port = IP:Port
        let ip_port = format!("{c}:1140");
        s.pop_layer();
        // Show the IP listening to
        s.add_layer(
            TextView::new(format!("Server listening on {ip_port}"))
                .h_align(HAlign::Center)
                .fixed_height(5),
        );
        // Run server thread
        tokio::spawn(async { server(ip_port).await });
    });
    s.add_layer(Dialog::around(select.scrollable()).title("Select Adaptor"));
}
