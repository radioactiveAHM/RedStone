pub fn ipfinder() -> Vec<(String, std::net::IpAddr)> {
    local_ip_address::list_afinet_netifas().expect("Error getting ip")
}
