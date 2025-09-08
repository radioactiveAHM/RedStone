use tokio::io::AsyncRead;

struct UdpReader<'a> {
    udp: &'a tokio::net::UdpSocket,
}
impl AsyncRead for UdpReader<'_> {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        self.udp.poll_recv(cx, buf)
    }
}

async fn recv_filename(udp: &tokio::net::UdpSocket) -> Result<String, Box<dyn std::error::Error>> {
    let mut buf = [0; 512];
    let len = udp.recv(&mut buf).await?;
    Ok(String::from_utf8(buf[..len].to_vec())?)
}

pub async fn server(ip_port: String) {
    let udp = tokio::net::UdpSocket::bind(ip_port.trim())
        .await
        .unwrap_or_else(|e| panic!("Failed to bind UDP socket on {}: {}", ip_port.trim(), e));

    loop {
        let filename = recv_filename(&udp)
            .await
            .expect("❌ Failed to receive valid filename");

        println!("📥 Incoming file transfer request: {}", filename);

        let mut file = tokio::fs::File::options()
            .create(true)
            .append(true)
            .open(&filename)
            .await
            .unwrap_or_else(|e| panic!("❌ Failed to open/create file '{}': {}", filename, e));

        println!("📝 File '{}' opened for writing", filename);

        tokio::io::copy(&mut UdpReader { udp: &udp }, &mut file)
            .await
            .unwrap_or_else(|e| panic!("❌ Failed to write content to '{}': {}", filename, e));

        println!("✅ Successfully wrote content to '{}'", filename);
    }
}
