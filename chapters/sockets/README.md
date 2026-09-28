# Sockets

## Chapters

1. [What is a socket?](./what-is-a-socket.md) — FD, kernel buffers, 5-tuple demux, full TCP state-machine pseudo-code
2. [Blocking, non-blocking, async](./blocking-vs-async.md) — asyncio/Tokio vs the same kernel path

## Demo

```bash
cd code && cargo run --bin tcp-echo-server
# nc 127.0.0.1 7878

cd code && cargo run --bin tcp-state-machine
```

## Questions

- What is a socket, really?
- Is it just a file descriptor with state?
- How does the kernel know which socket gets a packet?
- What happens on `writer.write()`?
- What is the kernel socket buffer?
- How does asyncio work with sockets?
- Blocking vs non-blocking vs async?

## Next

[TCP deep dive](../tcp-deep-dive/)
