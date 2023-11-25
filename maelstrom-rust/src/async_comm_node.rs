use crate::node::proto::MlstBodyType;
use crate::node::{MsgId, Node, NodeId};
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Clone)]
pub struct MsgCached {
    pub msg_str: String,
}

#[derive(Hash, Eq, PartialEq)]
pub struct MsgCachedKey {
    pub msg_id: MsgId,
    pub dest: NodeId,
}

pub trait AsyncCommNode: Node {
    fn await_communicate(&self, msg_id: MsgId, dest: NodeId, msg: impl serde::Serialize) {
        let msg_cached = MsgCached {
            msg_str: serde_json::to_string(&msg).unwrap(),
        };
        let key = MsgCachedKey { msg_id, dest };
        self.ack_await(key, msg_cached);
    }

    fn repeat_unacked(&self) {
        let unacked = self.get_pending_ack_ids().lock().unwrap();
        for (key, msg_cached) in &*unacked {
            let dest = key.dest.to_owned();
            let raw_val =
                serde_json::value::RawValue::from_string(msg_cached.msg_str.to_owned()).unwrap();
            self.communicate(dest, MlstBodyType::<u8>::Comm(raw_val));
        }
    }

    fn get_pending_ack_ids(&self) -> &Mutex<HashMap<MsgCachedKey, MsgCached>>;

    fn ack_await(&self, key: MsgCachedKey, msg_cached: MsgCached);

    fn ack_delivered(&self, key: &MsgCachedKey);
}
