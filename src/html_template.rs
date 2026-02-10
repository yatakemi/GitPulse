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
        .chart-title { position: absolute; top: 15px; left: 20px; font-size: 16px; font-weight: 600; color: #34495e; z-index: 10; display: flex; align-items: center; gap: 8px; }
        
        /* Tooltip Styles */
        .info-icon {
            display: inline-flex; justify-content: center; align-items: center;
            width: 18px; height: 18px; border-radius: 50%; background: #bdc3c7; color: white;
            font-size: 12px; font-weight: bold; cursor: help; position: relative;
        }
        .info-icon:hover { background: #3498db; }
        .info-icon:hover::after {
            content: attr(data-tooltip);
            position: absolute; bottom: 100%; left: 50%; transform: translateX(-50%);
            background: #34495e; color: white; padding: 8px 12px; border-radius: 6px;
            font-size: 12px; font-weight: 400; white-space: pre-wrap; width: 250px; text-align: left;
            box-shadow: 0 4px 6px rgba(0,0,0,0.1); z-index: 100; margin-bottom: 8px;
            line-height: 1.4;
        }
        .info-icon:hover::before {
            content: ''; position: absolute; bottom: 100%; left: 50%; margin-left: -5px; margin-bottom: 3px;
            border-width: 5px; border-style: solid; border-color: #34495e transparent transparent transparent;
        }
        
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
                    <option value="churn">Code Churn (Volatility)</option>
                </select>
            </div>
            
            <!-- ... existing controls ... -->

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
            <!-- ... existing cards ... -->
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
                <h3>Churn Rate</h3>
                <div class="value" id="churnRateValue">-</div>
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
                <div class="chart-title">
                    Timeline 
                    <span class="info-icon" data-tooltip="Shows activity trends over time. Look for spikes (sprints/releases) or gaps (blockers/downtime). Ideally, activity should be consistent.">i</span>
                </div>
                <canvas id="productivityChart"></canvas>
            </div>
            <div class="chart-box">
                <div class="chart-title">
                    User Share
                    <span class="info-icon" data-tooltip="Distribution of contributions. Helps identify 'Bus Factor' (reliance on single dev) or uneven workload distribution.">i</span>
                </div>
                <canvas id="shareChart"></canvas>
            </div>
            <div class="chart-box">
                <div class="chart-title">
                    Day of Week Activity
                    <span class="info-icon" data-tooltip="Weekly rhythm. Most teams peak Tue-Thu. High weekend activity might indicate crunch time or unhealthy work habits.">i</span>
                </div>
                <canvas id="dayOfWeekChart"></canvas>
            </div>
            <div class="chart-box full-width">
                <div class="chart-title">
                    Activity Heatmap (Hour vs Day)
                    <span class="info-icon" data-tooltip="Identifies core working hours. Look for clusters outside normal hours (e.g. late nights), which suggests overtime or burnout risk.">i</span>
                </div>
                <canvas id="heatmapChart"></canvas>
            </div>
             <div class="chart-box full-width">
                <div class="chart-title">
                    Commit Size Distribution
                    <span class="info-icon" data-tooltip="Breakdown of commit sizes. 'XS'/'S' are ideal (atomic commits). Too many 'XL' suggests large, risky changes that are hard to review.">i</span>
                </div>
                <canvas id="sizeDistChart"></canvas>
            </div>
            <div class="chart-box full-width">
                <div class="chart-title">
                    File Hotspots (Top 20 Modified)
                    <span class="info-icon" data-tooltip="Most frequently changed files. These are potential architectural bottlenecks, 'God Classes', or unstable modules needing refactoring.">i</span>
                </div>
                <canvas id="hotspotsChart"></canvas>
            </div>
            <div class="chart-box full-width">
                <div class="chart-title">
                    Est. Daily Work Duration
                    <span class="info-icon" data-tooltip="Time between first and last commit of the day. NOTE: Not actual work hours, but indicates span of activity. Long spans may suggest burnout.">i</span>
                </div>
                <canvas id="workDurationChart"></canvas>
            </div>
             <div class="chart-box full-width">
                <div class="chart-title">
                    Team Health Trends
                    <span class="info-icon" data-tooltip="Red: Churn Rate (Rework/Volatility). High = Unstable/Refactoring.
Purple: Avg Duration. Rising trend = Potential Overwork.">i</span>
                </div>
                <canvas id="healthTrendChart"></canvas>
            </div>
        </div>
    </div>

    <script>
        const reportData = {{ data | json_encode() | safe }};
        const rawCommits = reportData.commits;
        const filePaths = reportData.file_paths;
        
        // ... (existing preprocessing) ...
        const data = rawCommits.map(d => {
            const dateObj = new Date(d.date);
            const churn = (d.added + d.deleted) - Math.abs(d.added - d.deleted);
            return {
                ...d,
                dateObj: dateObj,
                dateStr: dateObj.toISOString().split('T')[0],
                dayOfWeek: dateObj.getDay(),
                hour: dateObj.getHours(),
                total_changes: d.added + d.deleted,
                commit_count: 1,
                is_merge: d.is_merge || false,
                files: d.files || [],
                churn: churn
            };
        });

        const ctx = document.getElementById('productivityChart').getContext('2d');
        const pieCtx = document.getElementById('shareChart').getContext('2d');
        const dowCtx = document.getElementById('dayOfWeekChart').getContext('2d');
        const heatmapCtx = document.getElementById('heatmapChart').getContext('2d');
        const sizeCtx = document.getElementById('sizeDistChart').getContext('2d');
        const hotCtx = document.getElementById('hotspotsChart').getContext('2d');
        const durCtx = document.getElementById('workDurationChart').getContext('2d');
        const healthCtx = document.getElementById('healthTrendChart').getContext('2d');

        let mainChart, pieChart, dowChart, heatmapChart, sizeChart, hotChart, durChart, healthChart;

        // ... (existing helper vars and functions) ...
        const allUsers = [...new Set(data.map(d => d.author))];
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

        function calculateMovingAverage(values, windowSize) {
           // ... (existing implementation) ...
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
            // ... (existing calls) ...
            const metric = document.getElementById('metricSelect').value;
            const chartType = document.getElementById('chartTypeSelect').value;
            const startDate = document.getElementById('startDate').value;
            const endDate = document.getElementById('endDate').value;
            const showTrend = document.getElementById('showTrend').checked;

            const filteredData = data.filter(d => d.dateStr >= startDate && d.dateStr <= endDate);
            
            updateSummary(filteredData, metric, startDate, endDate);
            updateTimelineChart(filteredData, metric, chartType, showTrend, startDate, endDate);
            updatePieChart(filteredData, metric);
            updateDayOfWeekChart(filteredData, metric);
            updateHeatmapChart(filteredData, metric);
            updateSizeDistChart(filteredData);
            updateHotspotsChart(filteredData);
            updateWorkDurationChart(filteredData);
            updateHealthTrendChart(filteredData, startDate, endDate);
        }

        // ... (existing updateSummary, updateTimelineChart, etc.) ...

        function updateHealthTrendChart(filteredData, startDate, endDate) {
            // Generate dense date list
            const dateMap = new Map(); // date -> { total_changes: 0, total_churn: 0, user_stats: { user: { min: 24, max: 0 } } }
            let curr = new Date(startDate);
            const end = new Date(endDate);
            const displayDates = [];
            
            while (curr <= end) {
                const dStr = curr.toISOString().split('T')[0];
                displayDates.push(dStr);
                dateMap.set(dStr, { 
                    total_changes: 0, 
                    total_churn: 0,
                    user_times: {} 
                });
                curr.setDate(curr.getDate() + 1);
            }

            filteredData.forEach(d => {
                if (!dateMap.has(d.dateStr)) return;
                const daily = dateMap.get(d.dateStr);
                
                daily.total_changes += d.total_changes;
                daily.total_churn += d.churn;
                
                if (!daily.user_times[d.author]) daily.user_times[d.author] = { min: 24, max: 0, count: 0 };
                const uStats = daily.user_times[d.author];
                if (d.hour < uStats.min) uStats.min = d.hour;
                if (d.hour > uStats.max) uStats.max = d.hour;
                uStats.count++;
            });

            // Calculate metrics per day
            const churnRates = [];
            const avgDurations = [];

            displayDates.forEach(date => {
                const stats = dateMap.get(date);
                
                // 1. Churn Rate
                const rate = stats.total_changes > 0 
                    ? (stats.total_churn / stats.total_changes) * 100 
                    : 0;
                churnRates.push(rate);

                // 2. Avg Duration
                let totalDuration = 0;
                let activeUsers = 0;
                Object.values(stats.user_times).forEach(u => {
                    if (u.count > 1) { // Needs start and end
                        totalDuration += (u.max - u.min);
                        activeUsers++;
                    }
                });
                const avgDur = activeUsers > 0 ? (totalDuration / activeUsers) : 0;
                avgDurations.push(avgDur);
            });

            // 7-day MA for smoother trends
            const churnTrend = calculateMovingAverage(churnRates, 7);
            const durTrend = calculateMovingAverage(avgDurations, 7);

            if (healthChart) healthChart.destroy();
            healthChart = new Chart(healthCtx, {
                type: 'line',
                data: {
                    labels: displayDates,
                    datasets: [
                        {
                            label: 'Churn Rate (%)',
                            data: churnTrend,
                            borderColor: '#e74c3c', // Red
                            backgroundColor: 'rgba(231, 76, 60, 0.1)',
                            yAxisID: 'y',
                            tension: 0.4,
                            pointRadius: 0,
                            borderWidth: 2,
                            fill: true
                        },
                        {
                            label: 'Avg Work Duration (Hours)',
                            data: durTrend,
                            borderColor: '#8e44ad', // Purple
                            backgroundColor: 'rgba(142, 68, 173, 0.1)',
                            yAxisID: 'y1',
                            tension: 0.4,
                            pointRadius: 0,
                            borderWidth: 2,
                            fill: true
                        }
                    ]
                },
                options: {
                    responsive: true,
                    maintainAspectRatio: false,
                    interaction: {
                        mode: 'index',
                        intersect: false,
                    },
                    scales: {
                        x: { display: true },
                        y: {
                            type: 'linear',
                            display: true,
                            position: 'left',
                            title: { display: true, text: 'Churn Rate (%)' },
                            beginAtZero: true,
                            max: 100
                        },
                        y1: {
                            type: 'linear',
                            display: true,
                            position: 'right',
                            title: { display: true, text: 'Hours' },
                            grid: { drawOnChartArea: false },
                            beginAtZero: true,
                            max: 24
                        }
                    },
                    plugins: {
                        tooltip: {
                            callbacks: {
                                label: function(context) {
                                    let label = context.dataset.label || '';
                                    if (label) {
                                        label += ': ';
                                    }
                                    if (context.parsed.y !== null) {
                                        label += context.parsed.y.toFixed(1);
                                    }
                                    return label;
                                }
                            }
                        }
                    }
                }
            });
        }

        function updateSummary(currentData, metric, startDate, endDate) {
            const currentTotal = currentData.reduce((acc, d) => acc + d[metric], 0);
            const mergeTotal = currentData.filter(d => d.is_merge).length;
            const activeDays = new Set(currentData.map(d => d.dateStr)).size;
            const avgPerDay = activeDays > 0 ? (currentTotal / activeDays).toFixed(1) : 0;
            
            // Calculate Global Churn Rate: (Total Churn / Total Changes) * 100
            const totalChanges = currentData.reduce((acc, d) => acc + d.total_changes, 0);
            const totalChurn = currentData.reduce((acc, d) => acc + d.churn, 0);
            const churnRate = totalChanges > 0 ? ((totalChurn / totalChanges) * 100).toFixed(1) : 0;

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
            if (metric === 'churn') titleFormatted = 'Total Code Churn';
            
            document.getElementById('summaryTitle').textContent = titleFormatted;
            document.getElementById('summaryValue').textContent = currentTotal.toLocaleString();
            document.getElementById('mergeCommitsValue').textContent = mergeTotal.toLocaleString();
            document.getElementById('churnRateValue').textContent = `${churnRate}%`;
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
        
        // ... (Charts 1-5 same as before) ...
        
        function updateWorkDurationChart(filteredData) {
            // Logic: For each user AND day, find min(hour) and max(hour).
            // Duration = max - min.
            
            const userDurations = {}; // { user: [duration1, duration2, ...] }
            const dailyStats = {}; // { "date-user": { min: 24, max: 0 } }
            
            filteredData.forEach(d => {
                const key = `${d.dateStr}-${d.author}`;
                if (!dailyStats[key]) dailyStats[key] = { min: 24, max: 0, count: 0 };
                
                if (d.hour < dailyStats[key].min) dailyStats[key].min = d.hour;
                if (d.hour > dailyStats[key].max) dailyStats[key].max = d.hour;
                dailyStats[key].count++;
            });
            
            Object.keys(dailyStats).forEach(key => {
                const stat = dailyStats[key];
                if (stat.count > 1) { // Need at least 2 commits to define a duration
                    const user = key.split('-').slice(3).join('-'); // Hacky split? No, dateStr is YYYY-MM-DD (2 dashes)
                    // Better: loop through dailyStats keys is risky if username has dashes.
                    // Let's reconstruct.
                }
            });
            
            // Clean approach:
            const userDailyMap = new Map();
            filteredData.forEach(d => {
                if (!userDailyMap.has(d.author)) userDailyMap.set(d.author, new Map());
                const userDates = userDailyMap.get(d.author);
                
                if (!userDates.has(d.dateStr)) userDates.set(d.dateStr, { min: 24, max: 0, count: 0 });
                const dayStat = userDates.get(d.dateStr);
                
                if (d.hour < dayStat.min) dayStat.min = d.hour;
                if (d.hour > dayStat.max) dayStat.max = d.hour;
                dayStat.count++;
            });
            
            // Buckets: 0-1h, 1-4h, 4-8h, 8h+
            const labels = ['< 1h', '1-4h', '4-8h', '8h+'];
            const datasets = [];
            
            allUsers.forEach(user => {
                if (!userDailyMap.has(user)) return;
                const stats = userDailyMap.get(user);
                const bins = [0, 0, 0, 0];
                
                stats.forEach(dayStat => {
                    if (dayStat.count < 2) return; // Single commit has 0 duration, usually not meaningful 'work session'
                    const duration = dayStat.max - dayStat.min;
                    
                    if (duration < 1) bins[0]++;
                    else if (duration < 4) bins[1]++;
                    else if (duration < 8) bins[2]++;
                    else bins[3]++;
                });
                
                // Only add if there is data
                if (bins.reduce((a,b)=>a+b,0) > 0) {
                     datasets.push({
                        label: user,
                        data: bins,
                        backgroundColor: stringToColor(user),
                        stack: 'stack1'
                    });
                }
            });
            
             if (durChart) durChart.destroy();
             durChart = new Chart(durCtx, {
                type: 'bar',
                data: {
                    labels: labels,
                    datasets: datasets
                },
                options: {
                    responsive: true,
                    maintainAspectRatio: false,
                    scales: {
                        x: { stacked: true },
                        y: { stacked: true, beginAtZero: true, title: { display: true, text: 'Days Count' } }
                    },
                     plugins: {
                        title: { display: true, text: 'Distribution of Daily Work Hours (First to Last Commit)' }
                    }
                }
            });
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

        function updateHotspotsChart(filteredData) {
            const fileCounts = {};
            
            filteredData.forEach(d => {
                d.files.forEach(fileIdx => {
                    const path = filePaths[fileIdx];
                    fileCounts[path] = (fileCounts[path] || 0) + 1;
                });
            });
            
            // Sort by count desc
            const sortedFiles = Object.entries(fileCounts).sort((a, b) => b[1] - a[1]).slice(0, 20);
            const labels = sortedFiles.map(e => e[0]);
            const values = sortedFiles.map(e => e[1]);
            
            if (hotChart) hotChart.destroy();
            hotChart = new Chart(hotCtx, {
                type: 'bar',
                data: {
                    labels: labels,
                    datasets: [{
                        label: 'Modification Count',
                        data: values,
                        backgroundColor: 'rgba(155, 89, 182, 0.7)',
                        borderColor: 'rgba(155, 89, 182, 1)',
                        borderWidth: 1
                    }]
                },
                options: {
                    indexAxis: 'y',
                    responsive: true,
                    maintainAspectRatio: false,
                    scales: {
                        x: { beginAtZero: true }
                    },
                    plugins: {
                         legend: { display: false }
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



