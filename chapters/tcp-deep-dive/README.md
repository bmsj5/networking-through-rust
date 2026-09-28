# TCP deep dive

Headers, flags, handshake, teardown, and why retransmission / congestion control exist.

## Chapters

1. [Headers and flags](./headers-and-flags.md)
2. [Lifecycle: data, retransmit, congestion](./lifecycle-and-reliability.md)

## Demos

```bash
cd code && cargo run --bin tcp-header-parser
cd code && cargo run --bin tcp-state-machine
```

## Questions that came up

- What are all the TCP header fields?
- What do SYN, ACK, PSH, RST, FIN, URG mean?
- What's the difference between RST and FIN?
- How does the 3-way handshake work?
- How does the 4-way close work?
- What are TCP options (MSS, Window Scale, SACK)?
- How does retransmission work?
- What about congestion control (slow start, AIMD)?

## Next

[IP deep dive](../ip-deep-dive/)
