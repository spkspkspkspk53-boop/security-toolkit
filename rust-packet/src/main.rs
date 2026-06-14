use std::net::Ipv4Addr;

fn create_ipv4_packet(src: &str, dst: &str) -> Vec<u8> {
    let mut packet = vec![0u8; 20];
    packet[0] = 0x45;
    packet[1] = 0x00;
    packet[2] = 0x00;
    packet[3] = 0x14;
    packet[4] = 0x00;
    packet[5] = 0x00;
    packet[6] = 0x40;
    packet[7] = 0x00;
    packet[8] = 0x40;
    packet[9] = 0x06;

    let src_bytes = src.parse::<Ipv4Addr>().unwrap().octets();
    let dst_bytes = dst.parse::<Ipv4Addr>().unwrap().octets();

    packet[12..16].copy_from_slice(&src_bytes);
    packet[16..20].copy_from_slice(&dst_bytes);

    packet
}

fn create_tcp_syn(sport: u16, dport: u16) -> Vec<u8> {
    let mut packet = vec![0u8; 20];
    packet[0..2].copy_from_slice(&sport.to_be_bytes());
    packet[2..4].copy_from_slice(&dport.to_be_bytes());
    packet[4..8].copy_from_slice(&[0, 0, 0, 1]);
    packet[8..12].copy_from_slice(&[0, 0, 0, 0]);
    packet[12] = 0x50;
    packet[13] = 0x02;
    packet[14..16].copy_from_slice(&[0x20, 0x00]);

    packet
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 5 {
        eprintln!("Usage: {} <src_ip> <dst_ip> <sport> <dport> [syn|ack]", args[0]);
        return;
    }

    let src_ip = &args[1];
    let dst_ip = &args[2];
    let sport: u16 = args[3].parse().unwrap_or(12345);
    let dport: u16 = args[4].parse().unwrap_or(80);
    let pkt_type = args.get(5).map(|s| s.as_str()).unwrap_or("syn");

    println!("📦 Packet Crafter");
    println!("Source:      {}:{}", src_ip, sport);
    println!("Destination: {}:{}", dst_ip, dport);
    println!("Type:        {}\n", pkt_type);

    let ip_pkt = create_ipv4_packet(src_ip, dst_ip);
    let tcp_pkt = create_tcp_syn(sport, dport);

    let mut full_pkt = ip_pkt;
    full_pkt.extend(tcp_pkt);

    println!("✅ Packet created: {} bytes", full_pkt.len());
    println!("Hex: {}", hex_encode(&full_pkt));
}

fn hex_encode(data: &[u8]) -> String {
    data.iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<_>>()
        .join("")
}