use crate::node::{Node, NodeId};
use proto::{MlstBodyReqTopology, MlstBodyRespTopology};

pub trait MlstBroadcast: Node {
    fn set_neighbor_ids(&mut self, values: Vec<NodeId>);

    fn process_topology(&mut self, req_body: &MlstBodyReqTopology) -> MlstBodyRespTopology {
        self.log("TOPOLOGY");
        let node_id = self.get_node_id();
        self.set_neighbor_ids(req_body.topology[node_id].to_owned());
        let resp_body = MlstBodyRespTopology {
            msg_type: "topology_ok".to_string(),
        };
        resp_body
    }
}

pub mod proto {
    use crate::node::NodeId;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct MlstBodyReqTopology {
        #[serde(rename = "type")]
        pub msg_type: String,
        pub topology: HashMap<String, Vec<NodeId>>,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct MlstBodyRespTopology {
        #[serde(rename = "type")]
        pub msg_type: String,
    }
}
