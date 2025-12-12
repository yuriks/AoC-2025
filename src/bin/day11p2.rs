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
    reaches_fft: Cell<bool>,
    reaches_dac: Cell<bool>,
    path_count: Cell<Option<u64>>,
    outs: Vec<NodeId>,
}

impl Node {
    fn new(outs: Vec<NodeId>) -> Node {
        Node {
            visited: Cell::new(false),
            reaches_fft: Cell::new(false),
            reaches_dac: Cell::new(false),
            path_count: Cell::new(None),
            outs,
        }
    }
}

struct State<'a> {
    path_len: u32,
    fft_node: &'a Node,
    dac_node: &'a Node,
}

fn recurse(state: &mut State, connections: &HashMap<NodeId, Node>, node_id: &NodeId) -> u64 {
    let node = &connections[node_id];
    if !node.reaches_dac.get() && !state.dac_node.visited.get() {
        return 0;
    }
    if !node.reaches_fft.get() && !state.fft_node.visited.get() {
        return 0;
    }
    if let Some(path_count) = node.path_count.get() {
        return path_count;
    }
    /*for _ in 0..state.path_len {
        print!("  > ");
    }
    println!("Visiting {}", str::from_utf8(node_id).unwrap());*/
    assert_eq!(node.visited.replace(true), false);
    state.path_len += 1;
    let mut path_count = 0;
    for id in node.outs.iter() {
        if id == b"out" {
            assert!(state.fft_node.visited.get() && state.dac_node.visited.get());
            path_count += 1;
        } else {
            if id != b"out" {
                path_count += recurse(state, connections, id);
            }
        }
    }
    node.visited.set(false);
    state.path_len -= 1;

    node.path_count.set(Some(path_count));
    path_count
}

fn tag_node_reachability(nodes: &HashMap<NodeId, Node>, node_ins: &HashMap<NodeId, Vec<NodeId>>, node_id: &NodeId, f: fn(&NodeId, &Node) -> bool) {
    let node = &nodes[node_id];
    if f(node_id, node) {
        for id in &node_ins[node_id] {
            tag_node_reachability(nodes, node_ins, id, f);
        }
    }
}

fn main() -> io::Result<()> {
    let mut nodes: HashMap<NodeId, Node> = HashMap::new();
    for line in io::stdin().lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }

        let mut it = line.split_ascii_whitespace();
        let src_node = it.next().unwrap().strip_suffix(':').unwrap();
        let dst_nodes = it.map(convert_node_id).collect();
        nodes.insert(convert_node_id(src_node), Node::new(dst_nodes));
    }
    nodes.insert(*b"out", Node::new(Vec::new()));

    let mut node_ins: HashMap<NodeId, Vec<NodeId>> = nodes.keys().map(|k| (*k, Vec::new())).collect::<HashMap<_, _>>();
    for (id, node) in &nodes {
        for out_node in node.outs.iter() {
            node_ins.get_mut(out_node).unwrap().push(*id);
        }
    }

    tag_node_reachability(&nodes, &node_ins, &b"fft", |node_id, node| {
        println!("fft <= {:?}", str::from_utf8(node_id));
        node.reaches_fft.replace(true) == false
    });
    tag_node_reachability(&nodes, &node_ins, &b"dac", |node_id, node| {
        println!("dac <= {:?}", str::from_utf8(node_id));
        node.reaches_dac.replace(true) == false
    });

    let fft_node = &nodes[b"fft"];
    let dac_node = &nodes[b"dac"];
    let mut state = State {
        path_len: 0,
        fft_node,
        dac_node,
    };

    let path_count = recurse(&mut state, &nodes, &b"svr");

    println!("Result: {path_count}");
    Ok(())
}
