use tokio::net::TcpListener;
use tokio::fs::File;
use tokio::io::copy;

pub async fn server(ip_port:String){
    // The w_mode determines whether the file is going to be created or written 
    let mut w_mode = false;
    // Name is going to hold file name as global
    let mut name = String::new();
    // TCP connection
    let tcp = TcpListener::bind(ip_port.replace("\r\n", "")).await.unwrap();

    // Wait for stream
    loop {
        println!("loop of stream");
        // Incoming stream
        let ( mut stream, _socket) = tcp.accept().await.unwrap();
        
        if w_mode{
            // This block creates and writes to the file
            println!("Write to file");
            // Write to file
            let mut file = File::create(&name).await.unwrap();
            copy(&mut stream, &mut file).await.unwrap();
            w_mode = false;
        } else {
            // This block gets the file name
            println!("receive file name");
            let mut buf = [0u8;1024];
            let lengh = stream.peek(&mut buf).await.unwrap();
            let file_name_buf = &buf[..lengh];
            name = std::string::String::from_utf8(file_name_buf.to_vec()).unwrap();
            w_mode = true;
        }
    }
}