use serde::{Deserialize, Serialize};
use std::io::{self, Write};

#[derive(Serialize, Deserialize)]
struct MlstBaseResp {
    src: String,
    dest: String,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct MlstBodyReqInit {
    msg_id: i64,
    #[serde(rename = "type")]
    msg_type: String,
    node_id: String,
    node_ids: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct MlstBodyRespInit {
    msg_id: i64,
    #[serde(rename = "type")]
    msg_type: String,
    in_reply_to: i64,
}

#[derive(Serialize, Deserialize)]
struct MlstRespInit {
    #[serde(flatten)]
    base: MlstBaseResp,
    body: MlstBodyRespInit,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct MlstBodyReqEcho {
    msg_id: i64,
    #[serde(rename = "type")]
    msg_type: String,
    echo: String,
}

#[derive(Serialize, Deserialize)]
struct MlstBodyRespEcho {
    msg_id: i64,
    #[serde(rename = "type")]
    msg_type: String,
    echo: String,
    in_reply_to: i64,
}

#[derive(Serialize, Deserialize)]
struct MlstRespEcho {
    #[serde(flatten)]
    base: MlstBaseResp,
    body: MlstBodyRespEcho,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum MlstBodyReq {
    Init(MlstBodyReqInit),
    Echo(MlstBodyReqEcho),
}

#[derive(Serialize, Deserialize)]
struct MlstReq {
    id: i64,
    src: String,
    dest: String,
    body: MlstBodyReq,
}

fn main() {
    loop {
        let _ = process_request();
    }
}

fn process_request() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    io::stderr().write_fmt(format_args!("Received: {0}\n", buffer))?;
    let request: MlstReq = serde_json::from_str(&buffer)?;
    let response: String = match request.body {
        MlstBodyReq::Init(ref req_body) => process_init(&request, req_body),
        MlstBodyReq::Echo(ref req_body) => process_echo(&request, req_body),
    };
    io::stderr().write_fmt(format_args!("Responded: {0}\n", response))?;
    io::stdout().write_fmt(format_args!("{0}\n", response))?;
    Ok(())
}

fn process_init(request: &MlstReq, req_body: &MlstBodyReqInit) -> String {
    io::stderr().write(b"INIT\n").unwrap();
    let resp_body = MlstBodyRespInit {
        msg_id: 1,
        msg_type: "init_ok".to_string(),
        in_reply_to: req_body.msg_id,
    };
    let resp_base = MlstBaseResp {
        src: request.dest.clone(),
        dest: request.src.clone(),
    };
    let resp = MlstRespInit {
        base: resp_base,
        body: resp_body,
    };
    serde_json::to_string(&resp).unwrap()
}

fn process_echo(request: &MlstReq, req_body: &MlstBodyReqEcho) -> String {
    io::stderr().write(b"ECHO\n").unwrap();
    let resp_body = MlstBodyRespEcho {
        msg_id: 1,
        msg_type: "echo_ok".to_string(),
        in_reply_to: req_body.msg_id,
        echo: req_body.echo.clone(),
    };
    let resp_base = MlstBaseResp {
        src: request.dest.clone(),
        dest: request.src.clone(),
    };
    let resp = MlstRespEcho {
        base: resp_base,
        body: resp_body,
    };
    serde_json::to_string(&resp).unwrap()
}
