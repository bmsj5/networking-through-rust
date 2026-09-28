# DNS resolution and geo answers

## Question

> How does DNS resolution work?
> What's the difference between recursive and authoritative DNS?
> How does Cloudflare return geo-aware A records?

## Answer

### Recursive vs authoritative

**Authoritative** — owns the zone data. Can answer for names it is responsible for, or refer you to another authoritative server.

**Recursive** — does the work for the stub (your OS/app): ask root → TLD → authoritative, cache answers, return an address to you.

```text
Stub (your machine)
  → Recursive resolver: "A for www.example.com?"
      → Root: where is .com?
      → TLD: where is example.com?
      → Auth for example.com: here is the A/AAAA
  ← address
```

`dig +trace cloudflare.com` shows that walk.

### Geo-aware A/AAAA

A CDN can return **different** addresses depending on where the query appears to come from (often the recursive resolver's location; sometimes EDNS Client Subnet carries a prefix of the client). Goal: steer the client toward a nearby PoP before the first TCP SYN, so Anycast / local capacity works better.

DNS is part of traffic steering, not only “name → one global IP.”

### Demo

```bash
cd code && cargo run --bin dns-resolver   # educational walkthrough
dig +trace cloudflare.com                 # real chain
```
