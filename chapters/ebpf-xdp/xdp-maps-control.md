# eBPF, XDP, maps, and the control loop

## Question

> How does eBPF/XDP work?
> What's the difference between kernel modules and eBPF?
> How does XDP intercept packets at the NIC driver level?
> How do eBPF programs communicate with userspace?
> What is an eBPF map? A ring buffer?
> How does dosd communicate with eBPF programs?
> What is xdpd? How does l4drop differ from Unimog?

## Answer

### eBPF vs kernel modules

| | Kernel module | eBPF |
|-|---------------|------|
| Power | full kernel access | restricted helpers + verifier |
| Crash | can panic the machine | verifier rejects unsafe programs |
| Reload | often painful | hot-reload without reboot |
| Use at CF | historically more | l4drop, Unimog, Tubular, … |

### XDP

Runs at the **NIC driver**, before the full network stack. Typical return codes:

- `XDP_DROP` — discard (cheap DDoS mitigation)
- `XDP_PASS` — continue up the stack
- `XDP_TX` — bounce out the same NIC
- `XDP_REDIRECT` — to another NIC / CPU / map destination

### Maps and ring buffers

**Map** — shared key/value memory between kernel eBPF and userspace (hash, array, LPM trie, per-CPU array, …). Userspace updates rules; XDP reads them on every packet. Or XDP increments counters; userspace scrapes them.

**Ring buffer** — high-volume event stream toward userspace (telemetry).

### Roles

| Component | Plane | Job |
|-----------|-------|-----|
| **l4drop** | data | XDP: drop by attack rules in maps |
| **Unimog** | data | XDP/TC: L4 load balance / steer |
| **dosd** | control | analyze traffic, decide mitigations, **write maps** |
| **xdpd** | control | load XDP programs, sync config (e.g. Quicksilver), metrics |

Insight:

> So dosd is a userspace program that receives info, makes decisions, and can update eBPF maps where Unimog/l4drop search for commands to drop a packet?

Exactly. dosd decides. XDP executes at line rate. Control plane configures data plane.

### XDP drop program (pseudo-C)

```c
// Runs in the kernel at the NIC driver level.
// Reads attack rules from an eBPF map and drops matching packets.

struct attack_rule {
    __u32 src_ip;
    __u32 dst_ip;
    __u16 src_port;
    __u16 dst_port;
    __u8  protocol;
};

BPF_MAP_DEF(attack_rules, BPF_MAP_TYPE_HASH, __u32, struct attack_rule, 1024);
BPF_MAP_DEF(packet_counts, BPF_MAP_TYPE_PERCPU_ARRAY, __u32, __u64, 256);

SEC("xdp")
int l4drop(struct xdp_md *ctx) {
    void *data = (void *)(long)ctx->data;
    void *data_end = (void *)(long)ctx->data_end;

    struct ethhdr *eth = data;
    if ((void *)(eth + 1) > data_end) return XDP_PASS;
    if (eth->h_proto != htons(ETH_P_IP)) return XDP_PASS;

    struct iphdr *ip = (void *)(eth + 1);
    if ((void *)(ip + 1) > data_end) return XDP_PASS;
    if (ip->protocol != IPPROTO_TCP && ip->protocol != IPPROTO_UDP)
        return XDP_PASS;

    __u16 src_port, dst_port;
    if (ip->protocol == IPPROTO_TCP) {
        struct tcphdr *tcp = (void *)(ip + 1);
        if ((void *)(tcp + 1) > data_end) return XDP_PASS;
        src_port = tcp->source;
        dst_port = tcp->dest;
    } else {
        struct udphdr *udp = (void *)(ip + 1);
        if ((void *)(udp + 1) > data_end) return XDP_PASS;
        src_port = udp->source;
        dst_port = udp->dest;
    }

    __u32 key = hash_5tuple(ip->saddr, ip->daddr, src_port, dst_port, ip->protocol);

    struct attack_rule *rule = bpf_map_lookup_elem(&attack_rules, &key);
    if (rule) {
        __u64 *counter = bpf_map_lookup_elem(&packet_counts, &rule->protocol);
        if (counter) __sync_fetch_and_add(counter, 1);
        return XDP_DROP;
    }

    return XDP_PASS;
}
```

### Updating maps from userspace (pseudo-Rust)

```rust
// What dosd or xdpd does in userspace:
// read config, generate rules, update eBPF maps.

fn update_attack_rules(
    map: &mut BpfMap<u32, AttackRule>,
    config: &QuicksilverConfig,
) -> Result<()> {
    let rules = config.get_attack_rules()?;

    for key in map.keys()? {
        map.remove(&key)?;
    }

    for rule in rules {
        let key = hash_5tuple(
            rule.src_ip,
            rule.dst_ip,
            rule.src_port,
            rule.dst_port,
            rule.protocol,
        );
        map.insert(&key, &rule)?;
    }
    Ok(())
}

async fn xdpd_main_loop() {
    let mut map = BpfMap::open("/sys/fs/bpf/attack_rules")?;
    let mut quicksilver = QuicksilverClient::connect()?;

    loop {
        let update = quicksilver.watch("attack_rules").await?;
        update_attack_rules(&mut map, &update)?;

        let counts = map.get_all_counters()?;
        prometheus::update_metrics(counts);
    }
}
```

**l4drop** vs **Unimog:** both can sit at XDP; l4drop's job is drop/mitigate, Unimog's job is steer/load-balance (and related redirector logic). Same toolbox, different programs and maps.
