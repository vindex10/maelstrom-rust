use crate::node::{CommId, MsgTypeType, Node, NodeId};
use proto::{MlstBodyReqTopology, MlstBodyRespTopology};

pub trait MlstTopology: Node {
    fn process_topology(
        &self,
        _comm_id: Option<CommId>,
        src: NodeId,
        _dest: NodeId,
        body_req: serde_json::Value,
    ) {
        self.log("TOPOLOGY");
        let req_body: MlstBodyReqTopology = serde_json::from_value(body_req).unwrap();
        let node_id_lock = self.get_node_id().lock();
        let topology = req_body.topology[node_id_lock.unwrap().as_ref().unwrap()].to_owned();
        self.set_neighbor_ids(topology);
        let resp_body = MlstBodyRespTopology {
            msg_type: "topology_ok".to_string(),
        };
        self.reply(req_body.msg_id, src, resp_body);
    }

    fn get_route_topology() -> MsgTypeType;
}

pub mod proto {
    use crate::node::{MsgId, MsgTypeType, NodeId};
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Serialize, Deserialize)]
    pub struct MlstBodyReqTopology {
        pub msg_id: MsgId,
        #[serde(rename = "type")]
        pub msg_type: MsgTypeType,
        pub topology: HashMap<NodeId, Vec<NodeId>>,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct MlstBodyRespTopology {
        #[serde(rename = "type")]
        pub msg_type: MsgTypeType,
    }
}
