
use tokio::net::{TcpStream, TcpListener};
use tokio::fs::File;
use tokio::io::{copy, AsyncWriteExt};
use rfd::FileDialog;

async fn client(ip_port:String){
    let file_path = FileDialog::new()
    .set_directory("/")
    .pick_file().unwrap();


    let mut file = File::open(&file_path).await.expect("Permission Denied, Try Administor");
    let mut tcp = TcpStream::connect(&ip_port.replace("\r\n", "")).await.expect("Network Unavaliable");
    
    //? send file name and get permission
    // get file name
    let file_name = std::path::Path::new(&file_path);
    // send file name
    tcp.write_all(file_name.file_name().unwrap().to_str().unwrap().as_bytes()).await.unwrap();
    tcp.flush().await.unwrap();
    tcp.shutdown().await.unwrap();

    //? send file
    let mut tcp = TcpStream::connect(&ip_port.replace("\r\n", "")).await.expect("Network Unavaliable");
    copy(&mut file, &mut tcp).await.unwrap();
}

async fn server(ip_port:String){
    let mut w_mode = false;
    let mut name = String::new();
    let tcp = TcpListener::bind(ip_port.replace("\r\n", "")).await.unwrap();

    loop {
        println!("loop of stream");
        let ( mut stream, _socket) = tcp.accept().await.unwrap();
        
        if w_mode{
            println!("Write to file");
            // Write to file
            let mut file = File::create(&name).await.unwrap();
            copy(&mut stream, &mut file).await.unwrap();
            w_mode = false;
        } else {
            println!("Create file");
            // Create file
            let mut buf = [0u8;1024];
            let lengh = stream.peek(&mut buf).await.unwrap();
            let file_name_buf = &buf[..lengh];
            name = std::string::String::from_utf8(file_name_buf.to_vec()).unwrap();
            w_mode = true;
        }
    }
}

fn read(message:&str)->String{
    println!("{message}");
    let mut state = String::new();
    std::io::stdin().read_line(&mut state).unwrap();
    state
}

#[tokio::main]
async fn main(){
    match read("Choose mod: Server(1), Client(2)").replace("\r\n", "").as_str() {
        "1" => {server(read("Enter ip:port")).await},
        "2" => {client(read("Enter ip:port")).await},
        _ => {panic!("X")}
    }
}