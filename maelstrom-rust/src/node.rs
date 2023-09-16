use proto::{MlstBodyComm, MlstComm};
use proto::{MlstBodyResp, MlstReq, MlstResp};
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::io::{self, Write};
use std::sync::Mutex;

pub type NodeId = String;
pub type MsgId = i64;
pub type CommId = i64;
pub type MsgType = i64;
pub type MsgTypeType = String;

fn dummy_concat_jsons(left: String, right: String) -> String {
    let left_len = left.len();
    let mut res = left[0..&left_len - 1].to_owned();
    res.push_str(",");
    res.push_str(&right[1..]);
    res
}

#[derive(Clone)]
pub struct MsgCached {
    pub msg_str: String,
}

#[derive(Hash, Eq, PartialEq)]
pub struct MsgCachedKey {
    pub msg_id: MsgId,
    pub dest: NodeId,
}

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

    fn dispatch_request(
        &self,
        comm_id: Option<CommId>,
        msg_type: MsgTypeType,
        src: NodeId,
        dest: NodeId,
        body: serde_json::Value,
    );

    fn communicate(&self, msg_id: Option<MsgId>, dest: NodeId, body: impl Serialize) {
        let body_resp_tpl_str = serde_json::to_string(&MlstBodyComm { msg_id }).unwrap();
        let body_str = serde_json::to_string(&body).unwrap();
        let body_resp_str = dummy_concat_jsons(body_resp_tpl_str, body_str);
        let msg = MlstComm {
            src: self.get_node_id().lock().unwrap().to_owned().unwrap(),
            dest,
            body: serde_json::value::RawValue::from_string(body_resp_str).unwrap(),
        };
        let str_msg = serde_json::to_string(&msg).unwrap();
        self.log(&format!("Responded: {}", str_msg));
        self.write(&str_msg);
    }

    fn await_communicate(&self, msg_id: MsgId, dest: NodeId, msg: impl serde::Serialize) {
        let msg_cached = MsgCached {
            msg_str: serde_json::to_string(&msg).unwrap(),
        };
        let key = MsgCachedKey { msg_id, dest };
        self.ack_await(key, msg_cached);
    }

    fn reply(&self, in_reply_to: MsgId, dest: NodeId, body: impl Serialize) {
        let msg_id = self.next_msg_id();
        let body_resp = MlstBodyResp {
            body,
            msg_id: Some(msg_id),
            in_reply_to,
        };
        let msg = MlstResp {
            src: self.get_node_id().lock().unwrap().to_owned().unwrap(),
            dest,
            body: body_resp,
        };
        let str_msg = serde_json::to_string(&msg).unwrap();
        self.log(&format!("Responded: {}", str_msg));
        self.write(&str_msg);
    }

    fn repeat_unacked(&self) {
        let unacked = self.get_pending_ack_ids().lock().unwrap();
        for (key, msg_cached) in &*unacked {
            //let ref mut copy_msg_cached = msg_cached.clone();
            //let dest = std::mem::take(&mut copy_msg_cached.dest);
            let dest = key.dest.to_owned();
            let some_msg_id = Some(key.msg_id.to_owned());
            let raw_val =
                serde_json::value::RawValue::from_string(msg_cached.msg_str.to_owned()).unwrap();
            self.communicate(some_msg_id, dest, raw_val);
        }
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

    fn get_pending_ack_ids(&self) -> &Mutex<HashMap<MsgCachedKey, MsgCached>>;

    fn ack_await(&self, key: MsgCachedKey, msg_cached: MsgCached);

    fn ack_delivered(&self, key: &MsgCachedKey);
}

pub mod proto {
    use crate::node::{CommId, MsgId, NodeId};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct MlstComm {
        pub src: NodeId,
        pub dest: NodeId,
        pub body: Box<serde_json::value::RawValue>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct MlstResp<TMlstBodyBaseResp> {
        pub src: NodeId,
        pub dest: NodeId,
        pub body: MlstBodyResp<TMlstBodyBaseResp>,
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
