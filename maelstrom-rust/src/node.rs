use proto::{MlstBodyReq, MlstBodyResp, MlstReq, MlstResp};
use proto::{MlstBodyComm, MlstComm};
use serde::Serialize;
use std::io::{self, Write};

pub type NodeId = String;
pub type MsgId = i64;

pub trait Node {
    type TMlstBodyBaseResp: Clone + serde::Serialize;
    type TMlstBodyBaseReq: serde::de::DeserializeOwned;

    fn get_node_id(&self) -> Option<&NodeId>;
    fn set_node_id(&mut self, value: NodeId);

    fn main(&mut self) -> io::Result<()> {
        loop {
            let buffer = self.read();
            self.log(&("buf read: ".to_string() + &buffer));
            let parsed = serde_json::from_str(&buffer)?;
            let (msg_id, src, dest, body) = match parsed {
                MlstReq {
                    src,
                    dest,
                    body:
                        MlstBodyReq {
                            body: ref body_arg,
                            msg_id,
                        },
                    ..
                } => (msg_id, src, dest, body_arg),
            };
            self.dispatch_request(msg_id, src, dest, body);
        }
    }

    fn dispatch_request(
        &mut self,
        msg_id: Option<MsgId>,
        src: NodeId,
        dest: NodeId,
        body: &Self::TMlstBodyBaseReq,
    );

    fn communicate(&self, dest: NodeId, body: impl Serialize) {
        let body_resp = MlstBodyComm {
            body,
            msg_id: None,
        };
        let msg = MlstComm {
            src: self.get_node_id().unwrap().to_owned(),
            dest,
            body: body_resp,
        };
        let str_msg = serde_json::to_string(&msg).unwrap();
        self.log(&format!("Responded: {}", str_msg));
        self.write(&str_msg);
    }

    fn reply(&self, in_reply_to: MsgId, dest: NodeId, body: impl Serialize) {
        let body_resp = MlstBodyResp {
            body,
            msg_id: Some(1),
            in_reply_to,
        };
        let msg = MlstResp {
            src: self.get_node_id().unwrap().to_owned(),
            dest,
            body: body_resp,
        };
        let str_msg = serde_json::to_string(&msg).unwrap();
        self.log(&format!("Responded: {}", str_msg));
        self.write(&str_msg);
    }

    fn read(&self) -> String {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        let trimlen = buffer.trim_end().len();
        buffer.truncate(trimlen);
        buffer
    }

    fn write(&self, msg: &str) {
        let with_newline = format!("{}\n", msg);
        io::stdout().write(&with_newline.into_bytes()).unwrap();
    }

    fn log(&self, msg: &str) {
        let undef = "UNDEF".to_string();
        let node_id = self.get_node_id().unwrap_or(&undef);
        let with_newline = format!("node {}: {}\n", node_id, msg);
        let _ = io::stderr().write(&with_newline.into_bytes());
    }
}

pub mod proto {
    use crate::node::{MsgId, NodeId};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct MlstComm<TMlstBodyBaseResp> {
        pub src: NodeId,
        pub dest: NodeId,
        pub body: MlstBodyComm<TMlstBodyBaseResp>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct MlstResp<TMlstBodyBaseResp> {
        pub src: NodeId,
        pub dest: NodeId,
        pub body: MlstBodyResp<TMlstBodyBaseResp>,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct MlstBodyComm<TMlstBodyBaseResp> {
        #[serde(flatten)]
        pub body: TMlstBodyBaseResp,
        pub msg_id: Option<MsgId>,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct MlstBodyResp<TMlstBodyBaseResp> {
        #[serde(flatten)]
        pub body: TMlstBodyBaseResp,
        pub msg_id: Option<MsgId>,
        pub in_reply_to: MsgId,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct MlstBodyReq<TMlstBodyBaseReq> {
        #[serde(flatten)]
        pub body: TMlstBodyBaseReq,
        pub msg_id: Option<MsgId>,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct MlstReq<TMlstBodyBaseReq> {
        pub id: i64,
        pub src: NodeId,
        pub dest: NodeId,
        pub body: MlstBodyReq<TMlstBodyBaseReq>,
    }
}
