# 🌍 Network Speed Monitor

A real-time network speed monitoring tool that helps you track your internet performance over time. Built with Rust and JavaScript, this tool provides beautiful visualizations and detailed statistics of your network speeds.

## 🚀 Quick Start

1. Clone the repository
2. Start the backend:
   ```bash
   cd backend
   cargo run
   ```
3. Open `frontend/index.html` in your browser
4. Click "Run Speed Test" to start monitoring your network speed

## 🛠️ How It Works

This tool uses a combination of reliable public endpoints to measure your network performance:

- Download speed: Tests using Google's Android Studio download
- Upload speed: Tests using a 1MB data upload to httpbin.org
- Ping: Measures latency using httpbin.org

The backend (written in Rust) handles the speed tests and data storage, while the frontend (JavaScript) creates beautiful visualizations using Plotly.js. Each test result is saved to a CSV file, allowing you to track your network performance over time.

## 📊 Features

- Real-time speed testing
- Line graphs showing download and upload speeds over time
- Average speed calculations
- Total number of tests performed
- Automatic data persistence
- Modern, responsive UI

## 🎨 Tech Stack

- **Backend**: Rust with Axum web framework
- **Frontend**: Vanilla JavaScript with Plotly.js for visualizations
- **Data Storage**: CSV file-based storage
- **HTTP Client**: reqwest for making HTTP requests
- **Data Processing**: serde for JSON serialization/deserialization

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🤝 Contributing

Feel free to open issues or submit pull requests if you have suggestions for improvements!
