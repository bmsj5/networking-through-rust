# eBPF & XDP

## Learning Objectives

- Contrast eBPF with kernel modules
- Know XDP actions: DROP, PASS, TX, REDIRECT
- See maps/ring buffers as kernel↔userspace IPC
- Place dosd / xdpd / l4drop in the control/data plane

## Topics

1. [XDP, Maps, Control Loop](./xdp-maps-control.md)

## Questions

- eBPF vs kernel modules?
- How does XDP intercept at the NIC driver?
- Maps? Ring buffers? dosd ↔ eBPF?
- xdpd? l4drop vs Unimog?

## Key Insight

> dosd **decides**. XDP programs **execute** rules from maps at line rate. Control plane configures data plane.

## Next

[Overlay Networks](../overlay-networks/)
