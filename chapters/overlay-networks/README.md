# Overlay Networks & Tunneling

## Learning Objectives

- Explain overlay vs underlay
- Trace outer UDP → VPN process → TUN → inner IP
- Know who runs the inner TCP state machine (kernel vs userspace)

## Topics

1. [TUN, Encapsulation, Nebula](./tun-encapsulation-nebula.md)

## Questions

- TCP inside UDP? Who decrypts? Who ACKs?
- TUN/TAP?
- Does WireGuard implement TCP?
- Nebula lighthouses / overlay IP mapping?

## Key Insight

> Kernel sees **two** packets: outer UDP on the real NIC, inner IP on `nebula1`/`wg0`. The VPN process is the bridge. WireGuard is a dumb L3 pipe — kernel TCP handles the inner stream.

## Next

[Anycast & BGP](../anycast-bgp/)
