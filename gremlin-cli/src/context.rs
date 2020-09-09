use gremlin_client::{aio::GremlinClient, GValue};

pub struct GremlinContext {
    pub prompt: String,
    pub alias: String,
    pub client: Option<GremlinClient>,
    pub last_results: Vec<GValue>,
}

impl GremlinContext {
    pub fn builder() -> GremlinContextBuilder {
        GremlinContextBuilder(GremlinContext {
            prompt: String::from("gremlin> "),
            alias: "g.".into(),
            client: None,
            last_results: vec![],
        })
    }
}

pub struct GremlinContextBuilder(GremlinContext);

impl GremlinContextBuilder {
    pub fn build(self) -> GremlinContext {
        self.0
    }
}
