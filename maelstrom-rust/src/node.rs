use crate::proto::{
    MlstBodyBaseReq, MlstBodyBaseResp, MlstBodyReq, MlstBodyReqEcho, MlstBodyReqInit, MlstBodyResp,
    MlstBodyRespEcho, MlstBodyRespInit, MlstReq, MlstResp,
};
use crate::types::NodeId;
use std::io::{self, Write};

pub struct Node {
    pub node_id: Option<NodeId>,
    pub node_ids: Vec<String>,
}

impl Node {
    pub fn new() -> Self {
        Self {
            node_id: None,
            node_ids: Vec::new(),
        }
    }

    pub fn main(&mut self) -> io::Result<()> {
        loop {
            let buffer = self.read();
            let _ = self.process_request(buffer);
        }
    }

    fn process_request(&mut self, buffer: String) -> io::Result<()> {
        self.log(&format!("Received: {0}", &buffer));
        let request: MlstReq = serde_json::from_str(&buffer)?;
        let response_body: MlstBodyResp = match request.body {
            MlstBodyReq {
                body: MlstBodyBaseReq::Init(ref req_body),
                ..
            } => self.process_init(req_body),
            MlstBodyReq {
                body: MlstBodyBaseReq::Echo(ref req_body),
                ..
            } => self.process_echo(req_body),
        };
        self.reply(request, response_body);
        Ok(())
    }

    fn process_init(&mut self, req_body: &MlstBodyReqInit) -> MlstBodyResp {
        self.log("INIT");
        self.node_id = Some(req_body.node_id.to_owned());
        let resp_body = MlstBodyRespInit {
            msg_type: "init_ok".to_string(),
        };
        MlstBodyResp {
            body: MlstBodyBaseResp::Init(resp_body),
            msg_id: 1,
            in_reply_to: None,
        }
    }

    fn process_echo(&self, req_body: &MlstBodyReqEcho) -> MlstBodyResp {
        self.log("ECHO");
        let resp_body = MlstBodyRespEcho {
            msg_type: "echo_ok".to_string(),
            echo: req_body.echo.clone(),
        };
        MlstBodyResp {
            body: MlstBodyBaseResp::Echo(resp_body),
            msg_id: 1,
            in_reply_to: None,
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

    fn send(&self, dest_arg: NodeId, body_arg: MlstBodyResp) {
        let msg = MlstResp {
            src: self.node_id.to_owned().unwrap(),
            dest: dest_arg,
            body: body_arg,
        };
        let str_msg = serde_json::to_string(&msg).unwrap();
        self.log(&format!("Responded: {}", str_msg));
        self.write(&str_msg);
    }

    fn reply(&self, req_arg: MlstReq, body_arg: MlstBodyResp) {
        let mut body = body_arg.to_owned();
        body.in_reply_to = Some(req_arg.body.msg_id.to_owned());
        self.send(req_arg.src, body);
    }
}
