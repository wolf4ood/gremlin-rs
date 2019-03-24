#[derive(Debug, PartialEq, Clone)]
pub struct TraversalMetrics {
    duration: f64,
    metrics: Vec<Metric>,
}

impl TraversalMetrics {
    pub fn duration(&self) -> &f64 {
        &self.duration
    }

    pub fn metrics(&self) -> &Vec<Metric> {
        &self.metrics
    }
}

impl TraversalMetrics {
    pub fn new(duration: f64, metrics: Vec<Metric>) -> Self {
        TraversalMetrics { duration, metrics }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Metric {
    id: String,
    duration: f64,
    name: String,
    count: i64,
    traversers: i64,
    perc_duration: f64,
}

impl Metric {
    pub fn id(&self) -> &String {
        &self.id
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn duration(&self) -> &f64 {
        &self.duration
    }

    pub fn perc_duration(&self) -> &f64 {
        &self.perc_duration
    }
    pub fn count(&self) -> &i64 {
        &self.count
    }
    pub fn traversers(&self) -> &i64 {
        &self.traversers
    }
}

impl Metric {
    pub fn new<T, V>(
        id: T,
        name: V,
        duration: f64,
        count: i64,
        traversers: i64,
        perc_duration: f64,
    ) -> Self
    where
        T: Into<String>,
        V: Into<String>,
    {
        Metric {
            id: id.into(),
            name: name.into(),
            duration,
            count,
            traversers,
            perc_duration,
        }
    }
}
