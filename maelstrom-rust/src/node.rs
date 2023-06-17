use proto::{MlstBodyResp, MlstReq, MlstResp};
use std::io::{self, Write};

pub type NodeId = String;
pub type MsgId = i64;

pub trait Node {
    type TMlstBodyBaseResp: Clone + serde::Serialize;
    type TMlstBodyBaseReq;

    fn get_node_id(&self) -> &NodeId;
    fn set_node_id(&mut self, value: NodeId);
    fn process_request(&mut self, buffer: String) -> io::Result<()>;

    fn main(&mut self) -> io::Result<()> {
        loop {
            let buffer = self.read();
            let _ = self.process_request(buffer);
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
        let with_newline = format!("{}\n", msg);
        let _ = io::stderr().write(&with_newline.into_bytes());
    }

    fn send(&self, dest_arg: NodeId, body_arg: MlstBodyResp<Self::TMlstBodyBaseResp>) {
        let msg = MlstResp {
            src: self.get_node_id().to_owned(),
            dest: dest_arg,
            body: body_arg,
        };
        let str_msg = serde_json::to_string(&msg).unwrap();
        self.log(&format!("Responded: {}", str_msg));
        self.write(&str_msg);
    }

    fn reply(
        &self,
        req_arg: MlstReq<Self::TMlstBodyBaseReq>,
        body_arg: MlstBodyResp<Self::TMlstBodyBaseResp>,
    ) {
        let mut body = body_arg.to_owned();
        body.in_reply_to = Some(req_arg.body.msg_id.to_owned());
        self.send(req_arg.src, body);
    }
}

pub mod proto {
    use crate::node::{MsgId, NodeId};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct MlstResp<TMlstBodyBaseResp> {
        pub src: NodeId,
        pub dest: NodeId,
        pub body: MlstBodyResp<TMlstBodyBaseResp>,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct MlstBodyResp<TMlstBodyBaseResp> {
        #[serde(flatten)]
        pub body: TMlstBodyBaseResp,
        pub msg_id: MsgId,
        pub in_reply_to: Option<MsgId>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct MlstBodyReq<TMlstBodyBaseReq> {
        #[serde(flatten)]
        pub body: TMlstBodyBaseReq,
        pub msg_id: MsgId,
    }

    #[derive(Serialize, Deserialize)]
    pub struct MlstReq<TMlstBodyBaseReq> {
        pub id: i64,
        pub src: NodeId,
        pub dest: NodeId,
        pub body: MlstBodyReq<TMlstBodyBaseReq>,
    }
}
