# Mapping your own code to the stack

## Context

A custom binary TCP protocol in Python with asyncio. Where does that code sit in OSI / TCP-IP terms?

## Question

> Does that mean, in my custom binary TCP protocol, the 6th layer (presentation) is when I do
> `writer.write(len(payload).to_bytes(4, 'big') + payload)`?
> I'm structuring the payload the way I need. But I could also do encryption, compression — not only translation. So is TLS the 6th layer thing?

## Answer

Yes. Length-prefix framing is presentation-layer work: it turns your Python value into bytes and adds a framing header so the peer knows message boundaries. Compression or encryption is more presentation-layer work. TLS operates here too — it encrypts and decrypts before data is useful to the application.

## Session layer in code

> Can I say that my infinite connection loop is the session layer?

```python
async def start(self):
    while True:
        try:
            await self._connection()
        except Exception as e:
            self.logger.error(f"Connection lost: {e}")
            await asyncio.sleep(3)
```

Yes, in OSI terms. That loop manages connection lifecycle — open, detect failure, close, reopen. TCP provides the reliable byte stream; your loop provides session management on top.

In TCP/IP terms this is not a separate kernel layer. It is application code managing a TCP connection. The “session” is the TCP connection itself, plus whatever policy you wrap around it.

## Full mapping

| OSI Layer | Your code |
|-----------|-----------|
| 7. Application | `message = '{"type": "ping"}'` |
| 6. Presentation | `payload = message.encode()` + framing (+ TLS) |
| 5. Session | `while True: try: await _connection()` |
| 4. Transport | `asyncio.open_connection()` (TCP) |
| 3. Network | OS kernel (IP) |
| 2. Data Link | NIC driver (Ethernet) |
| 1. Physical | Fiber / cable / Wi‑Fi |

Next: [TLS as a transparent wrapper](./tls-as-presentation.md) — how TLS actually plugs into this without calling `encrypt()` yourself.
