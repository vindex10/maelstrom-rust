use crate::node::proto::MlstBodyType;
use crate::node::{MsgId, Node, NodeId};
use std::collections::HashMap;
use std::sync::Mutex;

pub trait CrdtNode: Node {
    fn broadcast(&self) {}
}
