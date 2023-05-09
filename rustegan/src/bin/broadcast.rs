use rustegan::*;
use std::io::{StdoutLock, Write};
use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use anyhow::Context;


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
    Broadcast {
        message: usize,
    },
    BroadcastOk,
    Read,
    ReadOk { message: Vec<usize> },
    Topology {
        topology: HashMap<String, Vec<String>>,

    },
    TopologyOk,
}

struct BroadcastNode {
    node: String,
    id: usize,
    messages: Vec<usize>,
}

impl Node<(), Payload> for BroadcastNode {
    fn from_init(_state: (), init: Init) -> anyhow::Result<Self> {
        Ok(Self {
            node: init.node_id,
            id: 1,
            messages: Vec::new(),
        })
    }

    fn step(
        &mut self,
        input: Message<Payload>,
        output: &mut StdoutLock
    ) -> anyhow::Result<()> {
        match input.body.payload {
            Payload::Broadcast { message } => {
                let reply = input.into_reply();
                let reply = Message {
                    src: input.dst,
                    dst: input.src,
                    body: Body {
                        id: Some(self.id),
                        in_reply_to: input.body.id,
                        payload: Payload::BroadcastOk,
                    },
                };
                serde_json::to_writer(&mut *output, &reply).context("serialize response to echo")?;
                output.write_all(b"\n").context("write trailing newline")?;
                self.id += 1;

            }
            Payload::BroadcastOk { .. } => { }
            Payload::Read { .. } => { }
            Payload::ReadOk { .. } => { }
            Payload::Topology { .. } => {}
            Payload::TopologyOk { .. } => {}
        }
        Ok(())
    }
}


fn main() -> anyhow::Result<()> {
    main_loop::<_, BroadcastNode, _>(())
}
