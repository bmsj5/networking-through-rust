# What is a socket?

## Context

After mapping code to the stack: what actually happens when `writer.write()` runs? Where do the bytes go before the wire?

## Question

> What is a socket? Is that just that TcpSocket you showed — a program that simply has its own state and methods and also can write to a memory chunk on my memory stick and then those bytes are given to some program with an async system call?

## Answer

Yes. That intuition is right.

A socket is a **kernel-managed endpoint** for network communication, represented to your program as a **file descriptor**. The kernel keeps something like `struct tcp_sock` with sequence numbers, window sizes, and connection state. Your app writes to a userspace buffer; the kernel copies those bytes into a **kernel send buffer** and asynchronously pushes them out the NIC.

You cannot touch the network card from userspace. You ask the kernel via system calls: `socket()`, `bind()`, `connect()`, `write()`, `read()`, …

## Related insight

> So OSI is just a guideline… any app decides itself on presentation and session layers… they are all just programs that are allowed to use kernel's network calls to write to a TCP socket?

Yes. The kernel is a privileged gatekeeper. Presentation and session live in your process. The socket is the handle for talking to the kernel's network stack.

## What the kernel keeps per socket (conceptual)

```rust
// Simplified view of what the kernel keeps per socket
struct tcp_sock {
    snd_nxt: u32,       // Next sequence to send
    snd_una: u32,       // Oldest unacknowledged
    rcv_nxt: u32,       // Next sequence expected
    snd_wnd: u16,       // Send window
    rcv_wnd: u16,       // Receive window
    state: TcpState,    // ESTABLISHED, CLOSE_WAIT, etc.
    // ... plus retransmit timers, congestion window, sk_buffs, etc.
}
```

## How the kernel picks which socket gets a packet

Demultiplex by **5-tuple**:

`(src_ip, src_port, dst_ip, dst_port, protocol)`

- Listening socket: match local address/port (+ protocol); source can be wildcard
- Established socket: match the full tuple

## What `write()` / `writer.write()` does

1. App provides bytes (userspace buffer)
2. Kernel copies into the socket **send buffer** (or blocks / returns EAGAIN if non-blocking and full)
3. TCP decides when it may send (receive window, congestion window, Nagle, etc.)
4. Kernel builds TCP segments → IP packets → driver → NIC
5. ACKs arrive later; acknowledged data leaves the send buffer so more writes can proceed

Receive path is the reverse: NIC → kernel receive buffer → `read()` copies to userspace.

## TCP connection state machine (pseudo-code)

This is the logic the kernel maintains **per socket** — not something your app reimplements (unless you write a userspace TCP stack).

```rust
// Simplified TCP connection state machine
// This is what the kernel keeps per socket.

struct TcpConnection {
    // Identity
    src_ip: IpAddr,
    src_port: u16,
    dst_ip: IpAddr,
    dst_port: u16,

    // State
    state: TcpState,

    // Sequence numbers
    snd_nxt: u32,       // Next sequence to send
    snd_una: u32,       // Oldest unacknowledged
    rcv_nxt: u32,       // Next sequence expected

    // Windows
    snd_wnd: u16,       // Send window (from peer)
    rcv_wnd: u16,       // Receive window (advertised to peer)

    // Buffers
    send_buffer: VecDeque<u8>,
    receive_buffer: VecDeque<u8>,

    // Retransmission
    unacked_segments: Vec<Segment>,
    retransmit_timer: Option<Timer>,

    // Congestion control
    cwnd: u32,          // Congestion window
    ssthresh: u32,      // Slow start threshold
}

enum TcpState {
    Closed,
    Listen,
    SynSent,
    SynReceived,
    Established,
    FinWait1,
    FinWait2,
    CloseWait,
    Closing,
    LastAck,
    TimeWait,
}

impl TcpConnection {
    // --- Connection establishment ---

    fn send_syn(&mut self) {
        self.snd_nxt = random_u32();
        self.state = TcpState::SynSent;
        self.send_packet(TcpPacket {
            seq: self.snd_nxt,
            ack: 0,
            flags: Flags::SYN,
            payload: vec![],
        });
    }

    fn receive_syn_ack(&mut self, packet: TcpPacket) {
        self.snd_una = packet.ack;
        self.snd_nxt = packet.ack;
        self.rcv_nxt = packet.seq + 1;
        self.state = TcpState::Established;
        self.send_packet(TcpPacket {
            seq: self.snd_nxt,
            ack: self.rcv_nxt,
            flags: Flags::ACK,
            payload: vec![],
        });
    }

    // --- Data transfer ---

    fn send_data(&mut self, data: &[u8]) {
        let chunk_size = min(self.cwnd, self.snd_wnd as u32, MSS);
        for chunk in data.chunks(chunk_size as usize) {
            let seq = self.snd_nxt;
            let packet = TcpPacket {
                seq,
                ack: self.rcv_nxt,
                flags: Flags::ACK | Flags::PSH,
                payload: chunk.to_vec(),
            };
            self.send_packet(packet);
            self.unacked_segments.push(Segment {
                seq,
                data: chunk.to_vec(),
                sent_at: now(),
                retries: 0,
            });
            self.snd_nxt += chunk.len() as u32;
        }
        self.start_retransmit_timer();
    }

    fn receive_data(&mut self, packet: TcpPacket) {
        if packet.seq == self.rcv_nxt {
            self.receive_buffer.extend(&packet.payload);
            self.rcv_nxt += packet.payload.len() as u32;

            self.send_packet(TcpPacket {
                seq: self.snd_nxt,
                ack: self.rcv_nxt,
                flags: Flags::ACK,
                payload: vec![],
            });

            self.wake_up_application();
        } else if packet.seq < self.rcv_nxt {
            // Duplicate; send ACK again
            self.send_ack();
        } else {
            // Out of order; buffer for later
            self.out_of_order_buffer.push(packet);
        }
    }

    fn receive_ack(&mut self, ack_num: u32) {
        self.unacked_segments.retain(|seg| {
            seg.seq + seg.data.len() as u32 > ack_num
        });
        self.snd_una = ack_num;

        // Congestion control (classic sketch)
        if self.in_slow_start() {
            self.cwnd *= 2;
            if self.cwnd >= self.ssthresh {
                self.enter_congestion_avoidance();
            }
        } else {
            self.cwnd += 1; // linear increase (simplified)
        }
    }

    // --- Retransmission ---

    fn on_retransmit_timeout(&mut self, seq: u32) {
        for seg in &mut self.unacked_segments {
            if seg.seq == seq {
                if seg.retries < MAX_RETRIES {
                    seg.retries += 1;
                    seg.sent_at = now();
                    self.send_packet(TcpPacket {
                        seq: seg.seq,
                        ack: self.rcv_nxt,
                        flags: Flags::ACK,
                        payload: seg.data.clone(),
                    });
                    // Exponential backoff
                    self.start_retransmit_timer(
                        INITIAL_TIMEOUT * 2u32.pow(seg.retries)
                    );
                } else {
                    self.close();
                }
                break;
            }
        }
    }

    // --- Connection termination ---

    fn send_fin(&mut self) {
        self.send_packet(TcpPacket {
            seq: self.snd_nxt,
            ack: self.rcv_nxt,
            flags: Flags::FIN | Flags::ACK,
            payload: vec![],
        });
        self.snd_nxt += 1;
        self.state = TcpState::FinWait1;
    }

    fn receive_fin(&mut self, packet: TcpPacket) {
        self.rcv_nxt += 1;
        self.send_ack();
        match self.state {
            TcpState::Established => {
                self.state = TcpState::CloseWait;
                // Application decides when to close its side
            }
            TcpState::FinWait2 => {
                self.state = TcpState::TimeWait;
                self.start_time_wait_timer();
            }
            _ => {}
        }
    }

    fn send_ack(&mut self) {
        self.send_packet(TcpPacket {
            seq: self.snd_nxt,
            ack: self.rcv_nxt,
            flags: Flags::ACK,
            payload: vec![],
        });
    }

    fn close(&mut self) {
        self.state = TcpState::Closed;
        self.release_resources();
    }
}
```

Your app does not write this. Your app calls `write()`; the kernel runs this kind of machine on the socket.

## Demo

```bash
cd code && cargo run --bin tcp-echo-server
# nc 127.0.0.1 7878
```

Also: `cargo run --bin tcp-state-machine` for a tiny walkthrough of open → data → FIN.
