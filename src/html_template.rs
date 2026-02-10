pub const HTML_TEMPLATE: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Git Productivity Report</title>
    <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
    <style>
        body { font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; padding: 20px; background-color: #f8f9fa; color: #333; }
        h1 { text-align: center; margin-bottom: 30px; color: #2c3e50; }
        
        .container { max-width: 1200px; margin: 0 auto; }
        
        .controls { 
            background: white; padding: 20px; border-radius: 12px; box-shadow: 0 4px 6px rgba(0,0,0,0.05); 
            margin-bottom: 25px; display: flex; justify-content: center; gap: 20px; flex-wrap: wrap; 
        }
        .control-group { display: flex; align-items: center; gap: 8px; font-weight: 500; }
        select, input { padding: 8px 12px; border: 1px solid #ddd; border-radius: 6px; font-size: 14px; outline: none; transition: border-color 0.2s; }
        select:focus, input:focus { border-color: #3498db; }
        
        .summary-cards { display: flex; justify-content: center; gap: 20px; margin-bottom: 30px; flex-wrap: wrap; }
        .card { 
            background: white; padding: 20px 30px; border-radius: 12px; box-shadow: 0 4px 6px rgba(0,0,0,0.05); 
            text-align: center; min-width: 180px; flex: 1; max-width: 250px;
        }
        .card h3 { margin: 0 0 10px 0; font-size: 13px; color: #7f8c8d; text-transform: uppercase; letter-spacing: 1px; }
        .card .value { font-size: 28px; font-weight: 700; color: #2c3e50; }
        .card .diff { font-size: 14px; margin-top: 8px; font-weight: 500; }
        .diff.positive { color: #27ae60; }
        .diff.negative { color: #e74c3c; }
        .diff.neutral { color: #95a5a6; }

        .charts-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(500px, 1fr)); gap: 25px; margin-bottom: 30px; }
        .chart-box { background: white; padding: 20px; border-radius: 12px; box-shadow: 0 4px 6px rgba(0,0,0,0.05); height: 400px; position: relative; }
        .chart-box.full-width { grid-column: 1 / -1; height: 500px; }
        .chart-title { position: absolute; top: 15px; left: 20px; font-size: 16px; font-weight: 600; color: #34495e; z-index: 10; }
        
    </style>
</head>
<body>
    <div class="container">
        <h1>Git Productivity Report</h1>
        
        <div class="controls">
            <div class="control-group">
                <label>Metric:</label>
                <select id="metricSelect" onchange="updateDashboard()">
                    <option value="total_changes">Total Changes</option>
                    <option value="added">Added Lines</option>
                    <option value="deleted">Deleted Lines</option>
                    <option value="commit_count">Commit Count</option>
                </select>
            </div>
            <div class="control-group">
                <label>Chart:</label>
                <select id="chartTypeSelect" onchange="updateDashboard()">
                    <option value="line">Line Chart</option>
                    <option value="bar">Stacked Bar</option>
                </select>
            </div>
            <div class="control-group">
                <label>Start:</label>
                <input type="date" id="startDate" onchange="updateDashboard()">
            </div>
            <div class="control-group">
                <label>End:</label>
                <input type="date" id="endDate" onchange="updateDashboard()">
            </div>
            <div class="control-group">
                <input type="checkbox" id="showTrend" onchange="updateDashboard()">
                <label for="showTrend">7-Day Trend</label>
            </div>
        </div>

        <div class="summary-cards">
            <div class="card">
                <h3 id="summaryTitle">Total</h3>
                <div class="value" id="summaryValue">-</div>
                <div class="diff" id="summaryDiff">-</div>
            </div>
            <div class="card">
                <h3>Merge Commits</h3>
                <div class="value" id="mergeCommitsValue">-</div>
             </div>
            <div class="card">
                <h3>Active Days</h3>
                <div class="value" id="activeDaysValue">-</div>
            </div>
            <div class="card">
                <h3>Avg / Day</h3>
                <div class="value" id="avgPerDayValue">-</div>
            </div>
        </div>

        <div class="charts-grid">
            <div class="chart-box full-width">
                <div class="chart-title">Timeline</div>
                <canvas id="productivityChart"></canvas>
            </div>
            <div class="chart-box">
                <div class="chart-title">User Share</div>
                <canvas id="shareChart"></canvas>
            </div>
            <div class="chart-box">
                <div class="chart-title">Day of Week Activity</div>
                <canvas id="dayOfWeekChart"></canvas>
            </div>
            <div class="chart-box full-width">
                <div class="chart-title">Activity Heatmap (Hour vs Day)</div>
                <canvas id="heatmapChart"></canvas>
            </div>
             <div class="chart-box full-width">
                <div class="chart-title">Commit Size Distribution</div>
                <canvas id="sizeDistChart"></canvas>
            </div>
        </div>
    </div>

    <script>
        const rawData = {{ data | json_encode() | safe }};
        
        // --- Pre-process Data ---
        // Convert ISO dates strings to Date objects and extract YYYY-MM-DD
        const data = rawData.map(d => {
            const dateObj = new Date(d.date);
            return {
                ...d,
                dateObj: dateObj,
                dateStr: dateObj.toISOString().split('T')[0],
                dayOfWeek: dateObj.getDay(), // 0=Sun, 1=Mon...
                hour: dateObj.getHours(),
                total_changes: d.added + d.deleted,
                commit_count: 1, // Base count for every commit
                is_merge: d.is_merge || false
            };
        });

        const ctx = document.getElementById('productivityChart').getContext('2d');
        const pieCtx = document.getElementById('shareChart').getContext('2d');
        const dowCtx = document.getElementById('dayOfWeekChart').getContext('2d');
        const heatmapCtx = document.getElementById('heatmapChart').getContext('2d');
        const sizeCtx = document.getElementById('sizeDistChart').getContext('2d');

        let mainChart, pieChart, dowChart, heatmapChart, sizeChart;

        const allUsers = [...new Set(data.map(d => d.author))]; // Note: field is 'author' in CommitStats
        const allDates = [...new Set(data.map(d => d.dateStr))].sort();

        if (allDates.length > 0) {
            document.getElementById('startDate').value = allDates[0];
            document.getElementById('endDate').value = allDates[allDates.length - 1];
        }

        function stringToColor(str) {
            let hash = 0;
            for (let i = 0; i < str.length; i++) {
                hash = str.charCodeAt(i) + ((hash << 5) - hash);
            }
            const c = (hash & 0x00FFFFFF).toString(16).toUpperCase();
            return '#' + '00000'.substring(0, 6 - c.length) + c;
        }

        // Helper: Moving Average
        function calculateMovingAverage(values, windowSize) {
            const result = [];
            for (let i = 0; i < values.length; i++) {
                const start = Math.max(0, i - windowSize + 1);
                const subset = values.slice(start, i + 1);
                const sum = subset.reduce((a, b) => a + b, 0);
                result.push(sum / subset.length);
            }
            return result;
        }

        function updateDashboard() {
            const metric = document.getElementById('metricSelect').value;
            const chartType = document.getElementById('chartTypeSelect').value;
            const startDate = document.getElementById('startDate').value;
            const endDate = document.getElementById('endDate').value;
            const showTrend = document.getElementById('showTrend').checked;

            // Filter data by date range
            const filteredData = data.filter(d => d.dateStr >= startDate && d.dateStr <= endDate);
            
            // --- Summary Cards ---
            updateSummary(filteredData, metric, startDate, endDate);

            // --- 1. Main Timeline Chart ---
            updateTimelineChart(filteredData, metric, chartType, showTrend, startDate, endDate);

            // --- 2. Pie Chart (Share) ---
            updatePieChart(filteredData, metric);

            // --- 3. Day of Week Chart ---
            updateDayOfWeekChart(filteredData, metric);

            // --- 4. Heatmap (Bubble) ---
            updateHeatmapChart(filteredData, metric);

            // --- 5. Size Distribution ---
            updateSizeDistChart(filteredData);
        }

        function updateSummary(currentData, metric, startDate, endDate) {
            const currentTotal = currentData.reduce((acc, d) => acc + d[metric], 0);
            const mergeTotal = currentData.filter(d => d.is_merge).length;
            const activeDays = new Set(currentData.map(d => d.dateStr)).size;
            const avgPerDay = activeDays > 0 ? (currentTotal / activeDays).toFixed(1) : 0;

            // Previous period calc
            const start = new Date(startDate);
            const end = new Date(endDate);
            const duration = end - start;
            const prevEnd = new Date(start.getTime() - 86400000);
            const prevStart = new Date(prevEnd.getTime() - duration);
            const prevStartStr = prevStart.toISOString().split('T')[0];
            const prevEndStr = prevEnd.toISOString().split('T')[0];
            
            const prevData = data.filter(d => d.dateStr >= prevStartStr && d.dateStr <= prevEndStr);
            const prevTotal = prevData.reduce((acc, d) => acc + d[metric], 0);

            let titleFormatted = metric.replace('_', ' ');
            if (metric === 'commit_count') titleFormatted = 'Total Commits';
            
            document.getElementById('summaryTitle').textContent = titleFormatted;
            document.getElementById('summaryValue').textContent = currentTotal.toLocaleString();
            document.getElementById('mergeCommitsValue').textContent = mergeTotal.toLocaleString();
            document.getElementById('activeDaysValue').textContent = activeDays;
            document.getElementById('avgPerDayValue').textContent = Number(avgPerDay).toLocaleString();

            const diffEl = document.getElementById('summaryDiff');
            if (prevTotal === 0) {
                diffEl.textContent = currentTotal > 0 ? "New Activity" : "-";
                diffEl.className = "diff neutral";
            } else {
                const change = ((currentTotal - prevTotal) / prevTotal) * 100;
                const sign = change >= 0 ? "+" : "";
                const icon = change > 0 ? "🔼" : (change < 0 ? "🔽" : "➖");
                diffEl.textContent = `${sign}${change.toFixed(1)}% vs prev ${icon}`;
                diffEl.className = `diff ${change >= 0 ? 'positive' : 'negative'}`;
            }
        }

        function updateTimelineChart(filteredData, metric, chartType, showTrend, startDate, endDate) {
            // Aggregate by date and user
            // We need to generate a dense list of dates between start and end
            const dateMap = new Map();
            let curr = new Date(startDate);
            const end = new Date(endDate);
            const displayDates = [];
            while (curr <= end) {
                const dStr = curr.toISOString().split('T')[0];
                displayDates.push(dStr);
                dateMap.set(dStr, {});
                curr.setDate(curr.getDate() + 1);
            }

            filteredData.forEach(d => {
                if (!dateMap.has(d.dateStr)) return;
                const daily = dateMap.get(d.dateStr);
                
                // If metric is commit_count, we might want to split regular vs merge (optional)
                // For now, let's keep it simple: aggregated by user as before.
                // The prompt asked to distinguish them. 
                // Let's try to add a 'Merge Commits' dataset if metric is commit_count?
                // Or maybe just normal aggregation. The user said "distinguish merge commits".
                // Since we stack by user, adding "Merge" as a separate category is tricky if we want to keep user breakdown.
                // Alternative: When 'commit_count' is selected, maybe we shouldn't break down by user but by Type (Regular vs Merge)?
                // But the user breakdown is a core feature.
                // Let's keep User Breakdown for now, but maybe add (Merge) in tooltip?
                
                // Let's stick to standard user breakdown for the timeline for consistency.
                // The summary card already shows total merges.
                daily[d.author] = (daily[d.author] || 0) + d[metric];
            });

            const datasets = allUsers.map(user => {
                return {
                    label: user,
                    data: displayDates.map(date => dateMap.get(date)[user] || 0),
                    fill: chartType === 'bar',
                    borderColor: stringToColor(user),
                    backgroundColor: stringToColor(user),
                    tension: 0.1,
                    borderWidth: chartType === 'bar' ? 0 : 2,
                    order: 2
                };
            });

            if (showTrend) {
                const dailyTotals = displayDates.map(date => {
                    const daily = dateMap.get(date);
                    return Object.values(daily).reduce((a, b) => a + b, 0);
                });
                const trend = calculateMovingAverage(dailyTotals, 7);
                datasets.push({
                    label: '7-Day Trend',
                    data: trend,
                    borderColor: '#34495e',
                    borderWidth: 2,
                    borderDash: [5, 5],
                    pointRadius: 0,
                    fill: false,
                    type: 'line',
                    order: 1
                });
            }

            if (mainChart) mainChart.destroy();
            mainChart = new Chart(ctx, {
                type: chartType,
                data: { labels: displayDates, datasets: datasets },
                options: {
                    responsive: true,
                    maintainAspectRatio: false,
                    scales: {
                        x: { stacked: chartType === 'bar' },
                        y: { stacked: chartType === 'bar', beginAtZero: true }
                    },
                    plugins: { tooltip: { mode: 'index', intersect: false } },
                    interaction: { mode: 'nearest', axis: 'x', intersect: false }
                }
            });
        }

        function updatePieChart(filteredData, metric) {
            const userTotals = {};
            filteredData.forEach(d => {
                userTotals[d.author] = (userTotals[d.author] || 0) + d[metric];
            });

            const labels = Object.keys(userTotals);
            const values = Object.values(userTotals);

            if (pieChart) pieChart.destroy();
            pieChart = new Chart(pieCtx, {
                type: 'doughnut',
                data: {
                    labels: labels,
                    datasets: [{
                        data: values,
                        backgroundColor: labels.map(u => stringToColor(u))
                    }]
                },
                options: {
                    responsive: true,
                    maintainAspectRatio: false,
                    plugins: {
                        legend: { position: 'right' },
                        tooltip: {
                            callbacks: {
                                label: function(ctx) {
                                    const val = ctx.raw;
                                    const total = ctx.chart.data.datasets[0].data.reduce((a, b) => a + b, 0);
                                    const pct = total > 0 ? ((val / total) * 100).toFixed(1) : 0;
                                    return `${ctx.label}: ${val} (${pct}%)`;
                                }
                            }
                        }
                    }
                }
            });
        }

        function updateDayOfWeekChart(filteredData, metric) {
            const days = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];
            const dayTotals = new Array(7).fill(0);

            filteredData.forEach(d => {
                dayTotals[d.dayOfWeek] += d[metric];
            });

            if (dowChart) dowChart.destroy();
            dowChart = new Chart(dowCtx, {
                type: 'bar',
                data: {
                    labels: days,
                    datasets: [{
                        label: 'Activity by Day',
                        data: dayTotals,
                        backgroundColor: 'rgba(52, 152, 219, 0.7)',
                        borderColor: 'rgba(52, 152, 219, 1)',
                        borderWidth: 1
                    }]
                },
                options: {
                    responsive: true,
                    maintainAspectRatio: false,
                    scales: { y: { beginAtZero: true } }
                }
            });
        }

        function updateHeatmapChart(filteredData, metric) {
            // Bubble chart: x=Hour, y=Day, r=Value
            const buckets = {}; // key: "day-hour"
            
            filteredData.forEach(d => {
                const key = `${d.dayOfWeek}-${d.hour}`;
                buckets[key] = (buckets[key] || 0) + d[metric];
            });

            const dataPoints = [];
            const days = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];
            
            // Normalize radius
            const maxVal = Math.max(...Object.values(buckets), 0);
            
            for (let day = 0; day < 7; day++) {
                for (let hour = 0; hour < 24; hour++) {
                    const val = buckets[`${day}-${hour}`] || 0;
                    if (val > 0) {
                        // Radius scaling: min 2, max 20
                        const r = maxVal > 0 ? Math.sqrt(val / maxVal) * 15 + 2 : 0;
                        dataPoints.push({ x: hour, y: day, r: r, v: val });
                    }
                }
            }

            if (heatmapChart) heatmapChart.destroy();
            heatmapChart = new Chart(heatmapCtx, {
                type: 'bubble',
                data: {
                    datasets: [{
                        label: 'Activity',
                        data: dataPoints,
                        backgroundColor: 'rgba(231, 76, 60, 0.6)',
                        borderColor: 'rgba(231, 76, 60, 1)'
                    }]
                },
                options: {
                    responsive: true,
                    maintainAspectRatio: false,
                    scales: {
                        x: { 
                            min: -0.5, max: 23.5, 
                            ticks: { stepSize: 1 }, 
                            title: { display: true, text: 'Hour of Day' }
                        },
                        y: { 
                            min: -0.5, max: 6.5, 
                            ticks: { 
                                callback: function(val) { return days[val]; },
                                stepSize: 1
                            },
                            reverse: true
                        }
                    },
                    plugins: {
                        tooltip: {
                            callbacks: {
                                label: function(ctx) {
                                    const d = ctx.raw;
                                    return `${days[d.y]} ${d.x}:00 - ${d.v}`;
                                }
                            }
                        }
                    }
                }
            });
        }

        function updateSizeDistChart(filteredData) {
            // Bins for commit size (total_changes): 0-10, 11-50, 51-100, 101-500, 500+
            const bins = [0, 0, 0, 0, 0];
            const labels = ['XS (0-10)', 'S (11-50)', 'M (51-100)', 'L (101-500)', 'XL (500+)'];

            filteredData.forEach(d => {
                const total = d.added + d.deleted; // Always use total changes for size
                if (total <= 10) bins[0]++;
                else if (total <= 50) bins[1]++;
                else if (total <= 100) bins[2]++;
                else if (total <= 500) bins[3]++;
                else bins[4]++;
            });

            if (sizeChart) sizeChart.destroy();
            sizeChart = new Chart(sizeCtx, {
                type: 'bar',
                data: {
                    labels: labels,
                    datasets: [{
                        label: 'Commit Count',
                        data: bins,
                        backgroundColor: [
                            '#2ecc71', '#3498db', '#f1c40f', '#e67e22', '#e74c3c'
                        ]
                    }]
                },
                options: {
                    responsive: true,
                    maintainAspectRatio: false,
                    scales: { y: { beginAtZero: true } },
                    plugins: {
                        legend: { display: false },
                        title: { display: true, text: 'Commit Size (Lines Changed)' }
                    }
                }
            });
        }
        
        // Initial render
        updateDashboard();
    </script>
</body>
</html>
"#;



