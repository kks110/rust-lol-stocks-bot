pub mod models;

use rand::Rng;
use plotters::prelude::*;
use plotters::style::RGBColor;
use models::graph_data::GraphData;
use models::graph_data_multi_series::GraphDataMultiSeries;

pub fn build(graph_data: GraphData) {
    let drawing_area = BitMapBackend::new(&graph_data.file_name, (600, 400))
        .into_drawing_area();

    drawing_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&drawing_area)
        .margin(20)
        .caption(graph_data.graph_name, ("Arial", 30))
        .y_label_area_size(40)
        .x_label_area_size(40)
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


pub fn build_multi_series(graph_data: GraphDataMultiSeries) {
    let drawing_area = BitMapBackend::new(&graph_data.file_name, (600, 400))
        .into_drawing_area();

    drawing_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&drawing_area)
        .margin(20)
        .caption(graph_data.graph_name, ("Arial", 30))
        .y_label_area_size(40)
        .x_label_area_size(40)
        .build_cartesian_2d(graph_data.x_lower..graph_data.x_upper, graph_data.y_lower..graph_data.y_upper)
        .unwrap();

    chart.configure_mesh()
        .disable_mesh()
        .x_desc(graph_data.x_description)
        .y_desc(graph_data.y_description)
        .draw()
        .unwrap();

    for data_set in graph_data.data {
        // let mut rng = rand::thread_rng();
        // let r = rng.gen_range(0..256);
        // let g = rng.gen_range(0..256);
        // let b = rng.gen_range(0..256);
        // let rgb_colour = RGBColor(r, g, b);

        let mut data: Vec<(i32, i32)> = vec![];
        for series in data_set.series {
            data.push((series.x, series.y))
        }

        chart.draw_series(
            LineSeries::new(data, &BLUE),
        ).unwrap()
        .label(data_set.name)
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));
    }

    chart.configure_series_labels()
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .draw()
        .unwrap();
}
