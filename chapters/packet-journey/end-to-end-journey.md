# End-to-end packet path

## Question

> How does a packet travel from my laptop to another country?
> Does the destination or source IP change at each hop?
> Does every router unwrap Ethernet to see the IP destination?
> What happens to Sequence and Acknowledgment numbers?
> How do I see this with traceroute / dig?

## Answer

### IPs vs MACs

**MAC addresses change at every hop. IP addresses do not** (unless NAT is involved — a separate topic).

A router:

1. Receives an Ethernet frame
2. Checks destination MAC (is it for me?)
3. Extracts the IP packet
4. Decrements TTL; if 0 → ICMP Time Exceeded and drop
5. Looks up next hop in the routing table
6. Resolves next-hop MAC (ARP / NDP)
7. Builds a **new** Ethernet frame (new MACs, **same** IP packet)
8. Sends out the correct interface

TCP seq/ack are end-to-end. Routers do not rewrite them.

### Complete path

```
┌─────────────────────────────────────────────────────────────────────────────────────┐
│ SENDER                                                                              │
│                                                                                     │
│  Application: "GET / HTTP/1.1"                                                      │
│       │                                                                             │
│       ▼                                                                             │
│  Presentation: encode() + optional compression + optional encryption (TLS)          │
│       │                                                                             │
│       ▼                                                                             │
│  Session: manage connection lifecycle (reconnect loop, etc.)                        │
│       │                                                                             │
│       ▼                                                                             │
│  Transport (TCP):                                                                   │
│    - Add TCP header (src port, dst port, seq, ack, flags, window, checksum)         │
│    - Segment data into MSS-sized chunks                                             │
│    - Maintain state machine (SYN_SENT → ESTABLISHED → …)                            │
│       │                                                                             │
│       ▼                                                                             │
│  Network (IP):                                                                      │
│    - Add IP header (version, TTL, protocol, src IP, dst IP, checksum)               │
│    - Fragment if needed (rare with PMTUD)                                           │
│       │                                                                             │
│       ▼                                                                             │
│  Data Link (Ethernet):                                                              │
│    - Add Ethernet header (src MAC, dst MAC, EtherType)                              │
│       │                                                                             │
│       ▼                                                                             │
│  Physical: convert to bits / light, send on the medium                              │
└─────────────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────────────────────────┐
│ ROUTER (one hop)                                                                    │
│                                                                                     │
│  1. Receive Ethernet frame                                                          │
│  2. Check destination MAC                                                           │
│  3. Extract IP packet                                                               │
│  4. Decrement TTL; if 0, ICMP Time Exceeded and drop                                │
│  5. Routing table → next hop IP                                                     │
│  6. ARP/NDP → next hop MAC                                                          │
│  7. New Ethernet frame (new MACs, same IP packet)                                   │
│  8. Send out correct interface                                                      │
└─────────────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────────────────────────┐
│ RECEIVER                                                                            │
│                                                                                     │
│  1. Physical: receive bits                                                          │
│  2. Data Link: check dest MAC; extract IP packet                                    │
│  3. Network: check dest IP; extract TCP segment                                     │
│  4. Transport (TCP):                                                                │
│     - Check checksum                                                                │
│     - Check sequence numbers; reorder if needed                                     │
│     - Send ACK                                                                      │
│     - Reassemble into byte stream                                                   │
│  5. Session: connection state in the app                                            │
│  6. Presentation: decrypt + decompress + decode                                     │
│  7. Application: "GET / HTTP/1.1"                                                   │
└─────────────────────────────────────────────────────────────────────────────────────┘
```

### Router forwarding (pseudo)

```text
fn forward(frame):
    ip = unwrap_ethernet(frame)
    if ip.ttl <= 1:
        send_icmp_time_exceeded(ip)
        return DROP
    ip.ttl -= 1
    ip.checksum = recalc(ip)
    next_hop = route_lookup(ip.dst)
    mac = arp_resolve(next_hop)
    return ethernet(src=my_mac, dst=mac, payload=ip)
```

### traceroute

Sends probes with TTL = 1, 2, 3, … Each hop that expires a probe replies with ICMP Time Exceeded. `traceroute -A` also shows ASN — you can watch borders on the path.

Hands-on: [Linux tools](./hands-on-linux.md).
