use gremlin_client::process::traversal;

mod common;

use common::graph;

#[test]
fn test_simple_vertex_traversal() {
    let g = traversal().with_remote(graph());

    let results = g.v(&[]).to_list().unwrap();

    assert!(results.len() > 0);
}

#[test]
fn test_simple_edge_traversal() {
    let g = traversal().with_remote(graph());

    let results = g.e(&[]).to_list().unwrap();

    assert!(results.len() > 0);
}
