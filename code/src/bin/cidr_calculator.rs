//! CIDR host-count calculator.

fn hosts_in_prefix(prefix_len: u32) -> Option<u128> {
    if prefix_len > 32 {
        return None;
    }
    Some(1u128 << (32 - prefix_len))
}

fn main() {
    println!("CIDR calculator (IPv4)\n");
    for len in [32, 24, 22, 16, 12, 8] {
        let n = hosts_in_prefix(len).unwrap();
        println!("  /{len:<2}  →  {n:>10} addresses");
    }

    println!("\nExamples:");
    println!("  103.21.244.0/22  → {} IPs", hosts_in_prefix(22).unwrap());
    println!("  104.16.0.0/12    → {} IPs", hosts_in_prefix(12).unwrap());
    println!("\nBGP announces the prefix once; Anycast can originate it from many PoPs.");
}
