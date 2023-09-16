use crate::node::proto::MlstAckBodyReq;
use crate::node::{CommId, MsgCachedKey, MsgId, MsgTypeType, Node, NodeId};
use proto::{
    MlstBodyReqBroadcast, MlstBodyReqRead, MlstBodyReqTopology, MlstBodyRespBroadcast,
    MlstBodyRespRead, MlstBodyRespTopology,
};

pub trait MlstBroadcast: Node {
    fn process_topology(
        &self,
        _comm_id: Option<CommId>,
        src: NodeId,
        _dest: NodeId,
        body_req: serde_json::Value,
    ) {
        self.log("TOPOLOGY");
        let req_body: MlstBodyReqTopology = serde_json::from_value(body_req).unwrap();
        let node_id_lock = self.get_node_id().lock();
        let topology = req_body.topology[node_id_lock.unwrap().as_ref().unwrap()].to_owned();
        self.set_neighbor_ids(topology);
        let resp_body = MlstBodyRespTopology {
            msg_type: "topology_ok".to_string(),
        };
        self.reply(req_body.msg_id, src, resp_body);
    }

    fn get_route_topology() -> MsgTypeType;

    fn process_broadcast(
        &self,
        comm_id: Option<CommId>,
        src: NodeId,
        _dest: NodeId,
        body_req: serde_json::Value,
    ) {
        self.log("BROADCAST");
        let req_body: MlstBodyReqBroadcast = serde_json::from_value(body_req).unwrap();
        let msg_id = &req_body.msg_id;
        let msg = req_body.message.to_owned();
        if self.check_message(&msg) {
            return;
        }
        self.store_message(msg);
        // to_owned() her is because of interprocedural conflict. can be refactored to avoid copying
        let neighbor_ids = self.get_neighbor_ids().lock().unwrap().to_owned();
        for neighbor_id in neighbor_ids.iter() {
            if neighbor_id == &src {
                continue;
            };
            self.await_communicate(req_body.msg_id, neighbor_id.to_owned(), &req_body);
        }
        if comm_id.is_none() {
            self.log("do not reply");
            return;
        }
        self.log(&format!("msg_id: {}", msg_id));
        let resp_body = MlstBodyRespBroadcast {
            msg_type: "broadcast_ok".to_string(),
        };
        self.reply(msg_id.to_owned(), src, resp_body);
    }

    fn get_route_broadcast() -> MsgTypeType;

    fn process_broadcast_ok(
        &self,
        _msg_id: Option<MsgId>,
        src: NodeId,
        _dest: NodeId,
        body_req: serde_json::Value,
    ) {
        self.log("BROADCAST OK");
        let req_body: MlstAckBodyReq = serde_json::from_value(body_req).unwrap();
        let key = MsgCachedKey {
            msg_id: req_body.in_reply_to.to_owned(),
            dest: src,
        };
        self.ack_delivered(&key);
    }

    fn get_route_broadcast_ok() -> MsgTypeType;

    fn process_read(
        &self,
        _msg_id: Option<MsgId>,
        src: NodeId,
        _dest: NodeId,
        req_body_raw: serde_json::Value,
    ) {
        self.log("READ");
        let req_body: MlstBodyReqRead = serde_json::from_value(req_body_raw).unwrap();
        let resp_body = MlstBodyRespRead {
            msg_type: "read_ok".to_string(),
            messages: self
                .get_messages()
                .lock()
                .unwrap()
                .to_owned()
                .into_iter()
                .collect(),
        };
        self.reply(req_body.msg_id, src, resp_body);
    }

    fn get_route_read() -> MsgTypeType;
}

pub mod proto {
    use crate::node::{MsgId, MsgType, MsgTypeType, NodeId};
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Serialize, Deserialize)]
    pub struct MlstBodyReqTopology {
        pub msg_id: MsgId,
        #[serde(rename = "type")]
        pub msg_type: MsgTypeType,
        pub topology: HashMap<NodeId, Vec<NodeId>>,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct MlstBodyRespTopology {
        #[serde(rename = "type")]
        pub msg_type: MsgTypeType,
    }

    #[derive(Serialize, Deserialize)]
    pub struct MlstBodyReqBroadcast {
        pub msg_id: MsgId,
        #[serde(rename = "type")]
        pub msg_type: MsgTypeType,
        pub message: MsgType,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct MlstBodyRespBroadcast {
        #[serde(rename = "type")]
        pub msg_type: MsgTypeType,
    }

    #[derive(Serialize, Deserialize)]
    pub struct MlstBodyReqRead {
        pub msg_id: MsgId,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct MlstBodyRespRead {
        #[serde(rename = "type")]
        pub msg_type: MsgTypeType,
        pub messages: Vec<MsgType>,
    }
}
