use tokio::net::TcpStream;
use tokio::fs::File;
use tokio::io::{copy, AsyncWriteExt};
use rfd::FileDialog;

pub async fn client(ip_port:String){
    // Open FileDialog to pick the file
    let file_path = FileDialog::new()
    .set_directory("/")
    .pick_file().unwrap();

    // Open file as read only mode
    let mut file = File::open(&file_path).await.expect("Permission Denied, Try Administor");
    // Create TCP connection
    let mut tcp = TcpStream::connect(&ip_port.replace("\r\n", "")).await.expect("Network Unavaliable");
    
    // Get file name
    let file_name = std::path::Path::new(&file_path);
    // Send file name to create empty file
    tcp.write_all(file_name.file_name().unwrap().to_str().unwrap().as_bytes()).await.unwrap();
    tcp.flush().await.unwrap();
    tcp.shutdown().await.unwrap();

    // Write to file
    let mut tcp = TcpStream::connect(&ip_port.replace("\r\n", "")).await.expect("Network Unavaliable");
    copy(&mut file, &mut tcp).await.unwrap();
}