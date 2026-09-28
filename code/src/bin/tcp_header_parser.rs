//! Parse a crafted TCP header from bytes.

fn main() {
    // Synthetic TCP header (20 bytes, no options)
    // src=12345 dst=80 seq=1000 ack=2000 flags=PSH+ACK window=65535
    let hdr: [u8; 20] = [
        0x30, 0x39, // src port 12345
        0x00, 0x50, // dst port 80
        0x00, 0x00, 0x03, 0xe8, // seq 1000
        0x00, 0x00, 0x07, 0xd0, // ack 2000
        0x50, 0x18, // data offset=5 (20 bytes), flags=PSH|ACK
        0xff, 0xff, // window
        0x00, 0x00, // checksum (not validated)
        0x00, 0x00, // urgent pointer
    ];

    let src = u16::from_be_bytes([hdr[0], hdr[1]]);
    let dst = u16::from_be_bytes([hdr[2], hdr[3]]);
    let seq = u32::from_be_bytes([hdr[4], hdr[5], hdr[6], hdr[7]]);
    let ack = u32::from_be_bytes([hdr[8], hdr[9], hdr[10], hdr[11]]);
    let data_offset = (hdr[12] >> 4) as usize * 4;
    let flags = hdr[13];
    let window = u16::from_be_bytes([hdr[14], hdr[15]]);

    println!("TCP header parse");
    println!("  src_port     = {src}");
    println!("  dst_port     = {dst}");
    println!("  seq          = {seq}");
    println!("  ack          = {ack}");
    println!("  data_offset  = {data_offset} bytes");
    println!("  window       = {window}");
    println!("  flags        = {}", format_flags(flags));
}

fn format_flags(f: u8) -> String {
    let mut parts = Vec::new();
    if f & 0x20 != 0 {
        parts.push("URG");
    }
    if f & 0x10 != 0 {
        parts.push("ACK");
    }
    if f & 0x08 != 0 {
        parts.push("PSH");
    }
    if f & 0x04 != 0 {
        parts.push("RST");
    }
    if f & 0x02 != 0 {
        parts.push("SYN");
    }
    if f & 0x01 != 0 {
        parts.push("FIN");
    }
    if parts.is_empty() {
        "none".into()
    } else {
        parts.join("|")
    }
}
