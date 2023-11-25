use proto::MlstComm;
use proto::{MlstBodyResp, MlstBodyType, MlstReq};
use serde::Serialize;
use std::collections::HashSet;
use std::io::{self, Write};
use std::sync::Mutex;

pub type NodeId = String;
pub type MsgId = i64;
pub type CommId = i64;
pub type MsgType = i64;
pub type MsgTypeType = String;

pub trait Node {
    fn get_node_id(&self) -> &Mutex<Option<NodeId>>;
    fn set_node_id(&self, value: NodeId);
    fn next_msg_id(&self) -> MsgId;

    fn main(&self) {
        let buffer = self.read();
        self.log(&("buf read: ".to_string() + &buffer));
        let parsed: MlstReq = serde_json::from_str(&buffer).unwrap();
        let (comm_id, src, dest, body_req) = match parsed {
            MlstReq {
                id,
                src,
                dest,
                body: body_req,
            } => (id, src, dest, body_req),
        };
        let msg_type: MsgTypeType = body_req["type"].as_str().unwrap().to_string();
        self.dispatch_request(comm_id, msg_type, src, dest, body_req)
    }

    fn reply(&self, in_reply_to: MsgId, dest: NodeId, body: impl Serialize) {
        let msg_id = self.next_msg_id();
        let body_resp = MlstBodyResp {
            body,
            msg_id: Some(msg_id),
            in_reply_to,
        };
        self.communicate(dest, MlstBodyType::Resp(body_resp))
    }

    fn communicate(&self, dest: NodeId, body: MlstBodyType<impl Serialize>) {
        let msg = MlstComm {
            src: self.get_node_id().lock().unwrap().to_owned().unwrap(),
            dest,
            body,
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

    fn dispatch_request(
        &self,
        comm_id: Option<CommId>,
        msg_type: MsgTypeType,
        src: NodeId,
        dest: NodeId,
        body: serde_json::Value,
    );

    fn write(&self, msg: &str) {
        let with_newline = format!("{}\n", msg);
        io::stdout().write(&with_newline.into_bytes()).unwrap();
    }

    fn log(&self, msg: &str) {
        let undef = "UNDEF".to_string();
        let node_id = self
            .get_node_id()
            .lock()
            .unwrap()
            .to_owned()
            .unwrap_or(undef);
        let with_newline = format!("node {}: {}\n", node_id, msg);
        let _ = io::stderr().write(&with_newline.into_bytes());
    }

    fn set_neighbor_ids(&self, values: Vec<NodeId>);

    fn get_neighbor_ids(&self) -> &Mutex<Vec<NodeId>>;

    fn store_message(&self, message: MsgType);

    fn check_message(&self, message: &MsgType) -> bool;

    fn get_messages(&self) -> &Mutex<HashSet<MsgType>>;
}

pub mod proto {
    use crate::node::{CommId, MsgId, NodeId};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum MlstBodyType<T> {
        Resp(MlstBodyResp<T>),
        Comm(Box<serde_json::value::RawValue>),
    }

    #[derive(Serialize, Deserialize)]
    pub struct MlstComm<T> {
        pub src: NodeId,
        pub dest: NodeId,
        pub body: MlstBodyType<T>,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct MlstBodyComm {
        //#[serde(flatten)]
        //pub body: TMlstBodyBaseResp, // can't use flatten with RawValue
        pub msg_id: Option<MsgId>,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct MlstBodyResp<TMlstBodyBaseResp> {
        #[serde(flatten)]
        pub body: TMlstBodyBaseResp,
        pub msg_id: Option<MsgId>,
        pub in_reply_to: MsgId,
    }

    #[derive(Deserialize)]
    pub struct MlstAckBodyReq {
        pub msg_id: MsgId,
        pub in_reply_to: MsgId,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct MlstReq {
        pub id: Option<CommId>,
        pub src: NodeId,
        pub dest: NodeId,
        pub body: serde_json::Value,
    }
}
