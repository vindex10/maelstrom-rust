use crate::node::{MsgId, MsgTypeType, Node, NodeId};
use proto::{MlstBodyReqRead, MlstBodyRespRead};

pub trait MlstRead: Node {
    fn process_read(
        &self,
        _msg_id: Option<MsgId>,
        src: NodeId,
        _dest: NodeId,
        req_body_raw: serde_json::Value,
    ) {
        self.log("READ");
        let req_body: MlstBodyReqRead = serde_json::from_value(req_body_raw).unwrap();
        let resp_body = MlstBodyRespRead {
            msg_type: "read_ok".to_string(),
            messages: self
                .get_messages()
                .lock()
                .unwrap()
                .to_owned()
                .into_iter()
                .collect(),
        };
        self.reply(req_body.msg_id, src, resp_body);
    }

    fn get_route_read() -> MsgTypeType;
}

pub mod proto {
    use crate::node::{MsgId, MsgType, MsgTypeType};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct MlstBodyReqRead {
        pub msg_id: MsgId,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct MlstBodyRespRead {
        #[serde(rename = "type")]
        pub msg_type: MsgTypeType,
        pub messages: Vec<MsgType>,
    }
}
