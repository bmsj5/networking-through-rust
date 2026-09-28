# Resources

Authoritative sources referenced across chapters.

## RFCs
- [RFC 791 — Internet Protocol](https://datatracker.ietf.org/doc/html/rfc791)
- [RFC 793 — Transmission Control Protocol](https://datatracker.ietf.org/doc/html/rfc793)
- [RFC 792 — ICMP](https://datatracker.ietf.org/doc/html/rfc792)
- [RFC 1034 / 1035 — DNS](https://datatracker.ietf.org/doc/html/rfc1034)
- [RFC 8446 — TLS 1.3](https://datatracker.ietf.org/doc/html/rfc8446)
- [RFC 9000 — QUIC](https://datatracker.ietf.org/doc/html/rfc9000)
- [RFC 4271 — BGP-4](https://datatracker.ietf.org/doc/html/rfc4271)

## Cloudflare Engineering
- [Unimog — Cloudflare's edge load balancer](https://blog.cloudflare.com/unimog-cloudflares-edge-load-balancer/)
- [l4drop / DDoS at the edge](https://blog.cloudflare.com/l4drop-xdp-ebpf-based-ddos-mitigations/)
- [How Cloudflare’s architecture works](https://blog.cloudflare.com/cloudflare-architecture-and-how-bpf-eats-the-world/)
- [HTTP/2 Rapid Reset](https://blog.cloudflare.com/technical-breakdown-http2-rapid-reset-ddos-attack/)
- [Quicksilver](https://blog.cloudflare.com/introducing-quicksilver/)
- [Every request, every machine](https://blog.cloudflare.com/building-fast-interpreters-in-rust/)

## Kernel / eBPF
- [XDP Tutorial](https://github.com/xdp-project/xdp-tutorial)
- [BPF and XDP Reference Guide (Cilium)](https://docs.cilium.io/en/latest/bpf/)
- [Linux Socket Filtering / eBPF](https://www.kernel.org/doc/html/latest/bpf/index.html)

## Books & Essays
- *TCP/IP Illustrated, Vol. 1* — Stevens
- *Computer Networks* — Tanenbaum / Wetherall
## Hands-on Linux
```bash
traceroute -A 1.1.1.1
dig +trace cloudflare.com
ss -tunap
tcpdump -i any -nn port 443
ip route
ip addr
```
