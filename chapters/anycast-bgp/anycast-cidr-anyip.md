# Anycast, CIDR, and AnyIP

## Question

> How does Anycast work?
> How can Cloudflare advertise millions of IPs?
> What is CIDR (`/12`, `/22`, `/24`)?
> How do BGP announcements cover millions of IPs?
> Can I bind to a public IP I don't “own” on the box?
> What is AnyIP? What is `IP_FREEBIND`?
> What does `ip addr` look like on an edge server?
> Why millions of IPs if Anycast already handles load?
> Do they advertise my origin IP?

## Answer

### Anycast

The same prefix is announced from many locations. Routers pick a path (usually toward a nearby PoP). Example: `1.1.1.1` answered from many cities. Anycast gets traffic **into a city**. It does not by itself solve every “which customer / which service on this box” problem — that is why many distinct IPs still exist.

### CIDR and BGP

| Prefix | Addresses (IPv4) |
|--------|------------------|
| /32 | 1 |
| /24 | 256 |
| /22 | 1 024 |
| /12 | 1 048 576 |

BGP announces **prefixes**, not one route per host. One announcement for `104.16.0.0/12` covers about a million IPs. The global table has on the order of ~950k prefixes; a large CDN might contribute on the order of ~100 aggregates — not millions of BGP entries.

> Do they own `103.21.244.0/22`?

Yes in the sense that they announce space they are authorized to announce. `/22` = 4×256 = 1024 addresses.

### AnyIP — accepting a whole prefix

> Would `ip addr` show something like `inet 104.16.0.0/12`?

Large prefixes can appear as local ranges. The usual trick:

```bash
ip route add local <prefix> dev lo
```

Kernel treats destinations in that prefix as local. Applications bind specific addresses inside it, or **Tubular** (`sk_lookup`) dispatches by destination IP without binding every address.

> But how do they bind on all of these IPs? I could only bind on a specific IP or `0.0.0.0`.

AnyIP + per-IP bind or sk_lookup. For IPv6, **`IP_FREEBIND`** lets you bind addresses that are not “assigned” in the classic sense.

### Why so many IPs despite Anycast?

1. **Spectrum / non-HTTP** — no Host header or SNI to multiplex many customers on one IP
2. Legacy TLS clients without SNI
3. Enterprise dedicated IPs
4. **Egress:** ~64k source ports per src IP toward one origin → need many egress IPs
5. **DDoS blast radius** — spread attacks across address space

Anycast: nearest city. Many IPs: identify customers / services and scale outbound inside the city.

### BYOIP

> How do they have permission to advertise my IP if it's not their range?

By default they **do not** advertise your origin IP. They advertise theirs and proxy to you. To announce *your* space (BYOIP), you need authorization (e.g. Letter of Agency) proving you control that range.

### Demo

```bash
cd code && cargo run --bin cidr-calculator
```
