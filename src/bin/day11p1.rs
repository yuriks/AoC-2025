use std::io;
use std::cell::Cell;
use std::collections::HashMap;

fn convert_node_id(s: &str) -> NodeId {
    assert_eq!(s.len(), 3);
    assert!(s.is_ascii());
    s.as_bytes().try_into().unwrap()
}

type NodeId = [u8; 3];

struct Node {
    visited: Cell<bool>,
    outs: Vec<NodeId>,
}

fn recurse(counter: &mut u32, connections: &HashMap<NodeId, Node>, node_id: &NodeId) {
    let node = &connections[node_id];
    assert_eq!(node.visited.replace(true), false);
    for id in node.outs.iter() {
        if id == b"out" {
            *counter += 1;
        } else {
            recurse(counter, connections, id);
        }
    }
    node.visited.set(false);
}

fn main() -> io::Result<()> {
    let mut connections: HashMap<NodeId, Node> = HashMap::new();

    for line in io::stdin().lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }

        let mut it = line.split_ascii_whitespace();
        let src_node = it.next().unwrap().strip_suffix(':').unwrap();
        let dst_nodes = it.map(convert_node_id).collect();
        connections.insert(
            convert_node_id(src_node),
            Node { visited: Cell::new(false), outs: dst_nodes });
    }

    let mut path_count = 0u32;
    recurse(&mut path_count, &mut connections, &b"you");

    println!("Result: {path_count}");
    Ok(())
}
