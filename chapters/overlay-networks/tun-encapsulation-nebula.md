# Overlays, TUN, and inner TCP

## Question

> How does an overlay network work?
> How can TCP be inside UDP packets?
> What is encapsulation? What is a TUN/TAP device?
> How does the kernel process an inner packet?
> Who processes the inner TCP state machine — kernel or app?
> What happens to ACKs, congestion control, retransmissions?
> Does WireGuard implement TCP?
> How does Nebula work with lighthouses?
> How do overlay IPs map to real IPs?
> Do Cloudflare servers use public or private IPs internally?

## Answer

### Overlay vs underlay

- **Underlay** — real IPs and real routes (ISP, DC fabric).
- **Overlay** — virtual IPs; traffic between them is carried *inside* underlay packets.

### Encapsulation

```text
[ UDP | encrypted( [ IP_inner | TCP | payload ] ) ]
   ↑ outer envelope to peer's real IP : VPN port (e.g. 4242)
```

1. Kernel delivers outer UDP to the VPN process bound on that port.
2. VPN decrypts → inner IP packet.
3. VPN `write()`s the inner packet to a **TUN** FD.
4. Kernel injects it on the virtual interface (`nebula1`, `wg0`, …) and routes/TCP-processes normally.

**TUN** = virtual L3 interface (IP packets) backed by a file descriptor.  
**TAP** = virtual L2 interface (Ethernet frames).

### Insight: two packets

> Does that mean the kernel sees two separate packets: the outer UDP packet on ens3 and the inner IP packet on nebula1?

Yes. Completely separate. The kernel does not know they are related. The VPN process is the bridge: read outer → decrypt → inject inner via TUN.

### Who runs inner TCP?

> If WireGuard doesn't implement TCP, who handles the inner TCP state machine?

| Tunnel style | Inner TCP |
|--------------|-----------|
| L3 tunnel (WireGuard, Nebula) | **Kernel** — VPN is a dumb pipe |
| Userspace proxy with its own TCP stack | **Application** |

For WireGuard/Nebula: ACKs, retransmits, congestion control for the *inner* TCP connection are done by the kernels at the endpoints, as if the overlay were a normal interface. The outer UDP is just transport for encrypted inner IP packets.

### Nebula / lighthouses

Lighthouses help peers discover current underlay endpoints. Once known, peers often build **direct** tunnels. Overlay IPs identify peers; underlay IPs can change (roaming, new public IP) without changing overlay identity.

### Cloudflare-shaped interfaces (correction from the conversation)

> Do Cloudflare servers have their own public IPs, or private IPs?

Both, for different jobs:

| Interface | Role |
|-----------|------|
| eth0 (example) | All Cloudflare **public** IPs (AnyIP) — customer traffic |
| eth1 | Unique **private** IP — internal communication |
| nebula1 | **Overlay** IP on top of eth1 |

Internal traffic uses private / overlay addresses. Public Anycast space is for customers, not for machine-to-machine chatter inside the PoP.
