use crate::models::graph_data_series::GraphDataSeries;

pub struct GraphDataMultiSeries {
    pub file_name: String,
    pub graph_name: String,
    pub x_lower: i32,
    pub x_upper: i32,
    pub x_description: String,
    pub y_lower: i32,
    pub y_upper: i32,
    pub y_description: String,
    pub data: Vec<GraphDataSeries>
}
