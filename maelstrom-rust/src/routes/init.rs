use crate::node::Node;
use proto::{MlstBodyReqInit, MlstBodyRespInit};

pub trait MlstInit: Node {
    fn process_init(&mut self, req_body: &MlstBodyReqInit) -> MlstBodyRespInit {
        self.log("INIT");
        self.set_node_id(req_body.node_id.to_owned());
        let resp_body = MlstBodyRespInit {
            msg_type: "init_ok".to_string(),
        };
        resp_body
    }
}

pub mod proto {
    use crate::node::NodeId;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct MlstBodyReqInit {
        #[serde(rename = "type")]
        pub msg_type: String,
        pub node_id: NodeId,
        pub node_ids: Vec<String>,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct MlstBodyRespInit {
        #[serde(rename = "type")]
        pub msg_type: String,
    }
}
