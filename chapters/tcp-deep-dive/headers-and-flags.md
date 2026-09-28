# TCP headers and flags

## Question

> What are all the TCP header fields?
> What do SYN, ACK, PSH, RST, FIN, URG mean?
> What's the difference between RST and FIN?
> How do the 3-way handshake and 4-way termination work?
> What are TCP options (MSS, Window Scale, SACK)?

## Answer

TCP is a reliable bidirectional byte stream. The header is how two kernels keep that stream ordered, acknowledged, and open/closed correctly.

### Header fields

| Field | Role |
|-------|------|
| Source port / Dest port | With IPs, identify the connection (part of 5-tuple) |
| Sequence number | Byte offset of this segment's payload in the stream |
| Acknowledgment number | Next byte the sender of this segment expects to receive |
| Data offset | Header length in 32-bit words (options extend the header) |
| Flags | SYN, ACK, FIN, RST, PSH, URG, ECE, CWR, … |
| Window | Receive window advertised to the peer (flow control) |
| Checksum | Integrity over header + payload (+ pseudo-header) |
| Urgent pointer | Significant when URG is set (rarely used in modern apps) |
| Options | MSS, Window Scale, SACK permitted/blocks, Timestamps, … |

### Flags

| Flag | Meaning |
|------|---------|
| SYN | Synchronize sequence numbers — open |
| ACK | Acknowledgment field is valid |
| FIN | No more data from sender — graceful close of this direction |
| RST | Abort the connection immediately |
| PSH | Hint to push buffered data to the application sooner |
| URG | Urgent pointer field is significant |

**RST vs FIN:** FIN is orderly shutdown (both sides can still complete the handshake of closes). RST means “stop now” — wrong state, no listening socket, abort, etc. A packet that hits a host with no matching socket often gets RST.

### 3-way handshake

```
Client                         Server
  SYN seq=c  ─────────────────▶
  ◀─────────── SYN-ACK seq=s ack=c+1
  ACK ack=s+1 ────────────────▶
           both ESTABLISHED
```

Initial sequence numbers are chosen (randomized) so old duplicates are less dangerous.

### 4-way termination (typical)

```
  FIN ────────────────────────▶
  ◀──────────────────────── ACK
  ◀──────────────────────── FIN
  ACK ────────────────────────▶
```

One side often spends time in `TIME_WAIT` so late segments do not confuse a new connection reusing the same ports.

### Options you will see on SYN

- **MSS** — maximum segment size (payload) this side will accept; related to path MTU
- **Window Scale** — multiply the 16-bit window field by `2^shift` for large windows
- **SACK** — selective acknowledgment of non-contiguous blocks after loss
- **Timestamps** — RTT measurement / PAWS

### Demo

```bash
cd code && cargo run --bin tcp-header-parser
```

For the per-socket state machine (send, ACK, retransmit, FIN), see [What is a socket?](../sockets/what-is-a-socket.md) and [Lifecycle](./lifecycle-and-reliability.md).
