use rfd::FileDialog;
use std::ffi::OsStr;
use tokio::io::AsyncWrite;

struct UdpWriter<'a> {
    udp: &'a tokio::net::UdpSocket,
}
impl AsyncWrite for UdpWriter<'_> {
    fn poll_write(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<Result<usize, std::io::Error>> {
        self.udp.poll_send(cx, buf)
    }
    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        std::task::Poll::Ready(Ok(()))
    }
    fn poll_shutdown(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        std::task::Poll::Ready(Ok(()))
    }
}

pub async fn client(ip_port: String) {
    // Prompt user to select a file
    let file_path = FileDialog::new()
        .set_directory("/")
        .pick_file()
        .expect("❌ No file selected");

    println!("📁 Selected file: {:?}", file_path);

    // Open the selected file
    let mut file = tokio::fs::File::open(&file_path)
        .await
        .unwrap_or_else(|e| panic!("❌ Failed to open file {:?}: {}", file_path, e));

    // Bind to a local UDP socket
    let udp = tokio::net::UdpSocket::bind("0.0.0.0:0")
        .await
        .unwrap_or_else(|e| panic!("❌ Failed to bind local UDP socket: {}", e));

    // Connect to the server
    udp.connect(ip_port.trim()).await.unwrap_or_else(|e| {
        panic!(
            "❌ Failed to connect to server at {}: {}",
            ip_port.trim(),
            e
        )
    });

    println!("🔗 Connected to server at {}", ip_port.trim());

    // Send the filename to the server
    let filename = file_path
        .file_name()
        .unwrap_or(OsStr::new("noname"))
        .as_encoded_bytes();

    udp.send(filename)
        .await
        .unwrap_or_else(|e| panic!("❌ Failed to send filename: {}", e));

    println!(
        "📨 Sent filename: {:?}",
        std::str::from_utf8(filename).unwrap_or("noname")
    );

    // Send the file content
    tokio::io::copy(&mut file, &mut UdpWriter { udp: &udp })
        .await
        .unwrap_or_else(|e| panic!("❌ Failed to send file content: {}", e));
    println!("✅ File content sent successfully");

    let _ = udp.send(&[]).await;
}
