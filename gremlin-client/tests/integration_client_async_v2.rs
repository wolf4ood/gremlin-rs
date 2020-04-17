#[allow(dead_code)]
mod common;

#[cfg(feature = "async_gremlin")]
mod aio {

    use gremlin_client::GremlinError;
    use gremlin_client::{Edge, GValue, GraphSON, Map, Vertex};

    use super::common::aio::{connect_serializer, create_edge, create_vertex};
    #[cfg(feature = "async-std-runtime")]
    use async_std::prelude::*;

    #[cfg(feature = "tokio-runtime")]
    use tokio::stream::StreamExt;

    #[cfg_attr(feature = "async-std-runtime", async_std::test)]
    #[cfg_attr(feature = "tokio-runtime", tokio::test)]
    async fn test_client_connection_ok_v2() {
        connect_serializer(GraphSON::V2).await;
    }

    #[cfg(feature = "async-std-runtime")]
    #[cfg_attr(feature = "async-std-runtime", async_std::test)]
    async fn test_empty_query_v2() {
        let graph = connect_serializer(GraphSON::V2).await;

        assert_eq!(
            0,
            graph
                .execute("g.V().hasLabel('NotFound')", &[])
                .await
                .expect("It should execute a traversal")
                .count()
                .await
        )
    }

    #[cfg_attr(feature = "async-std-runtime", async_std::test)]
    #[cfg_attr(feature = "tokio-runtime", tokio::test)]
    async fn test_wrong_query_v2() {
        let error = connect_serializer(GraphSON::V2)
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
    }

    #[cfg_attr(feature = "async-std-runtime", async_std::test)]
    #[cfg_attr(feature = "tokio-runtime", tokio::test)]
    async fn test_wrong_alias_v2() {
        let error = connect_serializer(GraphSON::V2)
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
    }

    #[cfg_attr(feature = "async-std-runtime", async_std::test)]
    #[cfg_attr(feature = "tokio-runtime", tokio::test)]

    async fn test_vertex_query_v2() {
        let graph = connect_serializer(GraphSON::V2).await;

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
    }
    #[cfg_attr(feature = "async-std-runtime", async_std::test)]
    #[cfg_attr(feature = "tokio-runtime", tokio::test)]
    async fn test_edge_query_v2() {
        let graph = connect_serializer(GraphSON::V2).await;
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
    }

    #[cfg_attr(feature = "async-std-runtime", async_std::test)]
    #[cfg_attr(feature = "tokio-runtime", tokio::test)]
    async fn test_vertex_creation_v2() {
        let graph = connect_serializer(GraphSON::V2).await;
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
    }

    #[cfg_attr(feature = "async-std-runtime", async_std::test)]
    #[cfg_attr(feature = "tokio-runtime", tokio::test)]
    async fn test_edge_creation_v2() {
        let graph = connect_serializer(GraphSON::V2).await;
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
    }
}
