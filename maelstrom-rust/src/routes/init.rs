use crate::node::{MsgId, MsgTypeType, Node, NodeId};
use proto::{MlstBodyReqInit, MlstBodyRespInit};

pub trait MlstInit: Node {
    fn process_init(
        &mut self,
        msg_id: Option<MsgId>,
        src: NodeId,
        _dest: NodeId,
        body_req: serde_json::Value,
    ) {
        self.log("INIT");
        let req_body: MlstBodyReqInit = serde_json::from_value(body_req).unwrap();
        self.set_node_id(req_body.node_id.to_owned());
        let resp_body = MlstBodyRespInit {
            msg_type: "init_ok".to_string(),
        };
        self.reply(msg_id.unwrap(), src, resp_body);
    }

    fn get_route_init() -> MsgTypeType;
}

pub mod proto {
    use crate::node::{MsgTypeType, NodeId};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct MlstBodyReqInit {
        pub node_id: NodeId,
        pub node_ids: Vec<String>,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct MlstBodyRespInit {
        #[serde(rename = "type")]
        pub msg_type: MsgTypeType,
    }
}
