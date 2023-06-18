use crate::node::{MsgId, Node, NodeId};
use std::collections::HashSet;
use proto::{
    MlstBodyReqBroadcast, MlstBodyReqRead, MlstBodyReqTopology, MlstBodyRespBroadcast,
    MlstBodyRespRead, MlstBodyRespTopology,
};

pub type MsgType = i32;

pub trait MlstBroadcast: Node {
    fn set_neighbor_ids(&mut self, values: Vec<NodeId>);
    fn get_neighbor_ids(&self) -> &Vec<NodeId>;
    fn store_message(&mut self, message: MsgType);
    fn check_message(&mut self, message: &MsgType) -> bool;
    fn get_messages(&self) -> &HashSet<MsgType>;

    fn process_topology(
        &mut self,
        msg_id: Option<MsgId>,
        src: NodeId,
        _dest: NodeId,
        req_body: &MlstBodyReqTopology,
    ) {
        self.log("TOPOLOGY");
        let node_id = self.get_node_id().unwrap();
        self.set_neighbor_ids(req_body.topology[node_id].to_owned());
        let resp_body = MlstBodyRespTopology {
            msg_type: "topology_ok".to_string(),
        };
        self.reply(msg_id.unwrap(), src, resp_body);
    }

    fn process_broadcast(
        &mut self,
        msg_id: Option<MsgId>,
        src: NodeId,
        _dest: NodeId,
        req_body: &MlstBodyReqBroadcast,
    ) {
        self.log("BROADCAST");
        let msg = req_body.message.to_owned();
        if self.check_message(&msg) {
            return;
        }
        self.store_message(msg);
        for neighbor_id in self.get_neighbor_ids().iter() {
            self.communicate(neighbor_id.to_owned(), req_body)
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

    fn process_read(
        &self,
        msg_id: Option<MsgId>,
        src: NodeId,
        _dest: NodeId,
        _req_body: &MlstBodyReqRead,
    ) {
        self.log("READ");
        let resp_body = MlstBodyRespRead {
            msg_type: "read_ok".to_string(),
            messages: self.get_messages().to_owned().into_iter().collect(),
        };
        self.reply(msg_id.unwrap(), src, resp_body);
    }
}

pub mod proto {
    use crate::node::NodeId;
    use crate::routes::broadcast::MsgType;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct MlstBodyReqTopology {
        #[serde(rename = "type")]
        pub msg_type: String,
        pub topology: HashMap<NodeId, Vec<NodeId>>,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct MlstBodyRespTopology {
        #[serde(rename = "type")]
        pub msg_type: String,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct MlstBodyReqBroadcast {
        #[serde(rename = "type")]
        pub msg_type: String,
        pub message: MsgType,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct MlstBodyRespBroadcast {
        #[serde(rename = "type")]
        pub msg_type: String,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct MlstBodyReqRead {
        #[serde(rename = "type")]
        pub msg_type: String,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct MlstBodyRespRead {
        #[serde(rename = "type")]
        pub msg_type: String,
        pub messages: Vec<MsgType>,
    }
}
