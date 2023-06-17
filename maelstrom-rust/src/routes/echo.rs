use crate::node::Node;
use proto::{MlstBodyReqEcho, MlstBodyRespEcho};

pub trait MlstEcho: Node {
    fn process_echo(&self, req_body: &MlstBodyReqEcho) -> MlstBodyRespEcho {
        self.log("ECHO");
        let resp_body = MlstBodyRespEcho {
            msg_type: "echo_ok".to_string(),
            echo: req_body.echo.clone(),
        };
        resp_body
    }
}

pub mod proto {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct MlstBodyReqEcho {
        #[serde(rename = "type")]
        pub msg_type: String,
        pub echo: String,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct MlstBodyRespEcho {
        #[serde(rename = "type")]
        pub msg_type: String,
        pub echo: String,
    }
}
