# Lifecycle: reliability and congestion

## Question

> How does retransmission work in pseudo-code?
> What about congestion control (slow start, AIMD)?
> What about URG / PSH in real code?
> Sliding window / flow control?

## Answer

After the handshake, each socket runs the state machine sketched in [What is a socket?](../sockets/what-is-a-socket.md). This chapter focuses on the data path: windows, retransmit, congestion.

### Sliding window and flow control

The peer advertises `rcv_wnd`: how many bytes it is willing to receive. You must not put more than that in flight toward them (relative to what they have ACKed).

Independently, **congestion control** maintains `cwnd`: how much you think the *path* can take. Bytes in flight are limited by roughly:

```text
in_flight <= min(cwnd, peer_rwnd)
```

and each segment is also sized by MSS.

### Sending (sketch)

```text
app write(data)
  → append to socket send_buffer
  → while send_buffer has data and in_flight < min(cwnd, rwnd):
        take up to MSS bytes
        send TCP segment (seq, ack, ACK|PSH, payload)
        remember segment in unacked list
        arm / reset retransmit timer
```

### On ACK

```text
receive_ack(ack_num):
  drop unacked segments fully covered by ack_num
  snd_una = ack_num
  grow cwnd:
    if slow_start: cwnd increases aggressively (classically ~double per RTT)
    else: cwnd += ~1 MSS per RTT   # congestion avoidance (AIMD increase)
```

### Retransmission

```text
on_retransmit_timeout(oldest_unacked):
  retransmit that segment
  ssthresh = cwnd / 2          # classic
  cwnd = 1 MSS                 # back to slow start (classic Reno-ish)
  RTO *= 2                     # exponential backoff
  if retries exceeded: abort connection
```

Fast retransmit (common optimization): several duplicate ACKs for the same seq → retransmit the missing segment without waiting for the full RTO.

### Congestion control story (classic)

1. **Slow start** — probe capacity quickly until `ssthresh`
2. **Congestion avoidance** — additive increase
3. **Loss** — multiplicative decrease of the window, then recover

Modern stacks use CUBIC, BBR, etc. Same job, different formulas: share the path and avoid collapse.

### PSH and URG in practice

- **PSH** — often set on the last segment of an application write; stacks and apps mostly treat it as a hint. You rarely set it yourself from userspace APIs.
- **URG** — legacy. Almost no modern protocol depends on the urgent pointer. Do not design new protocols around it.

### Demo

```bash
cd code && cargo run --bin tcp-state-machine
```

Full pseudo-code for open / data / retransmit / FIN: [what-is-a-socket.md](../sockets/what-is-a-socket.md).
