mod node;
mod routes;

use crate::node::{MsgId, MsgType, MsgTypeType, Node, NodeId};
use crate::routes::broadcast::proto::{
    MlstBodyReqBroadcast, MlstBodyReqRead, MlstBodyReqTopology, MlstBodyRespBroadcast,
    MlstBodyRespRead, MlstBodyRespTopology,
};
use crate::routes::broadcast::MlstBroadcast;
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

impl MlstInit for MlstService {
    fn get_route_init() -> MsgTypeType {
        return "init".to_string();
    }
}
impl MlstEcho for MlstService {
    fn get_route_echo() -> MsgTypeType {
        return "echo".to_string();
    }
}
impl MlstBroadcast for MlstService {
    fn get_route_topology() -> MsgTypeType {
        return "topology".to_string();
    }

    fn get_route_broadcast() -> MsgTypeType {
        return "broadcast".to_string();
    }

    fn get_route_read() -> MsgTypeType {
        return "read".to_string();
    }
}

impl Node for MlstService {
    type TMlstBodyBaseResp = MlstBodyBaseResp;
    type TMlstBodyBaseReq = MlstBodyBaseReq;

    fn dispatch_request(
        &mut self,
        msg_id: Option<MsgId>,
        msg_type: MsgTypeType,
        src: NodeId,
        dest: NodeId,
        body_req: serde_json::Value,
    ) {
        let msg_type_str = msg_type.as_str();
        match msg_type_str {
            "init" => self.process_init(msg_id, src, dest, body_req),
            "echo" => self.process_echo(msg_id, src, dest, body_req),
            "topology" => self.process_topology(msg_id, src, dest, body_req),
            "broadcast" => self.process_broadcast(msg_id, src, dest, body_req),
            "read" => self.process_read(msg_id, src, dest, body_req),
            _ => panic!("Unmatched message type"),
        }
    }

    fn get_node_id(&self) -> Option<&NodeId> {
        return self.node_id.as_ref();
    }

    fn set_node_id(&mut self, value: NodeId) {
        self.node_id = Some(value)
    }

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
