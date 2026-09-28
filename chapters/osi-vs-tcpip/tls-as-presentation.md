# TLS as a transparent wrapper

## Context

After mapping an asyncio TCP server to the OSI layers: `writer.write()` is the transport boundary, length-prefix framing is presentation. What does it look like when you add TLS?

## Question

> Is TLS implemented like a language library? Can I literally insert it into my code and it would send ClientHello and ServerHello over TCP? And would it look like:

```python
data = (await reader.readexactly(length)).decode()
decrypted_data = tls.decrypt(data)
```

or

```python
payload = message.encode()
encrypted_data = tls.encrypt(payload)
writer.write(len(encrypted_data).to_bytes(4, "big") + encrypted_data)
```

## Answer

TLS is a library. But the real API is closer to **wrap the socket** than **call encrypt/decrypt on each message**.

### Without TLS

```python
reader, writer = await asyncio.open_connection(host, port)
writer.write(b"hello")
data = await reader.read(1024)
```

### With TLS

You upgrade the stream. After that, the writer is a TLS writer: `write(b"hello")` encrypts and sends over the underlying TCP socket; `read()` decrypts before returning.

You almost never call `encrypt()` / `decrypt()` directly. The library does it under the stream API.

### Python example

```python
import ssl
import asyncio

server_context = ssl.SSLContext(ssl.PROTOCOL_TLS_SERVER)
server_context.load_cert_chain("server.crt", "server.key")

async def connect_with_tls(host, port):
    # Plain TCP (transport)
    reader, writer = await asyncio.open_connection(host, port)

    # Upgrade to TLS (presentation)
    # Handshake: ClientHello, ServerHello, certificate, key derivation, Finished.
    # All sent as TLS records over the same TCP socket.
    await writer.start_tls(server_context)

    return reader, writer

async def main():
    reader, writer = await connect_with_tls("example.com", 443)

    message = "GET / HTTP/1.1\r\nHost: example.com\r\n\r\n"
    payload = message.encode()
    framed = len(payload).to_bytes(4, "big") + payload

    # Write to the TLS-wrapped socket — library encrypts before TCP send
    writer.write(framed)
    await writer.drain()

    response = await reader.read(4096)
    print(response)
```

(`writer.start_tls(...)` is Python 3.11+. Older code uses `loop.start_tls(transport, protocol, ssl_context)`.)

### What goes on the wire

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ TCP segment                                                                 │
│                                                                             │
│  ┌───────────────────────────────────────────────────────────────────────┐ │
│  │ TCP header                                                            │ │
│  │   src port, dst port, seq, ack, flags, window, checksum              │ │
│  └───────────────────────────────────────────────────────────────────────┘ │
│                                                                             │
│  ┌───────────────────────────────────────────────────────────────────────┐ │
│  │ TLS record                                                            │ │
│  │                                                                       │ │
│  │  ┌─────────────────────────────────────────────────────────────────┐ │ │
│  │  │ TLS record header                                               │ │ │
│  │  │   content type (application_data), version, length              │ │ │
│  │  └─────────────────────────────────────────────────────────────────┘ │ │
│  │                                                                       │ │
│  │  ┌─────────────────────────────────────────────────────────────────┐ │ │
│  │  │ ENCRYPTED PAYLOAD                                               │ │ │
│  │  │   your length prefix (4 bytes)                                  │ │ │
│  │  │   + "GET / HTTP/1.1\r\nHost: example.com\r\n\r\n"               │ │ │
│  │  │   + auth tag                                                    │ │ │
│  │  └─────────────────────────────────────────────────────────────────┘ │ │
│  └───────────────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────────────────┘
```

Your length prefix lives **inside** the encryption. TLS adds its own framing on top of your framing.

### Handshake during `start_tls()`

```
CLIENT                                    SERVER
   │  ──── ClientHello ────────────────────▶ │
   │   (version, cipher suites, random, SNI) │
   │  ◀──── ServerHello ──────────────────── │
   │  ◀──── Certificate ──────────────────── │
   │  ◀──── ServerKeyExchange / Done ─────── │
   │  ──── ClientKeyExchange ──────────────▶ │
   │  ──── ChangeCipherSpec / Finished ────▶ │
   │  ◀──── ChangeCipherSpec / Finished ──── │
   │       Encrypted application data        │
```

Every handshake message is a TLS record over the same TCP connection. From the app's view, you await `start_tls()` and then have an encrypted channel.

### Rust (`tokio-rustls`)

```rust
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_rustls::TlsConnector;
use tokio_rustls::rustls::{ClientConfig, RootCertStore};

async fn connect_with_tls(host: &str, port: u16) -> anyhow::Result<()> {
    let mut root_store = RootCertStore::empty();
    root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

    let config = ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    let connector = TlsConnector::from(std::sync::Arc::new(config));

    let tcp = TcpStream::connect(format!("{host}:{port}")).await?;
    let server_name = host.to_string().try_into()?;
    let mut tls_stream = connector.connect(server_name, tcp).await?;

    let message = b"hello";
    let framed = [&(message.len() as u32).to_be_bytes()[..], message].concat();

    tls_stream.write_all(&framed).await?;
    tls_stream.flush().await?;

    let mut buf = vec![0u8; 1024];
    let n = tls_stream.read(&mut buf).await?;
    println!("Read {n} bytes: {:?}", &buf[..n]);
    Ok(())
}
```

`TlsStream` implements the same `AsyncRead` / `AsyncWrite` as `TcpStream`. Your framing code does not change — only the stream type does.

### Why wrap instead of `encrypt()` / `decrypt()`?

TLS state is **per connection**, not per message:

- session keys
- record sequence numbers (anti-replay)
- cipher state
- handshake state

If you called `tls.encrypt(payload)` yourself, you would have to track which connection the payload belongs to, increment sequence numbers exactly once, split oversized payloads into records, etc. Wrapping the socket makes the library own that.

### Insight

> So I never call encrypt/decrypt directly? TLS is just… transparent?

Yes. Plaintext above the TLS object, ciphertext on the wire. Framing, protocol, and application logic stay the same; you only upgraded the socket.

```python
# Without TLS
reader, writer = await asyncio.open_connection(host, port)

# With TLS
reader, writer = await asyncio.open_connection(host, port)
await writer.start_tls(ssl_context)
```

### Summary

| Concept | Role |
|---------|------|
| `SSLContext` / `ClientConfig` | versions, certs, ciphers |
| `start_tls()` / `TlsConnector` | upgrade TCP → TLS |
| TLS handshake | ClientHello → … → Finished over TCP |
| TLS record | framing TLS adds above TCP |
| TLS reader/writer | encrypt on write, decrypt on read |
| Your framing | lives inside the ciphertext |
