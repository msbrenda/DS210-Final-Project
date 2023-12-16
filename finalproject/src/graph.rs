// graph.rs

use plotters::prelude::*;
use reqwest;

// Function to fetch wine quality data from the URL
fn fetch_data(url: &str) -> Result<Vec<(f64, f64)>, reqwest::Error> {
    let response = reqwest::blocking::get(url)?;
    let text = response.text()?;
    let mut data = Vec::new();

    for line in text.lines().skip(1) {
        let cols: Vec<f64> = line
            .split(';')
            .filter_map(|s| s.parse::<f64>().ok())
            .collect();

        if cols.len() == 12 {
            let quality = cols[11]; // Quality column
            let alcohol = cols[10]; // Alcohol column

            data.push((quality, alcohol));
        }
    }
    Ok(data)
}

// Function to create and save the graph for wine quality data fetched from URL
pub fn create_graph(output_path: &str, url: &str, title: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Fetch wine quality data
    let data = fetch_data(url)?;

    if data.is_empty() {
        eprintln!("No valid data points found from the URL: {}", url);
        return Ok(());
    }

    let root = BitMapBackend::new(output_path, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    // Create a scatter plot
    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 20))
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0.0..10.0, 8.0..16.0)?;

    chart.configure_mesh().draw()?;

    // Plot the data as points on the graph
    chart.draw_series(
        data.iter().map(|&(x, y)| Circle::new((x, y), 3, BLACK.filled())),
    )?;

    // Save the graph to a file
    root.present()?;
    Ok(())
}
