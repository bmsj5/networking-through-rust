# OSI vs TCP/IP

This is where it started: the seven layers looked clean in articles, then conflicted with what TCP actually does.

## Chapters

1. [The map is not the territory](./map-vs-territory.md) — OSI vs what the kernel really runs
2. [Mapping your own code](./your-code-on-the-stack.md) — where framing, reconnect loops, and TLS sit
3. [TLS as a transparent wrapper](./tls-as-presentation.md) — you don't call encrypt() yourself

## Questions that came up

- What are the 7 layers of the OSI model?
- How does data actually flow through the layers?
- If I use HTTP or HTTPS, am I "implementing" the full OSI model?
- Why do people say the Internet is built on TCP/IP if OSI has more layers?
- Are presentation and session layers optional?
- What's the actual difference between OSI and TCP/IP?

## Next

[How a packet travels](../packet-journey/)
