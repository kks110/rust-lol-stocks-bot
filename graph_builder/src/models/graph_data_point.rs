use chrono::NaiveDate;

pub struct GraphDataPoint {
    pub x: NaiveDate,
    pub y: i32
}

impl GraphDataPoint {
    pub fn new(x: NaiveDate, y: i32) -> GraphDataPoint {
        GraphDataPoint {
            x,
            y
        }
    }
}