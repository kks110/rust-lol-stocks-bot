pub struct GraphData {
    pub graph_name: String,
    pub x_lower: i32,
    pub x_upper: i32,
    pub x_description: String,
    pub y_lower: i32,
    pub y_upper: i32,
    pub y_description: String,
    pub data: Vec<(i32, i32)>
}