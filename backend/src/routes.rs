use crate::speedtest::SpeedTestManager;
use crate::viz::generate_plot;
use axum::response::Json;
use serde_json::json;

pub async fn run_speed_test() -> Json<serde_json::Value> {
    println!("Received speed test request");
    let manager = SpeedTestManager::new("data/network_speeds.csv");
    println!("Created SpeedTestManager");

    match manager.run_speed_test().await {
        Ok(result) => {
            println!("Speed test completed successfully: {:?}", result);
            Json(json!({
                "success": true,
                "result": result
            }))
        }
        Err(e) => {
            println!("Speed test failed: {}", e);
            Json(json!({
                "success": false,
                "error": e.to_string()
            }))
        }
    }
}

pub async fn get_visualization() -> Json<serde_json::Value> {
    println!("Received visualization request");
    let manager = SpeedTestManager::new("data/network_speeds.csv");

    match manager.load_results() {
        Ok(results) => {
            println!("Loaded {} results", results.len());
            let plot_data = generate_plot(&results);

            // Calculate statistics
            let stats = if !results.is_empty() {
                let total_tests = results.len();
                let avg_download =
                    results.iter().map(|r| r.download_speed).sum::<f64>() / total_tests as f64;
                let avg_upload =
                    results.iter().map(|r| r.upload_speed).sum::<f64>() / total_tests as f64;
                let avg_ping = results.iter().map(|r| r.ping).sum::<f64>() / total_tests as f64;

                json!({
                    "total_tests": total_tests,
                    "avg_download": avg_download,
                    "avg_upload": avg_upload,
                    "avg_ping": avg_ping
                })
            } else {
                json!({
                    "total_tests": 0,
                    "avg_download": 0.0,
                    "avg_upload": 0.0,
                    "avg_ping": 0.0
                })
            };

            println!("Sending visualization data with {} results", results.len());
            Json(json!({
                "success": true,
                "plot": plot_data,
                "stats": stats
            }))
        }
        Err(e) => {
            println!("Failed to load results: {}", e);
            Json(json!({
                "success": false,
                "error": e.to_string()
            }))
        }
    }
}
