# Edge stack and packet path

## Question

> What is Unimog and how does it work?
> What is l4drop and how does it differ from Unimog?
> What is FL2 and why Rust?
> What is the HTTP/2 Rapid Reset attack?
> Is the TLS termination proxy separate from FL2?
> How does dosd detect and mitigate attacks?
> What is xdpd? What is Tubular?
> What is the flow from NIC → XDP → kernel → Tubular → FL2?

## Answer

### Components

| Name | Role |
|------|------|
| **Unimog** | XDP/TC L4 load balancer; 5-tuple hash; daisy-chain redirector |
| **l4drop** | XDP DDoS mitigation — drop by rules in maps (same XDP world as Unimog, different job) |
| **xdpd** | Userspace: load XDP, read config (Quicksilver), metrics |
| **dosd** | Userspace: detect attacks, push mitigation rules into eBPF maps |
| **Tubular** | eBPF `sk_lookup` — which local app socket gets the packet |
| **FL2** | Rust L7 proxy — TLS termination, HTTP/2|3, routing (successor mindset to older NGINX-era path) |

TLS termination for customer HTTP lives in the L7 proxy path (FL2). L4 (Unimog) does not need to decrypt.

### Full packet path

```text
NIC
 └─ XDP
     ├─ l4drop?  → XDP_DROP (attack)
     └─ Unimog?  → steer / pass
         └─ kernel network stack
             └─ TC redirector (socket lookup / daisy chain to another machine)
                 └─ Tubular sk_lookup → listening socket
                     └─ FL2
                         ├─ TLS terminate
                         ├─ HTTP/2 or HTTP/3
                         ├─ WAF / cache / rules
                         └─ origin or next hop
```

Uniform software on every edge box. Anycast lands the client on a nearby PoP. Inside the PoP: private + overlay for machine identity; public AnyIP for customers.

### HTTP/2 Rapid Reset

HTTP/2 streams can be opened and cancelled (`RST_STREAM`) very cheaply. Attackers forced large amounts of work on L7 proxies for little client cost. Mitigations: stream limits, isolation, faster/safer proxies. This is part of why FL2 / Rust at the edge mattered operationally.

### Where to go deeper

- [L4 vs L7 / daisy chain](../l4-vs-l7/)
- [eBPF / XDP / dosd maps](../ebpf-xdp/)
- [Anycast / AnyIP](../anycast-bgp/)
- [Control vs data plane](../control-data-plane/)
- [resources.md](../../resources.md) — Unimog, l4drop, Rapid Reset, Quicksilver posts
