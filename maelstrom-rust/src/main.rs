mod node;
mod routes;

use crate::node::{MsgId, Node, NodeId};
use crate::routes::broadcast::proto::{
    MlstBodyReqBroadcast, MlstBodyReqRead, MlstBodyReqTopology, MlstBodyRespBroadcast,
    MlstBodyRespRead, MlstBodyRespTopology,
};
use crate::routes::broadcast::{MlstBroadcast, MsgType};
use crate::routes::echo::proto::{MlstBodyReqEcho, MlstBodyRespEcho};
use crate::routes::echo::MlstEcho;
use crate::routes::init::proto::{MlstBodyReqInit, MlstBodyRespInit};
use crate::routes::init::MlstInit;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::io;

fn main() -> io::Result<()> {
    let mut service = MlstService::new();
    let _ = service.main();
    Ok(())
}

struct MlstService {
    pub node_id: Option<NodeId>,
    pub neighbor_ids: Vec<NodeId>,
    pub messages: HashSet<MsgType>,
}

impl MlstService {
    pub fn new() -> Self {
        Self {
            node_id: None,
            neighbor_ids: Vec::new(),
            messages: HashSet::new(),
        }
    }
}

impl MlstInit for MlstService {}
impl MlstEcho for MlstService {}

impl MlstBroadcast for MlstService {
    fn set_neighbor_ids(&mut self, values: Vec<NodeId>) {
        self.neighbor_ids = values;
    }

    fn get_neighbor_ids(&self) -> &Vec<NodeId> {
        &self.neighbor_ids
    }

    fn store_message(&mut self, message: MsgType) {
        self.messages.insert(message);
    }

    fn check_message(&mut self, message: &MsgType) -> bool {
        self.messages.contains(message)
    }

    fn get_messages(&self) -> &HashSet<MsgType> {
        &self.messages
    }
}

impl Node for MlstService {
    type TMlstBodyBaseResp = MlstBodyBaseResp;
    type TMlstBodyBaseReq = MlstBodyBaseReq;

    fn dispatch_request(
        &mut self,
        msg_id: Option<MsgId>,
        src: NodeId,
        dest: NodeId,
        body: &Self::TMlstBodyBaseReq,
    ) {
        match body {
            MlstBodyBaseReq::Init(ref body) => self.process_init(msg_id, src, dest, body),
            MlstBodyBaseReq::Echo(ref body) => self.process_echo(msg_id, src, dest, body),
            MlstBodyBaseReq::Topology(ref body) => self.process_topology(msg_id, src, dest, body),
            MlstBodyBaseReq::Broadcast(ref body) => self.process_broadcast(msg_id, src, dest, body),
            MlstBodyBaseReq::Read(ref body) => self.process_read(msg_id, src, dest, body),
        }
    }

    fn get_node_id(&self) -> Option<&NodeId> {
        return self.node_id.as_ref();
    }

    fn set_node_id(&mut self, value: NodeId) {
        self.node_id = Some(value)
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum MlstBodyBaseResp {
    Init(MlstBodyRespInit),
    Echo(MlstBodyRespEcho),
    Topology(MlstBodyRespTopology),
    Broadcast(MlstBodyRespBroadcast),
    Read(MlstBodyRespRead),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum MlstBodyBaseReq {
    Init(MlstBodyReqInit),
    Echo(MlstBodyReqEcho),
    Topology(MlstBodyReqTopology),
    Broadcast(MlstBodyReqBroadcast),
    Read(MlstBodyReqRead),
}
