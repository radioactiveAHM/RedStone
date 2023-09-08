use crate::client::client;
use cursive::{
    view::Nameable,
    views::{Dialog, TextArea},
};

pub fn client_btn(s: &mut cursive::Cursive) {
    // Clear layer
    s.pop_layer();
    // Add input
    s.add_layer(
        Dialog::new()
            .title("IP:Port")
            .padding_lrtb(1, 1, 1, 0)
            .content(TextArea::new().with_name("ipport"))
            // Add functionality to button
            .button("Connect", |s| {
                // Get the ip:port from input
                s.call_on_name("ipport", |v: &mut TextArea| {
                    let temp = v.get_content().to_string();
                    // Spawn Client thread
                    tokio::spawn(async { client(temp).await });
                });
            }),
    );
}
