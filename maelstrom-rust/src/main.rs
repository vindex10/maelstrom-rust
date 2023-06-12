mod node;
mod proto;
mod types;

use crate::node::Node;
use std::io;

fn main() -> io::Result<()> {
    let mut node = Node::new();
    let _ = node.main();
    Ok(())
}
