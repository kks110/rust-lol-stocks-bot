use crate::models::graph_data_point::GraphDataPoint;

pub struct GraphDataSeries {
    pub name: String,
    pub series: Vec<GraphDataPoint>,
}

impl GraphDataSeries {
    pub fn new(name: &str, series: Vec<GraphDataPoint>) -> GraphDataSeries {
        GraphDataSeries {
            name: name.to_string(),
            series
        }
    }
}