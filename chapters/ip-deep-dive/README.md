# IP deep dive

TTL, fragmentation, and why ICMP shows up in traceroute.

## Chapters

1. [Headers, TTL, fragmentation](./headers-ttl-fragmentation.md)

## Demo

```bash
cd code && cargo run --bin ip-header-parser
```

## Questions that came up

- What are all the IPv4 header fields?
- What is TTL and why does it matter?
- What are DF / MF and fragment offset?
- Why do we need IP fragmentation if TCP already segments?
- What is Path MTU Discovery?
- What is ICMP?

## Next

[DNS, TLS & HTTP](../dns-tls-http/)
