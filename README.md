# Networking Through Rust 🦀

**Understanding how packets, kernels, and global networks actually work**

This repository is a hands-on curriculum built from real questions — from "what are the 7 OSI layers?" to Unimog, eBPF/XDP, and Anycast. It was inspired by Cloudflare's engineering blog and edge architecture. It mirrors [computer systems through Rust](https://github.com/bmsj5/how-computer-systems-rust-work): concepts first, then runnable code.

## What You'll Learn

- **OSI vs TCP/IP** — map vs territory; where your code actually sits
- **Packet journey** — MAC vs IP, TTL, traceroute, BGP at a glance
- **Sockets** — kernel FDs, buffers, blocking vs async
- **TCP & IP** — headers, handshake, congestion, fragmentation, ICMP
- **L4 vs L7 load balancing** — 5-tuple hash, daisy-chaining, Tubular
- **eBPF/XDP** — maps, ring buffers, control plane vs data plane
- **Overlays** — TUN/TAP, WireGuard, Nebula, inner vs outer packets
- **Anycast & BGP** — CIDR, AnyIP, IP_FREEBIND, why millions of IPs
- **DNS, TLS, HTTP** — resolution, termination, HTTP/2 vs HTTP/3
- **Control vs data plane** — who decides, who executes at the edge
- **Cloudflare case study** — NIC → XDP → Tubular → FL2 path

## Learning Path

### Phase 1: Foundations
1. [OSI vs TCP/IP](./chapters/osi-vs-tcpip/)
2. [How a Packet Travels](./chapters/packet-journey/)
3. [Sockets](./chapters/sockets/)

### Phase 2: Protocols Deep Dive
4. [TCP Deep Dive](./chapters/tcp-deep-dive/)
5. [IP Deep Dive](./chapters/ip-deep-dive/)
6. [DNS, TLS & HTTP](./chapters/dns-tls-http/)

### Phase 3: Production Networking
7. [L4 vs L7 Load Balancing](./chapters/l4-vs-l7/)
8. [eBPF & XDP](./chapters/ebpf-xdp/)
9. [Overlay Networks](./chapters/overlay-networks/)
10. [Anycast & BGP](./chapters/anycast-bgp/)

### Phase 4: Edge Architecture
11. [Control Plane vs Data Plane](./chapters/control-data-plane/)
12. [Cloudflare Case Study](./chapters/cloudflare-case-study/)

## Getting Started

### Prerequisites
- Rust 1.70+
- Linux recommended for `traceroute`, `ss`, `tcpdump` labs
- Curiosity about how the internet actually moves packets

### Quick Start
```bash
git clone https://github.com/bmsj5/networking-through-rust.git
cd networking-through-rust

cd code
cargo run --bin tcp-echo-server
cargo run --bin tcp-header-parser
cargo run --bin cidr-calculator
```

Or from the repo root:
```bash
make run-all
```

## Repository Structure

```
networking-through-rust/
├── chapters/                    # Educational content by topic
│   ├── osi-vs-tcpip/
│   ├── packet-journey/
│   ├── sockets/
│   ├── tcp-deep-dive/
│   ├── ip-deep-dive/
│   ├── l4-vs-l7/
│   ├── ebpf-xdp/
│   ├── overlay-networks/
│   ├── anycast-bgp/
│   ├── dns-tls-http/
│   ├── control-data-plane/
│   └── cloudflare-case-study/
├── code/                        # Runnable Rust demos
│   └── src/bin/
├── resources.md                 # RFCs, Cloudflare blogs, further reading
├── Makefile
└── README.md
```

## Interactive Demos

| Demo | Concept |
|------|---------|
| `tcp-echo-server` | Sockets, accept loop, byte streams |
| `tcp-header-parser` | TCP header fields & flags |
| `tcp-state-machine` | Handshake / teardown simulation |
| `ip-header-parser` | IPv4 header fields, TTL, fragmentation bits |
| `cidr-calculator` | Prefix length → host count |
| `dns-resolver` | Minimal recursive-style lookup walkthrough |

## How Each Chapter Works

Every chapter includes:
- The **questions** that drove the learning
- The **insight** that clicked
- Concepts, diagrams, and pseudo-code
- Links to demos and authoritative sources

## Not Covered (Yet)

Good candidates for future modules:
- HTTP/2 and HTTP/3 framing details
- QUIC transport mechanics beyond basics
- BGP path selection and route filtering
- TLS 1.3 handshake internals
- Kubernetes networking (CNI, kube-proxy, service mesh)

See [resources.md](./resources.md) for starting points.

## Development

```bash
make run-all          # all demos in learning order
make foundations      # echo server, headers, CIDR
make protocols        # TCP state machine, IP header, DNS
make clean
```

**Happy learning.** The goal is to turn "networking feels like magic" into a mental model you can map onto real kernels, real packets, and real production systems.
