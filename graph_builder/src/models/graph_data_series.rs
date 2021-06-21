use crate::models::graph_data_point::GraphDataPoint;

pub struct GraphDataSeries {
    pub name: String,
    pub series: Vec<GraphDataPoint>,
}
