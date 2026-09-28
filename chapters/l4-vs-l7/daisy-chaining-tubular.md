# Daisy chaining and Tubular

## Question

> So Unimog sends packets to some server, and then it checks if it doesn't have that connection yet, then it sends it to the old server in slot 2? Wtf? Doesn't make sense. For that to happen every server has to somehow check it first, and then how come the server can accept a new connection at all?

> But how does the redirector know there's an active connection? Is it a socket lookup?

> So Tubular doesn't have to do anything with that?

## Answer

### Daisy chaining

When Cloudflare wants to drain Server A, it updates the forwarding table so **new** connections go to Server B. Existing connections on A must keep working.

Each forwarding-table bucket has two slots: Slot 1 (new server) and Slot 2 (old server).

Packet arrives at Server B:

1. If it is a **SYN** (new connection) → B accepts it.
2. If it is a **non-SYN** (data/ACK/FIN for an existing connection) → B checks for a **local socket**.
   - Socket found → process locally.
   - No socket → forward to Slot 2 (Server A), which still holds the connection.

The check must happen **before** the normal TCP stack would generate RST for “no socket.”

### Redirector = socket lookup

Yes. The redirector is an eBPF/TC (or similar) program that does a socket lookup early. Found → local. Missing → second hop (daisy chain). That preserves connections across table changes.

### Tubular is separate

| | Unimog redirector | Tubular |
|-|-------------------|---------|
| Question | Which **server** in the PoP owns this connection? | Which **application socket** on *this* server should receive it? |
| Hook | XDP/TC in the L4 path | `sk_lookup` |
| Scope | across machines | on one machine |

Unimog decides the machine. Tubular decides the listening socket (e.g. dispatch by destination IP in a huge AnyIP range). Different stages, different programs.

### Redirector pseudo-code

```text
fn on_packet(pkt):
  if is_syn(pkt):
    # New connection — this server (slot 1 owner) accepts
    return LOCAL

  if socket_lookup(five_tuple(pkt)).is_some():
    return LOCAL

  # Existing connection we don't own — send to previous owner
  return FORWARD(slot2_of(bucket(hash_5tuple(pkt))))
```
