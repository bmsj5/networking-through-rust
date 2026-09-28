//! Educational TCP state-machine walkthrough (not a real stack).

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TcpState {
    Closed,
    SynSent,
    Established,
    FinWait1,
    TimeWait,
}

struct Conn {
    state: TcpState,
    snd_nxt: u32,
    rcv_nxt: u32,
}

impl Conn {
    fn new() -> Self {
        Self {
            state: TcpState::Closed,
            snd_nxt: 0,
            rcv_nxt: 0,
        }
    }

    fn send_syn(&mut self) {
        self.snd_nxt = 1000;
        self.state = TcpState::SynSent;
        println!("→ SYN seq={}", self.snd_nxt);
        println!("  state = {:?}", self.state);
    }

    fn recv_syn_ack(&mut self, peer_seq: u32) {
        self.rcv_nxt = peer_seq + 1;
        self.snd_nxt += 1;
        self.state = TcpState::Established;
        println!("← SYN-ACK seq={peer_seq} ack={}", self.snd_nxt);
        println!("→ ACK ack={}", self.rcv_nxt);
        println!("  state = {:?}", self.state);
    }

    fn send_data(&mut self, len: u32) {
        println!("→ DATA seq={} len={len}", self.snd_nxt);
        self.snd_nxt += len;
    }

    fn recv_ack(&mut self, ack: u32) {
        println!("← ACK ack={ack}");
    }

    fn send_fin(&mut self) {
        println!("→ FIN seq={}", self.snd_nxt);
        self.snd_nxt += 1;
        self.state = TcpState::FinWait1;
        println!("  state = {:?}", self.state);
    }

    fn recv_fin_ack_path(&mut self) {
        println!("← ACK (for our FIN)");
        println!("← FIN");
        self.rcv_nxt += 1;
        println!("→ ACK");
        self.state = TcpState::TimeWait;
        println!("  state = {:?} (then → Closed)", self.state);
        self.state = TcpState::Closed;
    }
}

fn main() {
    println!("TCP state machine (client view)\n");
    let mut c = Conn::new();
    c.send_syn();
    c.recv_syn_ack(5000);
    c.send_data(11); // "hello world"
    c.recv_ack(c.snd_nxt);
    c.send_fin();
    c.recv_fin_ack_path();
    println!("\ndone. Real stacks add RST, simultaneous close, TIME_WAIT timer, etc.");
}
