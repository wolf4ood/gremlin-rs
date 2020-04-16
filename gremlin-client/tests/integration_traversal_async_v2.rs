#[allow(dead_code)]
mod common;

#[cfg(feature = "async_std")]
mod aio {
    use gremlin_client::process::traversal::traversal;

    use super::common::aio::{
        connect_serializer, create_vertex_with_label, drop_vertices,
    };

    use async_std::prelude::*;
    use async_std::task;
    use gremlin_client::{GraphSON, Vertex};

    #[test]
    fn test_simple_vertex_traversal_with_multiple_id_v2() {
        task::block_on(async {
            let client = connect_serializer(GraphSON::V2).await;
            drop_vertices(&client, "test_simple_vertex_traversal_async")
                .await
                .unwrap();

            let vertex = create_vertex_with_label(
                &client,
                "test_simple_vertex_traversal_async",
                "Traversal",
            )
            .await;
            let vertex2 = create_vertex_with_label(
                &client,
                "test_simple_vertex_traversal_async",
                "Traversal",
            )
            .await;

            let g = traversal().with_remote_async(client);

            let results = g
                .v(vec![vertex.id(), vertex2.id()])
                .to_list()
                .await
                .unwrap();

            assert_eq!(2, results.len());

            assert_eq!(vertex.id(), results[0].id());
            assert_eq!(vertex2.id(), results[1].id());

            let has_next = g
                .v(())
                .has_label("test_simple_vertex_traversal_async")
                .has_next()
                .await
                .expect("It should return");

            assert_eq!(true, has_next);

            let next = g
                .v(())
                .has_label("test_simple_vertex_traversal_async")
                .next()
                .await
                .expect("It should execute one traversal")
                .expect("It should return one element");

            assert_eq!("test_simple_vertex_traversal_async", next.label());

            let vertices = g
                .v(())
                .has_label("test_simple_vertex_traversal_async")
                .iter()
                .await
                .expect("It should get the iterator")
                .collect::<Result<Vec<Vertex>, _>>()
                .await
                .expect("It should collect elements");

            assert_eq!(2, vertices.len());
        })
    }
}
