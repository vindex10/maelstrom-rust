use crate::node::proto::MlstAckBodyReq;
use crate::node::{CommId, MsgId, MsgTypeType, Node, NodeId};

pub trait MlstReplicate: Node {
    fn process_replicate(
        &self,
        comm_id: Option<CommId>,
        src: NodeId,
        _dest: NodeId,
        body_req: serde_json::Value,
    ) {
    }

    fn get_route_replicate() -> MsgTypeType;
}

pub mod proto {
    use crate::node::{MsgId, MsgType, MsgTypeType};
    use serde::{Deserialize, Serialize};
}
