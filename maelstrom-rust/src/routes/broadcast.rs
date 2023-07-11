use crate::node::{MsgId, MsgTypeType, Node, NodeId};
use proto::{
    MlstBodyReqBroadcast, MlstBodyReqTopology, MlstBodyRespBroadcast, MlstBodyRespRead,
    MlstBodyRespTopology,
};

pub trait MlstBroadcast: Node {
    fn process_topology(
        &mut self,
        msg_id: Option<MsgId>,
        src: NodeId,
        _dest: NodeId,
        body_req: serde_json::Value,
    ) {
        self.log("TOPOLOGY");
        let req_body: MlstBodyReqTopology = serde_json::from_value(body_req).unwrap();
        let node_id = self.get_node_id().unwrap();
        self.set_neighbor_ids(req_body.topology[node_id].to_owned());
        let resp_body = MlstBodyRespTopology {
            msg_type: "topology_ok".to_string(),
        };
        self.reply(msg_id.unwrap(), src, resp_body);
    }

    fn get_route_topology() -> MsgTypeType;

    fn process_broadcast(
        &mut self,
        msg_id: Option<MsgId>,
        src: NodeId,
        _dest: NodeId,
        body_req: serde_json::Value,
    ) {
        self.log("BROADCAST");
        let req_body: MlstBodyReqBroadcast = serde_json::from_value(body_req).unwrap();
        let msg = req_body.message.to_owned();
        if self.check_message(&msg) {
            return;
        }
        self.store_message(msg);
        for neighbor_id in self.get_neighbor_ids().iter() {
            if neighbor_id == &src {
                continue;
            };
            self.communicate(neighbor_id.to_owned(), &req_body)
        }
        if msg_id.is_none() {
            self.log("do not reply");
            return;
        }
        self.log(&format!("msg_id: {}", msg_id.as_ref().unwrap()));
        let resp_body = MlstBodyRespBroadcast {
            msg_type: "broadcast_ok".to_string(),
        };
        self.reply(msg_id.unwrap(), src, resp_body);
    }

    fn get_route_broadcast() -> MsgTypeType;

    fn process_read(
        &self,
        msg_id: Option<MsgId>,
        src: NodeId,
        _dest: NodeId,
        _req_body_raw: serde_json::Value,
    ) {
        self.log("READ");
        let resp_body = MlstBodyRespRead {
            msg_type: "read_ok".to_string(),
            messages: self.get_messages().to_owned().into_iter().collect(),
        };
        self.reply(msg_id.unwrap(), src, resp_body);
    }

    fn get_route_read() -> MsgTypeType;
}

pub mod proto {
    use crate::node::{MsgType, MsgTypeType, NodeId};
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Serialize, Deserialize)]
    pub struct MlstBodyReqTopology {
        #[serde(rename = "type")]
        pub msg_type: MsgTypeType,
        pub topology: HashMap<NodeId, Vec<NodeId>>,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct MlstBodyRespTopology {
        #[serde(rename = "type")]
        pub msg_type: MsgTypeType,
    }

    #[derive(Serialize, Deserialize)]
    pub struct MlstBodyReqBroadcast {
        #[serde(rename = "type")]
        pub msg_type: MsgTypeType,
        pub message: MsgType,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct MlstBodyRespBroadcast {
        #[serde(rename = "type")]
        pub msg_type: MsgTypeType,
    }

    #[derive(Serialize, Deserialize)]
    pub struct MlstBodyReqRead {}

    #[derive(Serialize, Deserialize, Clone)]
    pub struct MlstBodyRespRead {
        #[serde(rename = "type")]
        pub msg_type: MsgTypeType,
        pub messages: Vec<MsgType>,
    }
}
