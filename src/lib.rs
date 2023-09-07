pub fn ipfinder()->Vec<(String, std::net::IpAddr)>{
    local_ip_address::list_afinet_netifas().expect("Error getting ip")
}

pub fn read(message:&str)->String{
    println!("{message}");
    let mut state = String::new();
    std::io::stdin().read_line(&mut state).unwrap();
    state
}