# Cloudflare Case Study

## Learning Objectives

- Name the major edge components and their jobs
- Trace NIC → XDP → kernel → Tubular → FL2
- Relate Rapid Reset to L7 proxy stress

## Topics

1. [Edge Stack & Packet Path](./edge-stack-and-path.md)

## Questions

- Unimog vs l4drop vs Tubular vs FL2 vs dosd vs xdpd?
- Same binary set on every machine?
- HTTP/2 Rapid Reset exposure?

## Key Insight

> One uniform edge image. Anycast lands you in a PoP. XDP filters/steers early. Kernel + Tubular deliver to the right socket. FL2 terminates TLS and speaks HTTP. Control daemons continually retune maps from global config and telemetry.

## Journey Recap

This repo's path: OSI confusion → sockets → packets → Unimog daisy-chain → eBPF drops → Anycast IPs → overlays → control/data plane → this case study.
