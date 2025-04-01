use crate::speedtest::SpeedTestResult;
use plotly::layout::Axis;
use plotly::{Layout, Plot, Scatter};
use serde_json::Value;

pub fn generate_plot(results: &[SpeedTestResult]) -> Value {
    let mut plot = Plot::new();

    // Download speed trace
    let download_trace = Scatter::new(
        results.iter().map(|r| r.timestamp.to_rfc3339()).collect(),
        results.iter().map(|r| r.download_speed).collect(),
    )
    .name("Download Speed")
    .mode(plotly::common::Mode::LinesMarkers)
    .line(plotly::common::Line::new().color("rgb(31, 119, 180)"))
    .marker(plotly::common::Marker::new().color("rgb(31, 119, 180)"));

    // Upload speed trace
    let upload_trace = Scatter::new(
        results.iter().map(|r| r.timestamp.to_rfc3339()).collect(),
        results.iter().map(|r| r.upload_speed).collect(),
    )
    .name("Upload Speed")
    .mode(plotly::common::Mode::LinesMarkers)
    .line(plotly::common::Line::new().color("rgb(214, 39, 40)"))
    .marker(plotly::common::Marker::new().color("rgb(214, 39, 40)"));

    let layout = Layout::new()
        .title(plotly::common::Title::new("Network Speed Over Time"))
        .x_axis(Axis::new().title(plotly::common::Title::new("Time")))
        .y_axis(Axis::new().title(plotly::common::Title::new("Speed (Mbps)")))
        .show_legend(true)
        .height(500)
        .margin(
            plotly::layout::Margin::new()
                .top(50)
                .right(50)
                .bottom(50)
                .left(50),
        );

    plot.add_trace(download_trace);
    plot.add_trace(upload_trace);
    plot.set_layout(layout);

    let plot_value = serde_json::to_value(plot).unwrap();
    println!("Generated plot data: {}", plot_value);
    plot_value
}
