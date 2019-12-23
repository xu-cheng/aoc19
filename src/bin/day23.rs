use aoc2019::computer::*;
use aoc2019::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Packet {
    dst: Int,
    x: Int,
    y: Int,
}

struct Controller(Instant);

impl Controller {
    fn new(prog: &Program, addr: i64) -> Self {
        Self(prog.start_with_input(&[addr, -1]))
    }

    fn send(&mut self, p: Packet) {
        self.0.push_input(p.x);
        self.0.push_input(p.y);
    }

    fn send_if_empty(&mut self) {
        if self.0.input.is_empty() {
            self.0.push_input(-1);
        }
    }

    fn output(&mut self) -> Result<Vec<Packet>> {
        while self.0.step()? != StepResult::WaitInput {}
        let mut out = Vec::new();
        for mut data in &self.0.output_iter().chunks(3) {
            out.push(Packet {
                dst: *data.next().context("failed to get dst")?,
                x: *data.next().context("failed to get x")?,
                y: *data.next().context("failed to get y")?,
            });
        }
        self.0.clear_output();
        Ok(out)
    }
}

struct Network {
    controllers: Vec<Controller>,
    nat: Option<Packet>,
}

enum NatState {
    Receive(Packet),
    Send(Packet),
}

impl Network {
    fn new(prog: &Program, size: Int) -> Self {
        let mut controllers: Vec<Controller> = Vec::with_capacity(50);
        for i in 0..size {
            controllers.push(Controller::new(&prog, i));
        }
        Self {
            controllers,
            nat: None,
        }
    }

    fn run(&mut self) -> Result<NatState> {
        let mut out: Option<NatState> = None;
        loop {
            let mut packets: Vec<Packet> = Vec::new();
            for c in &mut self.controllers {
                let mut c_packets = c.output()?;
                packets.append(&mut c_packets);
            }

            if packets.is_empty() {
                let p = self.nat.context("nat is empty")?;
                self.controllers[0].send(p);
                out = Some(NatState::Send(p));
            } else {
                for p in packets {
                    if p.dst == 255 {
                        self.nat = Some(p);
                        out = Some(NatState::Receive(p));
                    } else {
                        self.controllers[p.dst as usize].send(p);
                    }
                }
            }

            for c in &mut self.controllers {
                c.send_if_empty();
            }

            if let Some(out) = out {
                return Ok(out);
            }
        }
    }
}

fn main() -> Result<()> {
    let prog = Program::load_from_input("day23.txt")?;
    let mut network = Network::new(&prog, 50);

    let p = match network.run()? {
        NatState::Receive(p) => p,
        _ => unreachable!(),
    };
    println!("ans1={:?}", p.y);

    let mut last_send: Option<Packet> = None;
    loop {
        let p = match network.run()? {
            NatState::Receive(_) => continue,
            NatState::Send(p) => p,
        };
        if last_send == Some(p) {
            println!("ans2={:?}", p.y);
            break;
        }
        last_send = Some(p);
    }

    Ok(())
}
