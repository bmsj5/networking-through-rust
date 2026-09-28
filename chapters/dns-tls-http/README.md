# DNS, TLS, and HTTP

Names → addresses, encrypting the stream, and what changes across HTTP versions.

## Chapters

1. [How DNS resolution works](./dns-and-geo.md)
2. [TLS termination and HTTP versions](./tls-and-http.md)

## Demo

```bash
cd code && cargo run --bin dns-resolver
```

Also see [TLS as a transparent wrapper](../osi-vs-tcpip/tls-as-presentation.md) for the coding angle.

## Questions that came up

- How does DNS resolution work?
- Recursive vs authoritative?
- How can answers be geo-aware?
- What does TLS termination mean?
- How does TLS relate to L7 load balancing?
- HTTP/2 multiplexing? QUIC / HTTP/3?

## Next

[L4 vs L7 load balancing](../l4-vs-l7/)
