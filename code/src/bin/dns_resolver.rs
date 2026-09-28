//! Simulated DNS resolution walk (educational, no network).

struct Zone {
    name: &'static str,
    records: Vec<(&'static str, &'static str)>,
}

fn lookup(zones: &[Zone], qname: &str) -> Option<&'static str> {
    // Longest-suffix authoritative match, then exact/relative A.
    let mut best: Option<&Zone> = None;
    for z in zones {
        if qname == z.name || qname.ends_with(&format!(".{}", z.name)) {
            if best.map(|b| z.name.len() > b.name.len()).unwrap_or(true) {
                best = Some(z);
            }
        }
    }
    let zone = best?;
    println!("  ask authoritative for zone {}", zone.name);
    for (name, ip) in &zone.records {
        if *name == qname || format!("{name}.{}", zone.name) == qname {
            return Some(ip);
        }
    }
    // NS glue style: if asking parent, return referral note
    None
}

fn main() {
    println!("DNS resolver walkthrough (simulated)\n");
    println!("Query: www.example.com A\n");

    let root = Zone {
        name: ".",
        records: vec![],
    };
    let _ = root;

    println!("1. Stub → recursive resolver");
    println!("2. Recursive → root: where is .com?");
    println!("   ← NS a.gtld-servers.net (referral)");
    println!("3. Recursive → TLD: where is example.com?");
    println!("   ← NS ns1.example.com (referral)");

    let zones = vec![
        Zone {
            name: "example.com",
            records: vec![
                ("www.example.com", "93.184.216.34"),
                ("example.com", "93.184.216.34"),
            ],
        },
    ];

    println!("4. Recursive → authoritative example.com");
    match lookup(&zones, "www.example.com") {
        Some(ip) => println!("   ← A {ip}"),
        None => println!("   ← NXDOMAIN"),
    }

    println!("\nGeo note: auth/CDN may return different A/AAAA by resolver location.");
    println!("Try locally: dig +trace cloudflare.com");
}
