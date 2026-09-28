# TLS termination and HTTP versions

## Question

> How does TLS termination work?
> How does TLS relate to L7 load balancing?
> How does Traefik (or similar) do TLS termination?
> What is HTTP/2 multiplexing?
> What is QUIC / HTTP/3?

## Answer

### TLS termination

The edge or proxy finishes the TLS handshake with the **client**, holds the certificate and private key, and obtains plaintext application data (HTTP). It may open a separate connection to the origin (plain or TLS again).

L7 load balancing needs Host, path, cookies, headers — so something usually terminates TLS first. That is the link between TLS termination and L7.

How TLS looks **in application code** (wrap the socket, don't call `encrypt()` yourself):  
[TLS as a transparent wrapper](../osi-vs-tcpip/tls-as-presentation.md).

Proxies like Traefik/NGINX/FL2: configure certs, listen on 443, terminate TLS, then route by HTTP rules to backends.

### HTTP versions

| Version | Transport | What changed |
|---------|-----------|----------------|
| HTTP/1.1 | TCP | text, one request at a time per connection (simple model) |
| HTTP/2 | TCP | binary frames; **many streams** multiplexed on one connection; HPACK |
| HTTP/3 | QUIC over UDP | multiplexing without TCP head-of-line blocking; 0-RTT possible |

**HTTP/2 multiplexing:** many request/response streams share one TCP connection. Efficient until TCP loss blocks the whole connection — one reason HTTP/3/QUIC exists.

**HTTP/2 Rapid Reset:** open and cancel streams cheaply → asymmetric load on L7 proxies. Edge proxies needed stricter limits; part of the operational story around FL2 / Rust proxies. See [Cloudflare case study](../cloudflare-case-study/).
