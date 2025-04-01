use anyhow::Result;
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
pub struct SpeedTestResult {
    pub timestamp: DateTime<Utc>,
    pub download_speed: f64, // in Mbps
    pub upload_speed: f64,   // in Mbps
    pub ping: f64,           // in ms
    pub server: String,
}

pub struct SpeedTestManager {
    data_file: String,
    client: Client,
}

impl SpeedTestManager {
    pub fn new(data_file: &str) -> Self {
        println!("Creating SpeedTestManager with data file: {}", data_file);
        // Create the data directory if it doesn't exist
        if let Some(parent) = Path::new(data_file).parent() {
            match std::fs::create_dir_all(parent) {
                Ok(_) => println!("Created/verified data directory: {:?}", parent),
                Err(e) => eprintln!("Error creating data directory: {}", e),
            }
        }

        Self {
            data_file: data_file.to_string(),
            client: Client::new(),
        }
    }

    pub async fn run_speed_test(&self) -> Result<SpeedTestResult> {
        println!("Starting speed test...");

        // Test download speed using a large file from a reliable source
        println!("Testing download speed...");
        let download_start = std::time::Instant::now();
        let download_response = self
            .client
            .get("https://dl.google.com/dl/android/studio/install/3.6.3.0/android-studio-ide-192.6392135-windows.exe")
            .timeout(Duration::from_secs(30))
            .send()
            .await?;

        if !download_response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Download test failed: {}",
                download_response.status()
            ));
        }

        let download_size = download_response.content_length().unwrap_or(0) as f64;
        let download_duration = download_start.elapsed().as_secs_f64();
        let download_speed = if download_duration > 0.0 {
            (download_size * 8.0) / (1024.0 * 1024.0 * download_duration) // Convert to Mbps
        } else {
            0.0
        };
        println!("Download speed: {:.2} Mbps", download_speed);

        // Test upload speed using a smaller file
        println!("Testing upload speed...");
        let upload_data = vec![0u8; 1 * 1024 * 1024]; // 1MB of data
        let upload_start = std::time::Instant::now();
        let upload_response = self
            .client
            .post("https://httpbin.org/post")
            .body(upload_data)
            .timeout(Duration::from_secs(10))
            .send()
            .await?;

        if !upload_response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Upload test failed: {}",
                upload_response.status()
            ));
        }

        let upload_duration = upload_start.elapsed().as_secs_f64();
        let upload_speed = if upload_duration > 0.0 {
            (1.0 * 8.0) / upload_duration // Convert to Mbps
        } else {
            0.0
        };
        println!("Upload speed: {:.2} Mbps", upload_speed);

        // Test ping using a simple HTTP request to a reliable endpoint
        println!("Testing ping...");
        let ping_start = std::time::Instant::now();
        let ping_response = self
            .client
            .get("https://httpbin.org/get")
            .timeout(Duration::from_secs(5))
            .send()
            .await?;

        if !ping_response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Ping test failed: {}",
                ping_response.status()
            ));
        }

        let ping = ping_start.elapsed().as_secs_f64() * 1000.0; // Convert to ms
        println!("Ping: {:.0} ms", ping);

        let result = SpeedTestResult {
            timestamp: Utc::now(),
            download_speed,
            upload_speed,
            ping,
            server: "Public Test Servers".to_string(),
        };

        // Save the result
        println!("Saving results to file...");
        self.save_result(&result)?;
        println!("Results saved successfully");

        Ok(result)
    }

    fn save_result(&self, result: &SpeedTestResult) -> Result<()> {
        let path = Path::new(&self.data_file);
        let file_exists = path.exists();
        println!("Saving to file: {:?} (exists: {})", path, file_exists);

        // Open file in append mode
        let file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;

        let mut writer = csv::Writer::from_writer(file);

        // Write the record
        writer.write_record(&[
            result.timestamp.to_rfc3339(),
            result.download_speed.to_string(),
            result.upload_speed.to_string(),
            result.ping.to_string(),
            result.server.clone(),
        ])?;

        writer.flush()?;
        println!("Successfully wrote record to CSV");
        Ok(())
    }

    pub fn load_results(&self) -> Result<Vec<SpeedTestResult>> {
        let path = Path::new(&self.data_file);
        println!(
            "Loading results from: {:?} (exists: {})",
            path,
            path.exists()
        );

        if !path.exists() {
            println!("No results file found, returning empty vector");
            return Ok(Vec::new());
        }

        let file = std::fs::File::open(path)?;
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(file);

        let results: Vec<SpeedTestResult> = reader
            .deserialize()
            .filter_map(|res| match res {
                Ok(record) => Some(record),
                Err(e) => {
                    eprintln!("Error parsing CSV row: {}", e);
                    None
                }
            })
            .collect();

        println!("Loaded {} results", results.len());
        for result in &results {
            println!("Result: {:?}", result);
        }
        Ok(results)
    }
}
