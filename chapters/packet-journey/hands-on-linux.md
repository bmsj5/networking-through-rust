# Linux tools

```bash
# Path + ASN of each hop
traceroute -A 1.1.1.1

# DNS resolution from the root downward
dig +trace cloudflare.com

# DNS queries on the wire
sudo tcpdump -i any -nn port 53

# Active sockets and owning processes
ss -tunap | grep ESTAB

# Addresses and routes
ip addr
ip route
```

| Tool | What it shows |
|------|----------------|
| `traceroute -A` | hops and which network (ASN) owns them |
| `dig +trace` | recursive resolution chain |
| `tcpdump` | real headers on the wire |
| `ss` | kernel socket table (state, peers, process) |
