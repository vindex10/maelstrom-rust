use crate::async_comm_node::{AsyncCommNode, MsgCachedKey};
use crate::node::proto::MlstAckBodyReq;
use crate::node::{CommId, MsgId, MsgTypeType, NodeId};
use proto::{MlstBodyReqBroadcast, MlstBodyRespBroadcast};

pub trait MlstBroadcast: AsyncCommNode {
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
            msg_type: Self::get_route_broadcast_ok(),
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
}

pub mod proto {
    use crate::node::{MsgId, MsgType, MsgTypeType};
    use serde::{Deserialize, Serialize};

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
}
