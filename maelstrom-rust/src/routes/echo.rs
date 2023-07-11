use crate::node::{MsgId, MsgTypeType, Node, NodeId};
use proto::{MlstBodyReqEcho, MlstBodyRespEcho};

pub trait MlstEcho: Node {
    fn process_echo(
        &self,
        msg_id: Option<MsgId>,
        src: NodeId,
        _dest: NodeId,
        body_req: serde_json::Value,
    ) {
        self.log("ECHO");
        let req_body: MlstBodyReqEcho = serde_json::from_value(body_req).unwrap();
        let resp_body = MlstBodyRespEcho {
            msg_type: "echo_ok".to_string(),
            echo: req_body.echo.clone(),
        };
        self.reply(msg_id.unwrap(), src, resp_body);
    }

    fn get_route_echo() -> MsgTypeType;
}

pub mod proto {
    use crate::node::MsgTypeType;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct MlstBodyReqEcho {
        #[serde(rename = "type")]
        pub msg_type: MsgTypeType,
        pub echo: String,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct MlstBodyRespEcho {
        #[serde(rename = "type")]
        pub msg_type: MsgTypeType,
        pub echo: String,
    }
}
