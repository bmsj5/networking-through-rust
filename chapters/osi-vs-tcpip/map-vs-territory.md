# The map is not the territory

## Question

> TCP establishes connections. Articles say the session layer initializes the session. Which is it?

## Answer

The OSI model describes an idealized 7-layer stack. The real internet runs on **TCP/IP**, which collapses OSI layers 5–7 into a single Application layer.

When an OSI article says “session layer initializes the communication session,” that is theory. In practice TCP does that — and TCP sits at what people call OSI layer 4, not 5.

OSI is a teaching tool. TCP/IP is what actually runs.

## Insight

> So OSI is just a guideline on how the internet should look like, and it means they suggest to implement presentation and session layers, but actually in every kernel there's just TCP and UDP, and in my network card there's Ethernet, and any app decides itself on presentation and session layers. But practically they are all just programs that are allowed to use kernel's network calls to write to a TCP socket?

Yes. Exactly.

The kernel only implements Transport (TCP/UDP) and Network (IP). Everything above — TLS, HTTP, your custom binary protocol, your reconnect loop — is userspace. The kernel gives you system calls. Your program decides what to do with them.

## Related confusion

> But if I have HTTP or HTTPS, I already implement the full OSI model.

You don't “implement OSI.” You implement TCP/IP, and OSI is the vocabulary people use to describe it. Layers 5–7 are collapsed into TCP/IP's Application layer. You can map your code onto all seven OSI names, but OSI is not a software stack you install.

## Layer mapping

| OSI | TCP/IP | What actually runs |
|-----|--------|--------------------|
| 7 Application | Application | HTTP, DNS, your protocol |
| 6 Presentation | Application | TLS, encoding, compression |
| 5 Session | Application | reconnect loops, session logic |
| 4 Transport | Transport | TCP / UDP (kernel) |
| 3 Network | Internet | IP (kernel) |
| 2 Data Link | Link | Ethernet / Wi‑Fi (NIC + driver) |
| 1 Physical | Link | fiber, copper, radio |

## Encapsulation and decapsulation

Going out:

```
App data
  → [TCP hdr | app data]          segment
  → [IP hdr  | TCP | data]        packet
  → [Eth hdr | IP | TCP | data]   frame
  → bits on the wire
```

Coming in, the receiver peels headers in reverse.

## Where familiar pieces sit

| Technology | OSI-ish layer | Reality |
|------------|---------------|---------|
| HTTP | 7 | userspace library / app |
| TLS | 6 | userspace library wrapping a socket |
| TCP | 4 | kernel |
| IP | 3 | kernel |
| Ethernet | 2 | NIC + driver |
