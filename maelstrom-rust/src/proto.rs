use crate::types::{MsgId, NodeId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct MlstResp {
    pub src: NodeId,
    pub dest: NodeId,
    pub body: MlstBodyResp,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MlstBodyReqInit {
    #[serde(rename = "type")]
    pub msg_type: String,
    pub node_id: NodeId,
    pub node_ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MlstBodyRespInit {
    #[serde(rename = "type")]
    pub msg_type: String,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MlstBodyReqEcho {
    #[serde(rename = "type")]
    pub msg_type: String,
    pub echo: String,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MlstBodyReqTopology {
    #[serde(rename = "type")]
    pub msg_type: String,
    pub topology: HashMap<String, Vec<NodeId>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MlstBodyRespEcho {
    #[serde(rename = "type")]
    pub msg_type: String,
    pub echo: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MlstBodyRespTopology {
    #[serde(rename = "type")]
    pub msg_type: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum MlstBodyBaseResp {
    Init(MlstBodyRespInit),
    Echo(MlstBodyRespEcho),
    Topology(MlstBodyRespTopology),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MlstBodyResp {
    #[serde(flatten)]
    pub body: MlstBodyBaseResp,
    pub msg_id: MsgId,
    pub in_reply_to: Option<MsgId>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum MlstBodyBaseReq {
    Init(MlstBodyReqInit),
    Echo(MlstBodyReqEcho),
    Topology(MlstBodyReqTopology),
}

#[derive(Serialize, Deserialize)]
pub struct MlstBodyReq {
    #[serde(flatten)]
    pub body: MlstBodyBaseReq,
    pub msg_id: MsgId,
}

#[derive(Serialize, Deserialize)]
pub struct MlstReq {
    pub id: i64,
    pub src: NodeId,
    pub dest: NodeId,
    pub body: MlstBodyReq,
}
