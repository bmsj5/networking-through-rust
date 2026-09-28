# L4 vs L7 Load Balancing

## Learning Objectives

- Contrast L4 (5-tuple) vs L7 (HTTP/TLS-aware) balancers
- Explain Unimog-style forwarding tables and daisy-chaining
- Separate Unimog redirector from Tubular (sk_lookup)

## Topics

1. [L4 vs L7 & 5-Tuple](./l4-l7-five-tuple.md)
2. [Daisy Chaining & Tubular](./daisy-chaining-tubular.md)

## Questions

- What is HTTP & TLS termination?
- How does a 5-tuple hash work? Why Unimog uses it?
- Forwarding table with 100× buckets?
- Daisy chaining / redirector / socket lookup?
- Tubular vs Unimog?

## Key Insight

> Unimog picks **which server** in the PoP. Tubular picks **which socket** on that server. Different hooks, different jobs.

## Next

[eBPF & XDP](../ebpf-xdp/)
