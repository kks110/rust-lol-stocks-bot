pub mod models;

use plotters::prelude::*;
use models::graph_data::GraphData;

pub fn build(graph_data: GraphData) {
    let drawing_area = BitMapBackend::new(&graph_data.file_name, (600, 400))
        .into_drawing_area();

    drawing_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&drawing_area)
        .margin(20)
        .caption(graph_data.graph_name, ("Arial", 30))
        .y_label_area_size(40)
        .x_label_area_size(40)
        // X - Week number. From 1 to however many performance histories there are
        // Y - Portfolio value. From 100 less than lowers to 100 more than highest
        .build_cartesian_2d(graph_data.x_lower..graph_data.x_upper, graph_data.y_lower..graph_data.y_upper)
        .unwrap();

    chart.configure_mesh()
        .disable_mesh()
        .x_desc(graph_data.x_description)
        .y_desc(graph_data.y_description)
        .draw()
        .unwrap();

    let mut data_vec: Vec<(i32, i32)> = vec![];
    for point in graph_data.data {
        data_vec.push((point.x, point.y))
    }

    chart.draw_series(
        LineSeries::new(data_vec, &BLACK),
    ).unwrap();
}
