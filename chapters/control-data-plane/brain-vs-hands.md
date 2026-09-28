# Control plane vs data plane

## Question

> What is control plane vs data plane?
> Does Cloudflare run the same software on every edge server?
> How do Unimog, FL2, and dosd run on the same server?
> How do they pass packets internally?
> How do they identify individual servers for management?
> What is Quicksilver and what does it store?
> How does config propagate to 300+ data centers?
> What are failure modes in distributed systems?

## Answer

### Decide vs execute

> So control plane is just some management tools/nodes or even data in Quicksilver that all edge workers should execute, and they are data plane?

Yes. Control plane **decides**. Data plane **executes**.

| Control plane | Data plane |
|---------------|------------|
| Conductor (generates forwarding tables) | Unimog (L4 load balancer) |
| dosd (detects attacks, generates rules) | l4drop (drops packets) |
| Quicksilver (distributes config) | FL2 (TLS termination, HTTP) |
| xdpd (loads XDP programs) | Tubular (socket dispatch) |

The brain does not lift every box; the hands do not invent strategy.

### Same software on every edge server

> Does that mean Cloudflare runs the same software on every edge server?

Yes. Unimog, FL2, dosd, and the rest run on every edge machine. Any machine can take on work. Requests often cross **many machines** inside a PoP on their way through the stack.

### Identity

Customer traffic: public AnyIP on the front NIC.  
Internal management / machine identity: private IPs and overlay IPs — not the customer Anycast space. See [Overlays](../overlay-networks/).

### Quicksilver (shape)

Global KV for configuration: writes are relatively rare; every edge reads constantly. Hierarchical replication (global → regional → edge) keeps reads local. Edge processes apply what was pushed; they do not invent global policy alone.

### Failure modes worth naming

Process crash, network partition, slow node, split brain, cascading overload. Mitigations: timeouts, backoff, isolation, load shedding, quorum where consistency matters. The happy-path diagram is incomplete without these.

Next: [Cloudflare case study](../cloudflare-case-study/) — NIC → XDP → Tubular → FL2.
