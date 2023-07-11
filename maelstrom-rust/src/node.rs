use proto::{MlstBodyComm, MlstComm};
use proto::{MlstBodyResp, MlstReq, MlstResp};
use serde::Serialize;
use std::collections::HashSet;
use std::io::{self, Write};

pub type NodeId = String;
pub type MsgId = i64;
pub type MsgType = i64;
pub type MsgTypeType = String;

pub trait Node {
    type TMlstBodyBaseResp: Clone + serde::Serialize;
    type TMlstBodyBaseReq: serde::de::DeserializeOwned;

    fn get_node_id(&self) -> Option<&NodeId>;
    fn set_node_id(&mut self, value: NodeId);

    fn main(&mut self) {
        loop {
            let buffer = self.read();
            self.log(&("buf read: ".to_string() + &buffer));
            let parsed: MlstReq = serde_json::from_str(&buffer).unwrap();
            let (src, dest, body_req) = match parsed {
                MlstReq {
                    src,
                    dest,
                    body: body_req,
                    ..
                } => (src, dest, body_req),
            };
            let msg_id: Option<MsgId> = body_req["msg_id"].as_i64();
            let msg_type: MsgTypeType = body_req["type"].as_str().unwrap().to_string();
            self.dispatch_request(msg_id, msg_type, src, dest, body_req);
        }
    }

    fn dispatch_request(
        &mut self,
        msg_id: Option<MsgId>,
        msg_type: MsgTypeType,
        src: NodeId,
        dest: NodeId,
        body: serde_json::Value,
    );

    fn communicate(&self, dest: NodeId, body: impl Serialize) {
        let body_resp = MlstBodyComm { body, msg_id: None };
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

    fn set_neighbor_ids(&mut self, values: Vec<NodeId>);

    fn get_neighbor_ids(&self) -> &Vec<NodeId>;

    fn store_message(&mut self, message: MsgType);

    fn check_message(&mut self, message: &MsgType) -> bool;

    fn get_messages(&self) -> &HashSet<MsgType>;
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

    // cant use flattened raw values until issue fixed: https://github.com/serde-rs/json/issues/599
    //
    //#[derive(Serialize, Deserialize, Clone)]
    //pub struct MlstBodyReq {
    //#[serde(flatten)]
    //pub body: Box<serde_json::value::RawValue>,
    //pub msg_id: Option<MsgId>,
    //#[serde(rename = "type")]
    //pub msg_type: MsgTypeType,
    //}

    #[derive(Serialize, Deserialize, Clone)]
    pub struct MlstReq {
        pub id: i64,
        pub src: NodeId,
        pub dest: NodeId,
        //pub body: MlstBodyReq,
        pub body: serde_json::Value,
    }
}
