pub mod models;

use std::error::Error;
use chrono::NaiveDate;
use plotters::prelude::*;
use models::graph_data::GraphData;
use models::graph_data_multi_series::GraphDataMultiSeries;

pub fn build(graph_data: GraphData) -> Result<(), Box<dyn Error>> {
    let drawing_area = BitMapBackend::new(&graph_data.file_name, (1200, 800))
        .into_drawing_area();

    drawing_area.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&drawing_area)
        .margin(20)
        .caption(graph_data.graph_name, ("Arial", 30))
        .y_label_area_size(40)
        .x_label_area_size(40)
        .build_cartesian_2d(graph_data.x_lower..graph_data.x_upper, graph_data.y_lower..graph_data.y_upper)?;

    chart.configure_mesh()
        .disable_mesh()
        .x_desc(graph_data.x_description)
        .y_desc(graph_data.y_description)
        .draw()?;

    let mut data_vec: Vec<(NaiveDate, i32)> = vec![];
    for point in graph_data.data {
        data_vec.push((point.x, point.y))
    }

    chart.draw_series(
        LineSeries::new(data_vec, &BLACK),
    )?;
    Ok(())
}


pub fn build_multi_series(graph_data: GraphDataMultiSeries) -> Result<(), Box<dyn Error>> {
    let drawing_area = BitMapBackend::new(&graph_data.file_name, (1200, 800))
        .into_drawing_area();

    drawing_area.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&drawing_area)
        .margin(20)
        .caption(graph_data.graph_name, ("Arial", 30))
        .y_label_area_size(40)
        .x_label_area_size(40)
        .build_cartesian_2d(graph_data.x_lower..graph_data.x_upper, graph_data.y_lower..graph_data.y_upper)?;

    chart.configure_mesh()
        .disable_mesh()
        .x_desc(graph_data.x_description)
        .y_desc(graph_data.y_description)
        .draw()?;

    for (idx, data_set) in (0..).zip(graph_data.data) {

        let mut data: Vec<(NaiveDate, i32)> = vec![];
        for series in data_set.series {
            data.push((series.x, series.y))
        }

        chart.draw_series(
            LineSeries::new(data, &Palette99::pick(idx)),
        ).unwrap()
        .label(data_set.name)
        .legend(move |(x, y)| Rectangle::new([(x, y), (x + 20, y)], &Palette99::pick(idx)));
    }

    chart.configure_series_labels()
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .position(SeriesLabelPosition::LowerLeft)
        .draw()?;
    Ok(())
}
