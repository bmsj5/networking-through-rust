//! Parse a crafted IPv4 header.

fn main() {
    // Minimal IPv4 header: 20 bytes
    // ver=4 ihl=5, total_len=40, id=0x1234, DF, TTL=64, proto=TCP(6)
    // src=192.0.2.1 dst=198.51.100.2
    let hdr: [u8; 20] = [
        0x45, 0x00, // ver/ihl, DSCP
        0x00, 0x28, // total length 40
        0x12, 0x34, // identification
        0x40, 0x00, // flags=DF, frag offset=0
        0x40, // TTL 64
        0x06, // protocol TCP
        0x00, 0x00, // checksum placeholder
        192, 0, 2, 1, // src
        198, 51, 100, 2, // dst
    ];

    let version = hdr[0] >> 4;
    let ihl = (hdr[0] & 0x0f) as usize * 4;
    let total_len = u16::from_be_bytes([hdr[2], hdr[3]]);
    let id = u16::from_be_bytes([hdr[4], hdr[5]]);
    let flags_frag = u16::from_be_bytes([hdr[6], hdr[7]]);
    let df = (flags_frag & 0x4000) != 0;
    let mf = (flags_frag & 0x2000) != 0;
    let frag_off = flags_frag & 0x1fff;
    let ttl = hdr[8];
    let proto = hdr[9];
    let src = format!("{}.{}.{}.{}", hdr[12], hdr[13], hdr[14], hdr[15]);
    let dst = format!("{}.{}.{}.{}", hdr[16], hdr[17], hdr[18], hdr[19]);

    println!("IPv4 header parse");
    println!("  version      = {version}");
    println!("  ihl          = {ihl} bytes");
    println!("  total_length = {total_len}");
    println!("  id           = {id:#x}");
    println!("  DF           = {df}");
    println!("  MF           = {mf}");
    println!("  frag_offset  = {frag_off}");
    println!("  TTL          = {ttl}");
    println!("  protocol     = {} ({})", proto, proto_name(proto));
    println!("  src          = {src}");
    println!("  dst          = {dst}");
}

fn proto_name(p: u8) -> &'static str {
    match p {
        1 => "ICMP",
        6 => "TCP",
        17 => "UDP",
        _ => "other",
    }
}
