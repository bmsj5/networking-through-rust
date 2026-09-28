# Anycast & BGP

## Learning Objectives

- Explain Anycast (same prefix, many PoPs)
- Read CIDR (`/12`, `/22`, `/24`) and host counts
- Understand AnyIP + `IP_FREEBIND`
- Why millions of IPs despite Anycast

## Topics

1. [Anycast, CIDR, AnyIP](./anycast-cidr-anyip.md)

## Demo

```bash
cd code && cargo run --bin cidr-calculator
```

## Questions

- How can Cloudflare advertise millions of IPs?
- What does `ip addr` look like on an edge box?
- Bind without "owning" every address?
- BYOIP vs advertising someone else's space?

## Key Insight

> BGP announces **prefixes**, not single IPs. Anycast gets you to the nearest city. Many IPs solve Spectrum, legacy SNI-less TLS, egress ports, and blast radius.

## Next

[Control Plane vs Data Plane](../control-data-plane/)
