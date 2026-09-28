# Headers, TTL, and fragmentation

## Question

> What are all the IPv4 header fields?
> What is TTL and why does it matter?
> What are IP flags (DF, MF) and fragment offset?
> Why do we need IP fragmentation if TCP already segments?
> What is Path MTU Discovery?
> What is ICMP?

## Answer

### IPv4 header fields

| Field | Role |
|-------|------|
| Version / IHL | IPv4; header length |
| DSCP / ECN | QoS / congestion notification |
| Total length | header + payload |
| Identification | groups fragments of one original datagram |
| Flags | **DF** Don't Fragment; **MF** More Fragments |
| Fragment offset | position of this fragment in the original |
| TTL | hop budget |
| Protocol | 6=TCP, 17=UDP, 1=ICMP, … |
| Header checksum | IP header only |
| Source IP / Dest IP | end-to-end (unchanged at normal routers) |

### TTL

Every router: `ttl -= 1`. If TTL hits 0 → drop + typically **ICMP Time Exceeded**. Stops routing loops. Enables traceroute (probes with increasing TTL).

### TCP segmentation vs IP fragmentation

**TCP** segments the byte stream using **MSS** negotiated in the handshake. That is normal.

**IP fragmentation** happens when a packet is larger than the **outgoing link MTU** on some hop. The original sender may not know every MTU along the path.

If **DF=1** and the packet is too big:

- Router drops it
- Sends ICMP **Fragmentation Needed** (Dest Unreachable, code 4)
- Sender shrinks — this is **Path MTU Discovery (PMTUD)**

So: TCP segments because it manages the stream. IP fragments (or ICMP + PMTUD) because some middle link is smaller than expected.

### Fragmentation on a router (pseudo)

```text
if packet.len > out_mtu:
  if DF set:
    icmp_fragmentation_needed(packet)
    drop
  else:
    split into fragments:
      same Identification
      increasing Fragment Offset
      MF=1 on all but last
```

### ICMP types you meet early

| Type / meaning | Use |
|----------------|-----|
| Time Exceeded | traceroute; TTL expired |
| Dest Unreachable / Frag Needed | PMTUD |
| Echo Request / Reply | ping |

ICMP is not a transport for your app data. It is control/error messaging for IP (and tools built on it).

### Demo

```bash
cd code && cargo run --bin ip-header-parser
```
