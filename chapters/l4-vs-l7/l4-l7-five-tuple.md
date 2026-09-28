# L4 vs L7 and the 5-tuple

## Question

> What does "HTTP & TLS Termination" mean?
> What's the difference between L4 and L7 load balancers?
> How does a 5-tuple hash work?
> Why does Unimog hash the 5-tuple?
> What is a forwarding table with 100× buckets?

## Answer

### L4 vs L7

| | L4 | L7 |
|-|----|----|
| Sees | IP + ports (+ TCP flags like SYN) | HTTP Host, path, headers, … |
| Needs TLS decrypt? | No | Usually yes (termination) |
| Cost | Cheap; can run at XDP/TC | More CPU; after crypto |
| Stickiness | 5-tuple hash | cookie / header / session |

**TLS termination:** the edge (or proxy) completes TLS with the client, holds certs/keys, and sees plaintext HTTP (or re-encrypts to origin). L7 routing on Host/path needs that plaintext. That is why “TLS termination” and “L7 load balancing” show up together.

### 5-tuple hash

```text
(src_ip, src_port, dst_ip, dst_port, protocol)
  → hash
  → bucket index
  → backend server
```

Same connection → same hash → same backend, as long as the table is stable. Stickiness without storing every flow in a huge connection table.

Unimog-style L4 balancers use this so all packets of a TCP connection land on the same machine (until the table changes — then daisy chaining saves old flows; see next page).

### Forwarding table with many buckets

Idea: far more buckets than servers (e.g. ~100×). Each bucket points at a server. Moving one bucket moves only a small slice of new connections — finer rebalance than “hash % N servers.”

Each bucket can have **two slots**:

- **Slot 1** — primary / where **new** connections go
- **Slot 2** — previous owner, still holding **old** connections

When you drain Server A onto Server B, update slot 1 → B, leave slot 2 → A. New SYNs go to B; non-SYN packets for A's old connections can still reach A via daisy chaining.

Continue: [Daisy chaining and Tubular](./daisy-chaining-tubular.md).
