async function runSpeedTest() {
    const button = document.getElementById('runTest');
    const plotElement = document.getElementById('plot');
    button.disabled = true;
    button.textContent = 'Running Test...';
    plotElement.innerHTML = '<div class="loading">Running speed test...<br>This may take a minute.</div>';

    try {
        console.log('Starting speed test...');
        const response = await fetch('http://127.0.0.1:8080/api/run-test', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            }
        });
        console.log('Response received:', response.status);
        const data = await response.json();
        console.log('Data received:', data);
        
        if (data.success) {
            console.log('Speed test successful, updating visualization...');
            // Show the current test results
            plotElement.innerHTML = `
                <div class="success">
                    Speed test completed successfully!<br>
                    Download: ${data.result.download_speed.toFixed(2)} Mbps<br>
                    Upload: ${data.result.upload_speed.toFixed(2)} Mbps<br>
                    Ping: ${data.result.ping.toFixed(0)} ms
                </div>
            `;
            // Update the visualization after showing the results
            await loadVisualization();
        } else {
            throw new Error(data.error || 'Speed test failed');
        }
    } catch (error) {
        console.error('Detailed error:', error);
        plotElement.innerHTML = `<div class="error">Error running speed test: ${error.message}<br>Please make sure the backend server is running.</div>`;
    } finally {
        button.disabled = false;
        button.textContent = 'Run Speed Test';
    }
}

async function loadVisualization() {
    try {
        console.log('Fetching visualization data...');
        const response = await fetch('http://127.0.0.1:8080/api/visualize');
        console.log('Visualization response received:', response.status);
        const data = await response.json();
        console.log('Visualization data received:', data);
        
        if (data.success) {
            // Clear existing plot
            const plotElement = document.getElementById('plot');
            plotElement.innerHTML = '';
            
            // Create a container for the plot
            const plotContainer = document.createElement('div');
            plotContainer.style.width = '100%';
            plotContainer.style.height = '500px';
            plotElement.appendChild(plotContainer);
            
            // Check if we have valid plot data
            if (data.plot && data.plot.data && data.plot.layout) {
                console.log('Creating plot with data:', data.plot);
                // Create the plot using Plotly
                Plotly.newPlot(plotContainer, data.plot.data, data.plot.layout);
            } else {
                console.error('Invalid plot data:', data.plot);
                throw new Error('Invalid plot data received from server');
            }
            
            // Update statistics
            if (data.stats) {
                document.getElementById('avgDownload').textContent = 
                    data.stats.avg_download.toFixed(2) + ' Mbps';
                document.getElementById('avgUpload').textContent = 
                    data.stats.avg_upload.toFixed(2) + ' Mbps';
                document.getElementById('avgPing').textContent = 
                    data.stats.avg_ping.toFixed(0) + ' ms';
                document.getElementById('totalTests').textContent = 
                    data.stats.total_tests;
                console.log('Statistics updated:', data.stats);
            } else {
                console.error('Invalid stats data:', data.stats);
                throw new Error('Invalid statistics data received from server');
            }
            
            console.log('Visualization updated successfully');
        } else {
            throw new Error(data.error || 'Failed to load visualization data');
        }
    } catch (error) {
        console.error('Error loading visualization:', error);
        document.getElementById('plot').innerHTML = 
            `<div class="error">Error loading visualization: ${error.message}<br>Please make sure the backend server is running.</div>`;
    }
}

// Add event listener for the speed test button
document.getElementById('runTest').addEventListener('click', runSpeedTest);

// Load initial visualization
loadVisualization();

// Refresh visualization every 30 seconds
setInterval(loadVisualization, 30000);
