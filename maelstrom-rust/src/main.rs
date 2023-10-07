mod node;
mod routes;

use crate::node::{CommId, MsgCached, MsgCachedKey, MsgId, MsgType, MsgTypeType, Node, NodeId};
use crate::routes::broadcast::MlstBroadcast;
use crate::routes::echo::MlstEcho;
use crate::routes::init::MlstInit;
use std::collections::{HashMap, HashSet};
use std::io;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> io::Result<()> {
    let service = Arc::new(MlstService::new());
    tokio::task::spawn({
        let s = Arc::clone(&service);
        async move {
            loop {
                s.repeat_unacked();
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
        }
    });
    let _ = tokio::task::spawn({
        let s = Arc::clone(&service);
        async move {
            loop {
                s.main();
            }
        }
    }).await;
    Ok(())
}

struct MlstService {
    pub node_id: Mutex<Option<NodeId>>,
    pub neighbor_ids: Mutex<Vec<NodeId>>,
    pub messages: Mutex<HashSet<MsgType>>,
    pub next_msg_id: Mutex<MsgId>,
    pub pending_ack_ids: Mutex<HashMap<MsgCachedKey, MsgCached>>,
}

impl MlstService {
    pub fn new() -> Self {
        Self {
            node_id: Mutex::new(None),
            neighbor_ids: Mutex::new(Vec::new()),
            messages: Mutex::new(HashSet::new()),
            next_msg_id: Mutex::new(1),
            pending_ack_ids: Mutex::new(HashMap::new()),
        }
    }
}

impl MlstInit for MlstService {
    #[inline]
    fn get_route_init() -> MsgTypeType {
        return "init".to_string();
    }
}
impl MlstEcho for MlstService {
    #[inline]
    fn get_route_echo() -> MsgTypeType {
        return "echo".to_string();
    }
}
impl MlstBroadcast for MlstService {
    #[inline]
    fn get_route_topology() -> MsgTypeType {
        return "topology".to_string();
    }

    #[inline]
    fn get_route_broadcast() -> MsgTypeType {
        return "broadcast".to_string();
    }

    #[inline]
    fn get_route_broadcast_ok() -> MsgTypeType {
        return "broadcast_ok".to_string();
    }

    #[inline]
    fn get_route_read() -> MsgTypeType {
        return "read".to_string();
    }
}

impl Node for MlstService {
    fn dispatch_request(
        &self,
        comm_id: Option<CommId>,
        msg_type: MsgTypeType,
        src: NodeId,
        dest: NodeId,
        body_req: serde_json::Value,
    ) {
        let msg_type_str = msg_type.as_str();
        match msg_type_str {
            "init" => self.process_init(comm_id, src, dest, body_req),
            "echo" => self.process_echo(comm_id, src, dest, body_req),
            "topology" => self.process_topology(comm_id, src, dest, body_req),
            "broadcast" => self.process_broadcast(comm_id, src, dest, body_req),
            "broadcast_ok" => self.process_broadcast_ok(comm_id, src, dest, body_req),
            "read" => self.process_read(comm_id, src, dest, body_req),
            _ => panic!("Unmatched message type"),
        }
    }

    fn next_msg_id(&self) -> MsgId {
        let mut next_msg_id = self.next_msg_id.lock().unwrap();
        let msg_id = *next_msg_id;
        *next_msg_id += 1;
        msg_id
    }

    fn get_node_id(&self) -> &Mutex<Option<NodeId>> {
        return &self.node_id;
    }

    fn set_node_id(&self, value: NodeId) {
        *self.node_id.lock().unwrap() = Some(value)
    }

    fn set_neighbor_ids(&self, values: Vec<NodeId>) {
        *self.neighbor_ids.lock().unwrap() = values;
    }

    fn get_neighbor_ids(&self) -> &Mutex<Vec<NodeId>> {
        &self.neighbor_ids
    }

    fn store_message(&self, message: MsgType) {
        self.messages.lock().unwrap().insert(message);
    }

    fn check_message(&self, message: &MsgType) -> bool {
        self.messages.lock().unwrap().contains(message)
    }

    fn get_messages(&self) -> &Mutex<HashSet<MsgType>> {
        &self.messages
    }

    fn get_pending_ack_ids(&self) -> &Mutex<HashMap<MsgCachedKey, MsgCached>> {
        &self.pending_ack_ids
    }

    fn ack_await(&self, key: MsgCachedKey, msg_cached: MsgCached) {
        self.pending_ack_ids
            .lock()
            .unwrap()
            .insert(key, msg_cached);
    }

    fn ack_delivered(&self, key: &MsgCachedKey) {
        self.log(&format!("Delivered OK: {}", &key.msg_id));
        self.pending_ack_ids.lock().unwrap().remove(key);
    }
}
