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
        .controls { text-align: center; margin-bottom: 20px; display: flex; justify-content: center; gap: 15px; flex-wrap: wrap; }
        .control-group { display: flex; align-items: center; gap: 5px; }
        .chart-container { width: 90%; margin: 0 auto; height: 60vh; }
        select, input { padding: 5px; font-size: 14px; }
    </style>
</head>
<body>
    <h1 style="text-align: center;">Git Productivity Report</h1>
    
    <div class="controls">
        <div class="control-group">
            <label for="metricSelect">Metric: </label>
            <select id="metricSelect" onchange="updateChart()">
                <option value="total_changes">Total Changes (Added + Deleted)</option>
                <option value="added">Added Lines</option>
                <option value="deleted">Deleted Lines</option>
            </select>
        </div>
        <div class="control-group">
            <label for="chartTypeSelect">Chart Type: </label>
            <select id="chartTypeSelect" onchange="updateChart()">
                <option value="line">Line Chart</option>
                <option value="bar">Stacked Bar Chart</option>
            </select>
        </div>
        <div class="control-group">
            <label for="startDate">Start: </label>
            <input type="date" id="startDate" onchange="updateChart()">
        </div>
        <div class="control-group">
            <label for="endDate">End: </label>
            <input type="date" id="endDate" onchange="updateChart()">
        </div>
    </div>

    <div class="chart-container">
        <canvas id="productivityChart"></canvas>
    </div>

    <script>
        const data = {{ data | json_encode() | safe }};
        const ctx = document.getElementById('productivityChart').getContext('2d');
        let chart;

        // Extract unique users and dates (globally)
        const allUsers = [...new Set(data.map(d => d.user))];
        const allDates = [...new Set(data.map(d => d.date))].sort();

        // Initialize date inputs
        if (allDates.length > 0) {
            document.getElementById('startDate').value = allDates[0];
            document.getElementById('endDate').value = allDates[allDates.length - 1];
        }

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
            const chartType = document.getElementById('chartTypeSelect').value;
            const startDate = document.getElementById('startDate').value;
            const endDate = document.getElementById('endDate').value;

            // Filter dates
            const filteredDates = allDates.filter(d => d >= startDate && d <= endDate);

            const datasets = allUsers.map(user => {
                return {
                    label: user,
                    data: filteredDates.map(date => {
                        const entry = data.find(d => d.user === user && d.date === date);
                        return entry ? entry[metric] : 0;
                    }),
                    fill: chartType === 'bar', // Only fill if bar (stacked logic handled in options)
                    borderColor: stringToColor(user),
                    backgroundColor: stringToColor(user), // For bar chart
                    tension: 0.1,
                    borderWidth: chartType === 'bar' ? 0 : 2
                };
            });

            if (chart) {
                chart.destroy();
            }

            const isStacked = chartType === 'bar';

            chart = new Chart(ctx, {
                type: chartType,
                data: {
                    labels: filteredDates,
                    datasets: datasets
                },
                options: {
                    responsive: true,
                    maintainAspectRatio: false,
                    scales: {
                        x: { 
                            title: { display: true, text: 'Date' },
                            stacked: isStacked 
                        },
                        y: { 
                            beginAtZero: true,
                            title: { display: true, text: 'Lines of Code' },
                            stacked: isStacked
                        }
                    },
                    plugins: {
                        title: {
                            display: true,
                            text: `Productivity by ${metric} (${chartType})`
                        },
                        tooltip: {
                            mode: isStacked ? 'index' : 'nearest',
                            intersect: false
                        }
                    },
                    interaction: {
                        mode: isStacked ? 'index' : 'nearest',
                        intersect: false
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


