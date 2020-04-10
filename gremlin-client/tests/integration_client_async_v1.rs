mod common_async;

mod aio {

    use gremlin_client::GremlinError;
    use gremlin_client::{Edge, GValue, GraphSON, Map, Vertex};

    use super::common_async::{connect_serializer, create_edge, create_vertex};
    use async_std::prelude::*;
    use async_std::task;

    #[test]
    fn test_client_connection_ok_v1() {
        task::block_on(async {
            connect_serializer(GraphSON::V1).await;
        })
    }

    #[test]
    fn test_empty_query_v1() {
        task::block_on(async {
            let graph = connect_serializer(GraphSON::V1).await;

            assert_eq!(
                0,
                graph
                    .execute("g.V().hasLabel('NotFound')", &[])
                    .await
                    .expect("It should execute a traversal")
                    .count()
                    .await
            )
        })
    }

    #[test]
    fn test_wrong_query_v1() {
        task::block_on(async {
            let error = connect_serializer(GraphSON::V1)
                .await
                .execute("g.V", &[])
                .await
                .expect_err("it should return an error");

            match error {
                GremlinError::Request((code, message)) => {
                    assert_eq!(597, code);
                    assert_eq!("No such property: V for class: org.apache.tinkerpop.gremlin.process.traversal.dsl.graph.GraphTraversalSource",message)
                }
                _ => panic!("wrong error type"),
            }
        })
    }

    #[test]
    fn test_wrong_alias_v1() {
        task::block_on(async {
            let error = connect_serializer(GraphSON::V1)
                .await
                .alias("foo")
                .execute("g.V()", &[])
                .await
                .expect_err("it should return an error");

            match error {
                GremlinError::Request((code, message)) => {
                    assert_eq!(499, code);
                    assert_eq!("Could not alias [g] to [foo] as [foo] not in the Graph or TraversalSource global bindings",message)
                }
                _ => panic!("wrong error type"),
            }
        })
    }

    #[test]

    fn test_vertex_query_v1() {
        task::block_on(async {
            let graph = connect_serializer(GraphSON::V1).await;
            // TODO
            println!("About to execute query.");
            let vertices = graph
                .execute(
                    "g.V().hasLabel('person').has('name',name)",
                    &[("name", &"marko")],
                )
                .await
                .expect("it should execute a query")
                .filter_map(Result::ok)
                .map(|f| f.take::<Vertex>())
                .collect::<Result<Vec<Vertex>, _>>()
                .await
                .expect("It should be ok");

            assert_eq!("person", vertices[0].label());
        })
    }
    #[test]
    fn test_edge_query_v1() {
        task::block_on(async {
            // TODO
            println!("About to execute query.");

            let graph = connect_serializer(GraphSON::V1).await;
            let edges = graph
                .execute("g.E().hasLabel('knows').limit(1)", &[])
                .await
                .expect("it should execute a query")
                .filter_map(Result::ok)
                .map(|f| f.take::<Edge>())
                .collect::<Result<Vec<Edge>, _>>()
                .await
                .expect("It should be ok");

            assert_eq!("knows", edges[0].label());
        })
    }

    #[test]
    fn test_vertex_creation_v1() {
        task::block_on(async {
            let graph = connect_serializer(GraphSON::V1).await;
            let mark = create_vertex(&graph, "mark").await;

            assert_eq!("person", mark.label());

            let value_map = graph
                .execute("g.V(identity).valueMap()", &[("identity", mark.id())])
                .await
                .expect("should fetch valueMap with properties")
                .filter_map(Result::ok)
                .map(|f| f.take::<Map>())
                .collect::<Result<Vec<Map>, _>>()
                .await
                .expect("It should be ok");

            assert_eq!(1, value_map.len());

            assert_eq!(
                Some(&GValue::List(vec![String::from("mark").into()].into())),
                value_map[0].get("name")
            );
        })
    }

    #[test]
    fn test_edge_creation_v1() {
        task::block_on(async {
            let graph = connect_serializer(GraphSON::V1).await;
            let mark = create_vertex(&graph, "mark").await;
            let frank = create_vertex(&graph, "frank").await;

            let edge = create_edge(&graph, &mark, &frank, "knows").await;

            assert_eq!("knows", edge.label());

            assert_eq!(&mark, edge.out_v());
            assert_eq!(&frank, edge.in_v());

            let edges = graph
                .execute("g.V(identity).outE()", &[("identity", mark.id())])
                .await
                .expect("should fetch edge")
                .filter_map(Result::ok)
                .map(|f| f.take::<Edge>())
                .collect::<Result<Vec<Edge>, _>>()
                .await
                .expect("It should be ok");

            assert_eq!(1, edges.len());

            let edge = &edges[0];

            assert_eq!("knows", edge.label());

            assert_eq!(&mark, edge.out_v());
            assert_eq!(&frank, edge.in_v());
        })
    }
}
