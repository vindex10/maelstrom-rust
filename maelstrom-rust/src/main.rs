mod node;
mod routes;

use crate::node::proto::{MlstBodyReq, MlstBodyResp, MlstReq};
use crate::node::{Node, NodeId};
use crate::routes::broadcast::proto::{MlstBodyReqTopology, MlstBodyRespTopology};
use crate::routes::broadcast::MlstBroadcast;
use crate::routes::echo::proto::{MlstBodyReqEcho, MlstBodyRespEcho};
use crate::routes::echo::MlstEcho;
use crate::routes::init::proto::{MlstBodyReqInit, MlstBodyRespInit};
use crate::routes::init::MlstInit;
use serde::{Deserialize, Serialize};
use std::io;

fn main() -> io::Result<()> {
    let mut service = MlstService::new();
    let _ = service.main();
    Ok(())
}

struct MlstService {
    pub node_id: Option<NodeId>,
    pub neighbor_ids: Vec<NodeId>,
}

impl MlstService {
    pub fn new() -> Self {
        Self {
            node_id: None,
            neighbor_ids: Vec::new(),
        }
    }
}

impl MlstInit for MlstService {}
impl MlstEcho for MlstService {}

impl MlstBroadcast for MlstService {
    fn set_neighbor_ids(&mut self, values: Vec<NodeId>) {
        self.neighbor_ids = values;
    }
}

impl Node for MlstService {
    type TMlstBodyBaseResp = MlstBodyBaseResp;
    type TMlstBodyBaseReq = MlstBodyBaseReq;

    fn get_node_id(&self) -> &NodeId {
        match self.node_id {
            Some(ref v) => v,
            None => panic!("node id is not defined"),
        }
    }

    fn set_node_id(&mut self, value: NodeId) {
        self.node_id = Some(value)
    }

    fn process_request(&mut self, buffer: String) -> io::Result<()> {
        self.log(&format!("Received: {0}", &buffer));
        let request: MlstReq<Self::TMlstBodyBaseReq> = serde_json::from_str(&buffer)?;
        let response_body_base: Self::TMlstBodyBaseResp = match request.body {
            MlstBodyReq {
                body: MlstBodyBaseReq::Init(ref req_body),
                ..
            } => MlstBodyBaseResp::Init(self.process_init(req_body)),
            MlstBodyReq {
                body: MlstBodyBaseReq::Echo(ref req_body),
                ..
            } => MlstBodyBaseResp::Echo(self.process_echo(req_body)),
            MlstBodyReq {
                body: MlstBodyBaseReq::Topology(ref req_body),
                ..
            } => MlstBodyBaseResp::Topology(self.process_topology(req_body)),
        };
        let response_body = MlstBodyResp {
            body: response_body_base,
            msg_id: 1,
            in_reply_to: None,
        };
        self.reply(request, response_body);
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum MlstBodyBaseResp {
    Init(MlstBodyRespInit),
    Echo(MlstBodyRespEcho),
    Topology(MlstBodyRespTopology),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum MlstBodyBaseReq {
    Init(MlstBodyReqInit),
    Echo(MlstBodyReqEcho),
    Topology(MlstBodyReqTopology),
}
