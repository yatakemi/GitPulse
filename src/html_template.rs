pub const HTML_TEMPLATE: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Git Productivity Report</title>
    <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
    <style>
        body { font-family: sans-serif; padding: 20px; }
        .controls { text-align: center; margin-bottom: 20px; }
        .chart-container { width: 80%; margin: 0 auto; }
        select { padding: 5px; font-size: 16px; }
    </style>
</head>
<body>
    <h1 style="text-align: center;">Git Productivity Report</h1>
    
    <div class="controls">
        <label for="metricSelect">Metric: </label>
        <select id="metricSelect" onchange="updateChart()">
            <option value="total_changes">Total Changes (Added + Deleted)</option>
            <option value="added">Added Lines</option>
            <option value="deleted">Deleted Lines</option>
        </select>
    </div>

    <div class="chart-container">
        <canvas id="productivityChart"></canvas>
    </div>

    <script>
        const data = {{ data | json_encode() | safe }};
        const ctx = document.getElementById('productivityChart').getContext('2d');
        let chart;

        // Extract unique users and dates
        const users = [...new Set(data.map(d => d.user))];
        const dates = [...new Set(data.map(d => d.date))].sort();
        
        // Hash string to color
        function stringToColor(str) {
            let hash = 0;
            for (let i = 0; i < str.length; i++) {
                hash = str.charCodeAt(i) + ((hash << 5) - hash);
            }
            const c = (hash & 0x00FFFFFF).toString(16).toUpperCase();
            return '#' + '00000'.substring(0, 6 - c.length) + c;
        }

        function updateChart() {
            const metric = document.getElementById('metricSelect').value;
            
            const datasets = users.map(user => {
                return {
                    label: user,
                    data: dates.map(date => {
                        const entry = data.find(d => d.user === user && d.date === date);
                        return entry ? entry[metric] : 0;
                    }),
                    fill: false,
                    borderColor: stringToColor(user),
                    tension: 0.1
                };
            });

            if (chart) {
                chart.destroy();
            }

            chart = new Chart(ctx, {
                type: 'line',
                data: {
                    labels: dates,
                    datasets: datasets
                },
                options: {
                    responsive: true,
                    scales: {
                        x: { title: { display: true, text: 'Date' } },
                        y: { 
                            beginAtZero: true,
                            title: { display: true, text: 'Lines of Code' } 
                        }
                    },
                    plugins: {
                        title: {
                            display: true,
                            text: 'Productivity by ' + metric
                        }
                    }
                }
            });
        }

        // Initial render
        updateChart();
    </script>
</body>
</html>
"#;

