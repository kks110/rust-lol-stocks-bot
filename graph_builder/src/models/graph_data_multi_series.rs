use chrono::NaiveDate;
use crate::models::graph_data_series::GraphDataSeries;

pub struct GraphDataMultiSeries {
    pub file_name: String,
    pub graph_name: String,
    pub x_lower: NaiveDate,
    pub x_upper: NaiveDate,
    pub x_description: String,
    pub y_lower: i32,
    pub y_upper: i32,
    pub y_description: String,
    pub data: Vec<GraphDataSeries>
}


impl GraphDataMultiSeries {
    pub fn new(
        file_name: &str,
        graph_name: &str,
        x_lower: NaiveDate,
        x_upper: NaiveDate,
        x_description: &str,
        y_lower: i32,
        y_upper: i32,
        y_description: &str,
        data: Vec<GraphDataSeries>
    ) -> GraphDataMultiSeries {
        GraphDataMultiSeries {
            file_name: file_name.to_string(),
            graph_name: graph_name.to_string(),
            x_lower,
            x_upper,
            x_description: x_description.to_string(),
            y_lower,
            y_upper,
            y_description: y_description.to_string(),
            data
        }
    }
}