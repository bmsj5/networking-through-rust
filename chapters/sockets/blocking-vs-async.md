# Blocking, non-blocking, and async

## Question

> How does asyncio work with sockets? What's the difference between blocking and non-blocking I/O?

## Answer

### Blocking

`read` / `write` wait inside the kernel until they can make progress (or fail). The calling thread sleeps.

### Non-blocking

If the operation cannot finish now, the syscall returns immediately with `EAGAIN` / `EWOULDBLOCK`. The socket still has the same send/receive buffers; you just poll or wait for readiness somehow else.

### Async (asyncio, Tokio, …)

Same sockets. The runtime:

1. Sets the FD non-blocking
2. Registers interest with the OS (`epoll`, `io_uring`, `kqueue`, …)
3. Wakes your task when the FD is readable/writable

So `await writer.drain()` is not a different network stack. It is “suspend this task until the kernel says this socket can accept more bytes,” then continue the same send-buffer path described in [What is a socket?](./what-is-a-socket.md).

`writer.write()` in asyncio often copies into a userspace buffer first, then drains to the kernel when writable — still ends in the kernel socket buffer.

### System calls

```
socket()    create endpoint → FD
bind()      assign local address
listen()    mark passive (server)
accept()    new FD for one connection
connect()   active open (SYN)
write/read  copy through kernel buffers
shutdown/close
```

### epoll / io_uring (why async scales)

With thousands of connections, one thread blocked per socket does not work. Event APIs let one thread wait on many FDs: “wake me when any of these can read/write.” That is how a single asyncio/Tokio process handles many sockets without one OS thread each.

## Demo

`tcp-echo-server` (Tokio): accept loop, read until EOF, echo bytes back — same kernel path as a Python asyncio echo server.
