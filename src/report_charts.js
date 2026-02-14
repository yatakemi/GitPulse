
function updateFileTypeChart() {
    const startDate = document.getElementById('startDate').value;
    const endDate = document.getElementById('endDate').value;
    const extMap = {};
    let processedCommits = 0;
    let commitsWithFiles = 0;

    // Use RAW commits instead of aggregated daily stats to access file info
    const filteredCommits = dashboardData.commits.filter(c => {
        const date = c.date.split('T')[0];
        return date >= startDate && date <= endDate && selectedUsers.has(normalizeAuthor(c.author));
    });

    filteredCommits.forEach(c => {
        processedCommits++;
        const total = (c.added || 0) + (c.deleted || 0);
        const churn = total - Math.abs((c.added || 0) - (c.deleted || 0));

        const commitExts = new Set();
        if (c.files && Array.isArray(c.files) && c.files.length > 0) {
            commitsWithFiles++;
            c.files.forEach(fidx => {
                const path = filePaths[fidx];
                if (path && typeof path === 'string') {
                    const pathLower = path.toLowerCase();
                    const filename = path.split('/').pop() || "";

                    // Detect test files
                    if (pathLower.includes('/test/') || pathLower.includes('/tests/') ||
                        filename.includes('.spec.') || filename.includes('.test.') ||
                        filename.endsWith('_test.rs') || filename.endsWith('_spec.rb')) {
                        commitExts.add('test');
                        return;
                    }

                    const lastDotIndex = filename.lastIndexOf('.');
                    if (lastDotIndex > 0 && lastDotIndex < filename.length - 1) {
                        const ext = filename.substring(lastDotIndex + 1).toLowerCase();
                        if (ext.length <= 15) commitExts.add(ext);
                        else commitExts.add('others');
                    } else if (filename.startsWith('.')) {
                        commitExts.add('config');
                    } else {
                        commitExts.add('no-ext');
                    }
                }
            });
        }

        if (commitExts.size === 0 && total > 0) {
            commitExts.add('others');
        }

        if (commitExts.size === 0) return;

        const linesPerExt = Math.round(total / commitExts.size);
        const churnPerExt = Math.round(churn / commitExts.size);

        commitExts.forEach(ext => {
            if (!extMap[ext]) extMap[ext] = { ext, added: 0, deleted: 0, churn: 0, commits: 0 };
            if (total > 0) {
                const ratio = (c.added || 0) / total;
                extMap[ext].added += Math.round(linesPerExt * ratio);
                extMap[ext].deleted += Math.round(linesPerExt * (1 - ratio));
            }
            extMap[ext].churn += churnPerExt;
            extMap[ext].commits += 1 / commitExts.size;
        });
    });

    const sortedExts = Object.values(extMap).sort((a, b) => {
        const totalA = a.added + a.deleted;
        const totalB = b.added + b.deleted;
        if (totalA !== totalB) return totalB - totalA;
        return b.commits - a.commits;
    });

    const topExts = sortedExts.slice(0, 15);

    if (fileTypeChart) fileTypeChart.destroy();

    const tbody = document.getElementById('fileTypeTableBody');
    if (topExts.length === 0) {
        if (tbody) tbody.innerHTML = `<tr><td colspan="4" style="text-align: center; color: #7f8c8d; padding: 20px;">No file data found.<br><small>(Processed ${processedCommits} entries)</small></td></tr>`;
        return;
    }
    if (fileTypeCtx) {
        fileTypeChart = new Chart(fileTypeCtx, {
            type: 'doughnut',
            data: {
                labels: topExts.map(e => e.ext),
                datasets: [{
                    data: topExts.map(e => e.added + e.deleted),
                    backgroundColor: topExts.map(e => stringToColor(e.ext))
                }]
            },
            options: { responsive: true, maintainAspectRatio: false, plugins: { legend: { position: 'right' } } }
        });
    } else {
        fileTypeChart = null;
    }

    if (tbody) {
        tbody.innerHTML = '';
        topExts.forEach(e => {
            const tr = document.createElement('tr');
            const total = e.added + e.deleted;
            const churnRate = total > 0 ? (e.churn / total * 100).toFixed(1) : '0.0';
            tr.innerHTML = `
                        <td><strong>.${e.ext}</strong></td>
                        <td class="badge added">+${e.added.toLocaleString()}</td>
                        <td class="badge deleted">-${e.deleted.toLocaleString()}</td>
                        <td>${churnRate}%</td>
                    `;
            tbody.appendChild(tr);
        });
    }
}

function updateReviewActivityChart(startDate, endDate) {
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

    if (dashboardData.github_prs) {
        dashboardData.github_prs.forEach(pr => {
            if (pr.review_comments) {
                pr.review_comments.forEach(comm => {
                    const date = comm.created_at.split('T')[0];
                    if (dateMap.has(date)) {
                        const norm = normalizeAuthor(comm.user);
                        if (selectedUsers.has(norm)) {
                            const daily = dateMap.get(date);
                            daily[norm] = (daily[norm] || 0) + 1;
                        }
                    }
                });
            }
        });
    }

    const datasets = Array.from(selectedUsers).map(user => ({
        label: user,
        data: displayDates.map(date => dateMap.get(date)[user] || 0),
        borderColor: stringToColor(user),
        backgroundColor: stringToColor(user) + '33',
        tension: 0.1,
        fill: false
    }));

    if (reviewActivityChart) reviewActivityChart.destroy();
    if (reviewActivityCtx) {
        reviewActivityChart = new Chart(reviewActivityCtx, {
            type: 'line',
            data: { labels: displayDates, datasets },
            options: {
                responsive: true,
                maintainAspectRatio: false,
                scales: {
                    y: {
                        beginAtZero: true,
                        ticks: { stepSize: 1 },
                        title: { display: true, text: 'Comments Given' }
                    }
                }
            }
        });
    } else {
        reviewActivityChart = null;
    }
}

function updateGitHubAdvancedMetrics(startDate, endDate) {
    const githubDiv = document.getElementById('githubAdvancedSummary');
    const reciprocityBox = document.getElementById('reciprocityBox');
    const scatterBox = document.getElementById('scatterBox');

    if (!dashboardData.github_prs || dashboardData.github_prs.length === 0) {
        if (githubDiv) githubDiv.style.display = 'none';
        if (reciprocityBox) reciprocityBox.style.display = 'none';
        if (scatterBox) scatterBox.style.display = 'none';
        return;
    }

    if (githubDiv) githubDiv.style.display = 'flex';
    if (reciprocityBox) reciprocityBox.style.display = 'block';
    if (scatterBox) scatterBox.style.display = 'block';

    const filteredPRs = dashboardData.github_prs.filter(pr => {
        const date = pr.created_at.split('T')[0];
        return date >= startDate && date <= endDate && selectedUsers.has(normalizeAuthor(pr.author));
    });

    if (filteredPRs.length === 0) return;

    // 1. Stats Calculation
    const reworkPRs = [];
    const allComments = [];
    const responseTimes = [];
    const leadTimes = [];
    const iterationCounts = [];

    const matrix = {}; // {author: {reviewer: count}}
    const scatterData = [];

    filteredPRs.forEach(pr => {
        const author = normalizeAuthor(pr.author);

        // Review Depth (Use our specifically collected review comments/threads)
        const depth = (pr.review_comments && pr.review_comments.length) || 0;
        allComments.push(depth);

        // Response Time
        const startStr = pr.first_assigned_at || pr.created_at;
        if (startStr && pr.reviews && pr.reviews.length > 0) {
            const humanReviews = pr.reviews
                .filter(r => !isBot(r.user))
                .sort((a, b) => a.submitted_at.localeCompare(b.submitted_at));

            if (humanReviews.length > 0) {
                const startTime = new Date(startStr);
                const firstResponseTime = new Date(humanReviews[0].submitted_at);
                const diff = (firstResponseTime - startTime) / (1000 * 60 * 60);
                if (diff >= 0) responseTimes.push(diff);
            }
        }

        // Iterations: Count total human review submissions (not just unique days)
        const humanReviews = pr.reviews ? pr.reviews.filter(r => !isBot(r.user)) : [];
        const iterations = Math.max(1, humanReviews.length);
        iterationCounts.push(iterations);

        // Rework Rate (Adjusted: Changes Requested OR >1 Iteration)
        const hasRequestChanges = pr.reviews && pr.reviews.some(r => r.state === 'CHANGES_REQUESTED' && !isBot(r.user));
        if (hasRequestChanges || iterations > 1) reworkPRs.push(pr);

        // Lead Time
        if (pr.merged_at) {
            const lt = (new Date(pr.merged_at) - new Date(pr.created_at)) / (1000 * 60 * 60 * 24);
            if (lt > 0) leadTimes.push(lt);
        }

        // Matrix Data
        if (!matrix[author]) matrix[author] = {};
        if (pr.reviews) {
            pr.reviews.forEach(r => {
                const reviewer = normalizeAuthor(r.user);
                if (reviewer !== author && selectedUsers.has(reviewer)) {
                    matrix[author][reviewer] = (matrix[author][reviewer] || 0) + 1;
                }
            });
        }

        // Scatter Data
        if (pr.merged_at) {
            const lt = (new Date(pr.merged_at) - new Date(pr.created_at)) / (1000 * 60 * 60 * 24);
            const size = (pr.additions || 0) + (pr.deletions || 0);
            if (lt > 0) {
                scatterData.push({ x: Math.max(1, size), y: lt, label: pr.title });
            }
        }
    });

    const resStats = getDetailedStats(responseTimes);
    const depthStats = getDetailedStats(allComments);
    const iterStats = getDetailedStats(iterationCounts);

    // Update Summary Cards
    const reworkRateEl = document.getElementById('reworkRateValue');
    const reviewDepthEl = document.getElementById('reviewDepthValue');
    const avgResponseEl = document.getElementById('avgResponseTimeValue');
    const avgIterEl = document.getElementById('avgIterationsValue');

    if (reworkRateEl) {
        reworkRateEl.textContent = ((reworkPRs.length / filteredPRs.length) * 100).toFixed(1) + '%';
    }
    if (reviewDepthEl) {
        reviewDepthEl.textContent = depthStats.avg.toFixed(1);
        reviewDepthEl.title = `Median: ${depthStats.median.toFixed(1)}, Min: ${depthStats.min}, Max: ${depthStats.max}`;
    }
    if (avgResponseEl) {
        avgResponseEl.textContent = resStats.avg.toFixed(1) + 'h';
        avgResponseEl.title = `Median: ${resStats.median.toFixed(1)}h, Min: ${resStats.min.toFixed(1)}h, Max: ${resStats.max.toFixed(1)}h`;
    }
    if (avgIterEl) {
        avgIterEl.textContent = iterStats.avg.toFixed(1);
        avgIterEl.title = `Median: ${iterStats.median}, Min: ${iterStats.min}, Max: ${iterStats.max}`;
    }

    // Update Distribution Chart
    updateDistributionChart(responseTimes, leadTimes);

    // 2. Render Reciprocity Matrix
    const currentSelected = Array.from(selectedUsers).sort();
    const matrixData = [];
    currentSelected.forEach((author, i) => {
        currentSelected.forEach((reviewer, j) => {
            const val = (matrix[author] && matrix[author][reviewer]) ? matrix[author][reviewer] : 0;
            matrixData.push({ x: j, y: i, v: val });
        });
    });

    if (reciprocityChart) reciprocityChart.destroy();
    if (reciprocityCtx) {
        reciprocityChart = new Chart(reciprocityCtx, {
            type: 'matrix',
            data: {
                datasets: [{
                    label: 'Review Count',
                    data: matrixData,
                    backgroundColor: ctx => {
                        const v = ctx.dataset.data[ctx.dataIndex].v;
                        return `rgba(230, 126, 34, ${Math.min(v / 5, 1)})`;
                    },
                    width: ({ chart }) => chart.chartArea ? (chart.chartArea.width / currentSelected.length) - 1 : 0,
                    height: ({ chart }) => chart.chartArea ? (chart.chartArea.height / currentSelected.length) - 1 : 0
                }]
            },
            options: {
                responsive: true, maintainAspectRatio: false,
                plugins: {
                    legend: { display: false },
                    tooltip: {
                        callbacks: {
                            label: ctx => {
                                const d = ctx.raw;
                                return `${currentSelected[d.y]} (Author) <- ${currentSelected[d.x]} (Reviewer): ${d.v} reviews`;
                            }
                        }
                    }
                },
                scales: {
                    x: {
                        type: 'linear', min: 0, max: currentSelected.length - 1,
                        ticks: { stepSize: 1, callback: v => currentSelected[v] },
                        grid: { display: false },
                        title: { display: true, text: 'Reviewer' }
                    },
                    y: {
                        type: 'linear', min: 0, max: currentSelected.length - 1,
                        ticks: { stepSize: 1, callback: v => currentSelected[v] },
                        grid: { display: false },
                        title: { display: true, text: 'Author' }
                    }
                }
            }
        });
    } else {
        reciprocityChart = null;
    }

    // 3. Render Scatter Chart (Size vs Lead Time)
    if (scatterChart) scatterChart.destroy();
    if (scatterCtx) {
        scatterChart = new Chart(scatterCtx, {
            type: 'scatter',
            data: {
                datasets: [{
                    label: 'PRs',
                    data: scatterData,
                    backgroundColor: 'rgba(52, 152, 219, 0.6)',
                    pointRadius: 6
                }]
            },
            options: {
                responsive: true, maintainAspectRatio: false,
                plugins: {
                    tooltip: {
                        callbacks: {
                            label: ctx => `${ctx.raw.label}: ${ctx.raw.x} lines, ${ctx.raw.y.toFixed(1)} days`
                        }
                    }
                },
                scales: {
                    x: { title: { display: true, text: 'Size (Additions + Deletions)' }, type: 'logarithmic' },
                    y: { title: { display: true, text: 'Lead Time (Days)' }, beginAtZero: true }
                }
            }
        });
    } else {
        scatterChart = null;
    }

    // 4. Render Distribution Charts (Response Time & Lead Time)
    updateDistributionChart(responseTimes, leadTimes);
}

function updateSummary(currentData, metric, startDate, endDate) {
    const currentTotal = currentData.reduce((acc, d) => acc + d[metric], 0);
    const activeDays = new Set(currentData.map(d => d.dateStr)).size;
    const avgPerDay = activeDays > 0 ? (currentTotal / activeDays).toFixed(1) : 0;
    const totalChanges = currentData.reduce((acc, d) => acc + d.total_changes, 0);
    const totalChurn = currentData.reduce((acc, d) => acc + d.churn, 0);
    const totalMerges = currentData.reduce((acc, d) => acc + d.merges, 0);
    const churnRate = totalChanges > 0 ? ((totalChurn / totalChanges) * 100).toFixed(1) : '0.0';

    const metricSelect = document.getElementById('metricSelect');
    const metricLabel = metricSelect.options[metricSelect.selectedIndex].text;
    document.getElementById('summaryTitle').textContent = metricLabel;

    document.getElementById('summaryValue').textContent = (currentTotal || 0).toLocaleString();
    document.getElementById('mergeCommitsValue').textContent = (totalMerges || 0).toLocaleString();
    document.getElementById('churnRateValue').textContent = `${churnRate}%`;
    document.getElementById('activeDaysValue').textContent = activeDays || 0;
    document.getElementById('avgPerDayValue').textContent = Number(avgPerDay || 0).toLocaleString();
}

function updateTimelineChart(filteredData, metric, chartType, showTrend, startDate, endDate) {
    const dateMap = new Map();
    let curr = new Date(startDate);
    const end = new Date(endDate);
    const displayDates = [];

    // Update Title
    const metricSelect = document.getElementById('metricSelect');
    const metricLabel = metricSelect.options[metricSelect.selectedIndex].text;
    document.getElementById('timelineTitleText').textContent = t('chart_timeline') + ': ' + metricLabel;

    // Safety limit to 2 years of daily data
    let safety = 0;
    while (curr <= end && safety < 730) {
        const dStr = curr.toISOString().split('T')[0];
        displayDates.push(dStr);
        // Initialize based on metric type
        if (metric === 'lead_time') {
            dateMap.set(dStr, { 'Average': 0, _count: 0 }); // Special structure for average
        } else {
            dateMap.set(dStr, {});
        }
        curr.setDate(curr.getDate() + 1);
        safety++;
    }

    if (metric === 'lead_time') {
        const stats = dashboardData.daily_lead_time_stats || [];
        stats.forEach(s => {
            if (dateMap.has(s.date)) {
                // For lead time, we map 'Average' as the key to simplify reuse of logic or just hardcode dataset
                dateMap.get(s.date)['Average'] = s.avg_days;
            }
        });

        // For lead time, we just show one dataset (Average)
        const datasets = [{
            label: 'Average Lead Time',
            data: displayDates.map(date => dateMap.get(date)['Average'] || 0),
            fill: chartType === 'bar',
            borderColor: '#27ae60',
            backgroundColor: '#27ae6033',
            tension: 0.1,
            borderWidth: chartType === 'bar' ? 0 : 2
        }];

        if (showTrend) {
            datasets.push({
                label: '7-Day Trend',
                data: calculateMovingAverage(datasets[0].data, 7),
                borderColor: '#2ecc71',
                borderDash: [5, 5],
                fill: false,
                pointRadius: 0
            });
        }

        if (mainChart) mainChart.destroy();
        if (ctx) {
            mainChart = new Chart(ctx, {
                type: chartType,
                data: { labels: displayDates, datasets },
                options: {
                    responsive: true,
                    maintainAspectRatio: false,
                    scales: {
                        x: { stacked: chartType === 'bar' },
                        y: { stacked: chartType === 'bar', beginAtZero: true, title: { display: true, text: 'Days' } }
                    }
                }
            });
        }
        return;
    }

    // Default behavior for other metrics (User based)
    filteredData.forEach(d => {
        if (!dateMap.has(d.dateStr)) return;
        const daily = dateMap.get(d.dateStr);
        daily[d.author] = (daily[d.author] || 0) + (d[metric] || 0);
    });
    const datasets = allUsers.map(user => {
        const rawData = displayDates.map(date => dateMap.get(date)[user] || 0);
        const plotData = showTrend ? calculateMovingAverage(rawData, 7) : rawData;
        return {
            label: user,
            data: plotData,
            fill: chartType === 'bar',
            borderColor: stringToColor(user),
            backgroundColor: stringToColor(user),
            tension: 0.1,
            borderWidth: chartType === 'bar' ? 0 : 2
        };
    });
    if (mainChart) mainChart.destroy();
    if (ctx) {
        mainChart = new Chart(ctx, {
            type: chartType,
            data: { labels: displayDates, datasets },
            options: { responsive: true, maintainAspectRatio: false, scales: { x: { stacked: chartType === 'bar' }, y: { stacked: chartType === 'bar', beginAtZero: true } } }
        });
    } else {
        mainChart = null;
    }
}

function updatePieChart(filteredData, metric) {
    const userTotals = {};
    filteredData.forEach(d => {
        const val = Number(d[metric]) || 0;
        userTotals[d.author] = (userTotals[d.author] || 0) + val;
    });

    // Sort users by total values in descending order, then by name for ties
    const sortedEntries = Object.entries(userTotals)
        .sort((a, b) => {
            if (b[1] !== a[1]) return b[1] - a[1];
            return a[0].localeCompare(b[0]);
        });

    const labels = sortedEntries.map(e => e[0]);
    const values = sortedEntries.map(e => e[1]);

    if (pieChart) pieChart.destroy();
    if (pieCtx) {
        pieChart = new Chart(pieCtx, {
            type: 'doughnut',
            data: { labels, datasets: [{ data: values, backgroundColor: labels.map(u => stringToColor(u)) }] },
            options: { responsive: true, maintainAspectRatio: false, plugins: { legend: { position: 'right' } } }
        });
    } else {
        pieChart = null;
    }
}

function updateDayOfWeekChart(filteredData, metric) {
    const days = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];
    const dayTotals = new Array(7).fill(0);
    filteredData.forEach(d => { dayTotals[d.dayOfWeek] += (d[metric] || 0); });
    if (dowChart) dowChart.destroy();
    if (dowCtx) {
        dowChart = new Chart(dowCtx, {
            type: 'bar',
            data: { labels: days, datasets: [{ label: t('label_activity'), data: dayTotals, backgroundColor: '#3498db99' }] },
            options: { responsive: true, maintainAspectRatio: false, plugins: { legend: { display: false } } }
        });
    } else {
        dowChart = null;
    }
}

function updateHeatmapChart(filteredData, metric) {
    const heatmapData = [];
    const counts = {};
    filteredData.forEach(d => { if (d.hours) d.hours.forEach(h => { const key = `${d.dayOfWeek}-${h}`; counts[key] = (counts[key] || 0) + 1; }); });
    for (let d = 0; d < 7; d++) for (let h = 0; h < 24; h++) heatmapData.push({ x: h, y: d, v: counts[`${d}-${h}`] || 0 });

    if (heatmapChart) heatmapChart.destroy();
    if (heatmapCtx) {
        heatmapChart = new Chart(heatmapCtx, {
            type: 'matrix',
            data: {
                datasets: [{
                    label: 'Commit Frequency',
                    data: heatmapData,
                    backgroundColor: ctx => `rgba(52, 152, 219, ${Math.min(ctx.dataset.data[ctx.dataIndex].v / 10, 1)})`,
                    width: ({ chart }) => chart.chartArea ? (chart.chartArea.width / 24) - 1 : 0,
                    height: ({ chart }) => chart.chartArea ? (chart.chartArea.height / 7) - 1 : 0
                }]
            },
            options: {
                responsive: true,
                maintainAspectRatio: false,
                plugins: {
                    legend: { display: false },
                    tooltip: {
                        callbacks: {
                            label: ctx => {
                                const d = ctx.raw;
                                const days = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];
                                return `${days[d.y]} ${d.x}:00 - ${d.v} ${t('label_commits')}`;
                            }
                        }
                    }
                },
                scales: {
                    x: {
                        type: 'linear', min: 0, max: 23,
                        ticks: { stepSize: 1, callback: v => v + ':00' },
                        grid: { display: false },
                        title: { display: true, text: 'Hour of Day' }
                    },
                    y: {
                        type: 'linear', min: 0, max: 6,
                        ticks: {
                            stepSize: 1,
                            callback: v => ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'][v]
                        },
                        grid: { display: false },
                        reverse: true
                    }
                }
            }
        });
    }
}

function updateSizeDistChart(filteredData) {
    const counts = [0, 0, 0, 0, 0];
    filteredData.forEach(d => { if (d.commit_sizes) d.commit_sizes.forEach(s => { if (s < 10) counts[0]++; else if (s < 50) counts[1]++; else if (s < 200) counts[2]++; else if (s < 500) counts[3]++; else counts[4]++; }); });
    if (sizeChart) sizeChart.destroy();
    if (sizeCtx) {
        sizeChart = new Chart(sizeCtx, {
            type: 'bar',
            data: {
                labels: ['XS (<10)', 'S (10-50)', 'M (50-200)', 'L (200-500)', 'XL (>500)'],
                datasets: [{ data: counts, backgroundColor: '#f1c40f99' }]
            },
            options: {
                responsive: true,
                maintainAspectRatio: false,
                plugins: {
                    legend: { display: false }
                },
                scales: {
                    y: {
                        beginAtZero: true,
                        title: { display: true, text: t('label_commit_count') }
                    }
                }
            }
        });
    } else {
        sizeChart = null;
    }
}

function updateWorkDurationChart(filteredData) {
    const userDatasets = {};
    filteredData.forEach(d => { if (d.hours && d.hours.length > 1) { if (!userDatasets[d.author]) userDatasets[d.author] = [0, 0, 0, 0]; const dur = Math.max(...d.hours) - Math.min(...d.hours); if (dur < 1) userDatasets[d.author][0]++; else if (dur < 4) userDatasets[d.author][1]++; else if (dur < 8) userDatasets[d.author][2]++; else userDatasets[d.author][3]++; } });
    const datasets = Object.entries(userDatasets).map(([user, bins]) => ({ label: user, data: bins, backgroundColor: stringToColor(user) }));
    if (durChart) durChart.destroy();
    if (durCtx) {
        durChart = new Chart(durCtx, {
            type: 'bar',
            data: { labels: ['<1h', '1-4h', '4-8h', '8h+'], datasets },
            options: { responsive: true, maintainAspectRatio: false, scales: { x: { stacked: true }, y: { stacked: true } } }
        });
    } else {
        durChart = null;
    }
}

function updateHealthTrendChart(filteredData, startDate, endDate) {
    const displayDates = [];
    let curr = new Date(startDate);
    const end = new Date(endDate);
    const dateMap = new Map();

    while (curr <= end) {
        const dStr = curr.toISOString().split('T')[0];
        displayDates.push(dStr);
        dateMap.set(dStr, { changes: 0, churn: 0, durations: [] });
        curr.setDate(curr.getDate() + 1);
    }

    filteredData.forEach(d => {
        if (dateMap.has(d.dateStr)) {
            const entry = dateMap.get(d.dateStr);
            entry.changes += d.total_changes;
            entry.churn += d.churn;
            if (d.hours && d.hours.length > 1) {
                entry.durations.push(Math.max(...d.hours) - Math.min(...d.hours));
            }
        }
    });

    const churnRates = displayDates.map(d => {
        const e = dateMap.get(d);
        return e.changes > 0 ? (e.churn / e.changes) * 100 : 0;
    });

    const avgDurations = displayDates.map(d => {
        const e = dateMap.get(d);
        return e.durations.length > 0 ? e.durations.reduce((a, b) => a + b, 0) / e.durations.length : 0;
    });

    if (healthChart) healthChart.destroy();
    if (healthCtx) {
        healthChart = new Chart(healthCtx, {
            type: 'line',
            data: {
                labels: displayDates,
                datasets: [
                    {
                        label: 'Churn Rate (%)',
                        data: calculateMovingAverage(churnRates, 7),
                        borderColor: '#e74c3c',
                        backgroundColor: 'rgba(231, 76, 60, 0.1)',
                        fill: true,
                        yAxisID: 'y',
                        tension: 0.4,
                        pointRadius: 0
                    },
                    {
                        label: 'Avg Work Duration (Hours)',
                        data: calculateMovingAverage(avgDurations, 7),
                        borderColor: '#8e44ad',
                        backgroundColor: 'rgba(142, 68, 173, 0.1)',
                        fill: true,
                        yAxisID: 'y1',
                        tension: 0.4,
                        pointRadius: 0
                    }
                ]
            },
            options: {
                responsive: true, maintainAspectRatio: false,
                scales: {
                    y: { beginAtZero: true, max: 100, title: { display: true, text: 'Churn Rate (%)' } },
                    y1: { beginAtZero: true, max: 24, position: 'right', grid: { drawOnChartArea: false }, title: { display: true, text: 'Hours' } }
                }
            }
        });
    } else {
        healthChart = null;
    }
}

function updateOwnershipChart(filteredData, startDate, endDate) {
    const filteredAuthors = new Set(filteredData.map(d => d.author));
    const fileUserMap = {};
    dashboardData.file_stats.forEach(fs => { if (filteredAuthors.has(fs.author)) { const fName = filePaths[fs.file_idx] || fs.file_idx; if (!fileUserMap[fName]) fileUserMap[fName] = {}; fileUserMap[fName][fs.author] = (fileUserMap[fName][fs.author] || 0) + fs.count; } });
    const fileTotals = Object.entries(fileUserMap).map(([f, users]) => ({ file: f, total: Object.values(users).reduce((a, b) => a + b, 0), users })).sort((a, b) => b.total - a.total).slice(0, 15).reverse();
    const ownerUsers = [...new Set(fileTotals.flatMap(f => Object.keys(f.users)))];
    const datasets = ownerUsers.map(user => ({ label: user, data: fileTotals.map(f => f.users[user] || 0), backgroundColor: stringToColor(user) }));
    if (ownerChart) ownerChart.destroy();
    if (ownerCtx) {
        ownerChart = new Chart(ownerCtx, {
            type: 'bar',
            data: { labels: fileTotals.map(f => f.file), datasets },
            options: { indexAxis: 'y', responsive: true, maintainAspectRatio: false, scales: { x: { stacked: true }, y: { stacked: true } } }
        });
    } else {
        ownerChart = null;
    }
}

function updateIsolatedFilesTable(filteredData) {
    const tableBody = document.getElementById('isolatedFilesTableBody');
    if (!tableBody) return;
    tableBody.innerHTML = '';

    const filteredAuthors = new Set(filteredData.map(d => d.author));
    const fileAuthorMap = {};

    dashboardData.file_stats.forEach(fs => {
        if (filteredAuthors.has(fs.author)) {
            const fName = filePaths[fs.file_idx] || fs.file_idx;
            if (!fileAuthorMap[fName]) fileAuthorMap[fName] = new Set();
            fileAuthorMap[fName].add(fs.author);
        }
    });

    const isolatedFiles = [];
    Object.entries(fileAuthorMap).forEach(([file, authors]) => {
        if (authors.size === 1) {
            const author = [...authors][0];
            const totalModifications = dashboardData.file_stats
                .filter(fs => {
                    const mappedFile = filePaths[fs.file_idx] || fs.file_idx;
                    return mappedFile === file && fs.author === author;
                })
                .reduce((sum, fs) => sum + fs.count, 0);

            isolatedFiles.push({ file, author, count: totalModifications });
        }
    });

    isolatedFiles.sort((a, b) => b.count - a.count);

    isolatedFiles.slice(0, 30).forEach(item => {
        const row = document.createElement('tr');
        const color = stringToColor(item.author);
        row.innerHTML = `
            <td>${item.file}</td>
            <td><span class="user-badge" style="background-color: ${color}33; color: ${color}; border: 1px solid ${color}">${item.author}</span></td>
            <td>${item.count}</td>
        `;
        tableBody.appendChild(row);
    });
}

function updateLeadTimeChart(filteredData, startDate, endDate) {
    const allFilteredMerges = dashboardData.merge_events
        .filter(me => me.date >= startDate && me.date <= endDate)
        .sort((a, b) => b.date.localeCompare(a.date));

    // Calculate a week ago from the endDate to show "about a week's worth"
    const endD = new Date(endDate);
    const weekAgo = new Date(endD);
    weekAgo.setDate(endD.getDate() - 7);
    const weekAgoStr = weekAgo.toISOString().split('T')[0];

    let limit = 15;
    const recentMergesCount = allFilteredMerges.filter(me => me.date >= weekAgoStr).length;
    limit = Math.max(limit, recentMergesCount);
    limit = Math.min(limit, 50); // Hard cap at 50 to maintain readability

    const branches = allFilteredMerges.slice(0, limit).reverse();

    if (leadChart) leadChart.destroy();
    if (leadCtx) {
        leadChart = new Chart(leadCtx, {
            type: 'bar',
            data: {
                labels: branches.map(b => b.branch),
                datasets: [{
                    label: t('label_leadtime_days'),
                    data: branches.map(b => b.days),
                    backgroundColor: '#27ae6099'
                }]
            },
            options: {
                indexAxis: 'y',
                responsive: true,
                maintainAspectRatio: false,
                scales: {
                    x: {
                        beginAtZero: true,
                        title: { display: true, text: t('label_days') }
                    }
                }
            }
        });
    } else {
        leadChart = null;
    }

    // Update stats summary
    const statsContainer = document.getElementById('leadTimeStats');
    if (allFilteredMerges.length > 0) {
        const days = allFilteredMerges.map(m => m.days).sort((a, b) => a - b);
        const avg = days.reduce((a, b) => a + b, 0) / days.length;
        const median = days[Math.floor(days.length * 0.5)];
        const p90 = days[Math.floor(days.length * 0.9)];

        statsContainer.innerHTML = `
                <span><strong>${t('label_leadtime_avg')}:</strong> ${avg.toFixed(1)}${t('label_days')}</span>
                <span><strong>${t('label_leadtime_median')}:</strong> ${median.toFixed(1)}${t('label_days')}</span>
                <span><strong>${t('label_leadtime_p90')}:</strong> ${p90.toFixed(1)}${t('label_days')}</span>
            `;
    } else {
        statsContainer.innerHTML = `<span>No merge data for this period</span>`;
    }
}

function updateLeadTimeTrendChart(startDate, endDate) {
    const stats = dashboardData.daily_lead_time_stats || [];
    const filtered = stats.filter(s => s.date >= startDate && s.date <= endDate)
        .sort((a, b) => a.date.localeCompare(b.date));

    if (leadTimeTrendChart) leadTimeTrendChart.destroy();

    const movingAvg = calculateMovingAverage(filtered.map(s => s.avg_days), 7);

    if (leadTimeTrendCtx) {
        leadTimeTrendChart = new Chart(leadTimeTrendCtx, {
            type: 'line',
            data: {
                labels: filtered.map(s => s.date),
                datasets: [
                    {
                        label: 'Daily Avg Lead Time',
                        data: filtered.map(s => s.avg_days),
                        borderColor: '#27ae6033',
                        backgroundColor: '#27ae6011',
                        borderWidth: 1,
                        pointRadius: 2,
                        fill: true,
                        tension: 0.1
                    },
                    {
                        label: '7-Day Trend',
                        data: movingAvg,
                        borderColor: '#27ae60',
                        borderWidth: 2,
                        pointRadius: 0,
                        fill: false,
                        tension: 0.4
                    }
                ]
            },
            options: {
                responsive: true,
                maintainAspectRatio: false,
                scales: {
                    y: { beginAtZero: true, title: { display: true, text: 'Days' } }
                }
            }
        });
    } else {
        leadTimeTrendChart = null;
    }
}

function updateFileTypeTrendChart(startDate, endDate) {
    const stats = dashboardData.daily_file_type_stats || [];
    const dateMap = new Map();
    const extensions = new Set();

    stats.filter(s => s.date >= startDate && s.date <= endDate).forEach(s => {
        if (!dateMap.has(s.date)) dateMap.set(s.date, {});
        dateMap.get(s.date)[s.extension] = (dateMap.get(s.date)[s.extension] || 0) + s.added;
        extensions.add(s.extension);
    });

    const sortedDates = Array.from(dateMap.keys()).sort();
    const sortedExts = Array.from(extensions).sort((a, b) => {
        if (a === 'test') return -1;
        if (b === 'test') return 1;
        return a.localeCompare(b);
    });

    const datasets = sortedExts.map(ext => {
        const dataPoints = sortedDates.map(date => dateMap.get(date)[ext] || 0);
        const color = ext === 'test' ? '#e67e22' : stringToColor(ext);
        return {
            label: ext,
            data: calculateMovingAverage(dataPoints, 7),
            borderColor: color,
            backgroundColor: color + '22',
            borderWidth: ext === 'test' ? 3 : 1,
            pointRadius: 0,
            fill: ext === 'test',
            tension: 0.4
        };
    });

    if (fileTypeTrendChart) fileTypeTrendChart.destroy();
    if (fileTypeTrendCtx) {
        fileTypeTrendChart = new Chart(fileTypeTrendCtx, {
            type: 'line',
            data: {
                labels: sortedDates,
                datasets: datasets
            },
            options: {
                responsive: true,
                maintainAspectRatio: false,
                scales: {
                    y: { beginAtZero: true, title: { display: true, text: 'Added Lines (7-day Avg)' } }
                },
                plugins: {
                    legend: { position: 'right' }
                }
            }
        });
    } else {
        fileTypeTrendChart = null;
    }
}

function updateVelocitySizeChart(startDate, endDate) {
    const dateMap = new Map();
    let curr = new Date(startDate);
    const end = new Date(endDate);
    const displayDates = [];
    while (curr <= end) {
        const dStr = curr.toISOString().split('T')[0];
        displayDates.push(dStr);
        dateMap.set(dStr, { commits: 0, changes: 0 });
        curr.setDate(curr.getDate() + 1);
    }

    // data contains aggregated DailyStat with total_changes and commit_count
    data.forEach(d => {
        if (dateMap.has(d.dateStr)) {
            const entry = dateMap.get(d.dateStr);
            entry.commits += d.commit_count;
            entry.changes += d.total_changes;
        }
    });

    const commitCounts = displayDates.map(d => dateMap.get(d).commits);
    const avgSizes = displayDates.map(d => {
        const entry = dateMap.get(d);
        return entry.commits > 0 ? entry.changes / entry.commits : 0;
    });

    const movingCommits = calculateMovingAverage(commitCounts, 7);
    const movingSizes = calculateMovingAverage(avgSizes, 7);

    if (velocitySizeChart) velocitySizeChart.destroy();
    if (velocitySizeCtx) {
        velocitySizeChart = new Chart(velocitySizeCtx, {
            type: 'line',
            data: {
                labels: displayDates,
                datasets: [
                    {
                        label: 'Commit Density (Count/Day)',
                        data: movingCommits,
                        borderColor: '#3498db',
                        backgroundColor: 'rgba(52, 152, 219, 0.1)',
                        yAxisID: 'y',
                        fill: true,
                        tension: 0.4,
                        pointRadius: 0
                    },
                    {
                        label: 'Avg Commit Size (Lines/Commit)',
                        data: movingSizes,
                        borderColor: '#e67e22',
                        backgroundColor: 'rgba(230, 126, 34, 0.1)',
                        yAxisID: 'y1',
                        fill: false,
                        tension: 0.4,
                        pointRadius: 0
                    }
                ]
            },
            options: {
                responsive: true,
                maintainAspectRatio: false,
                scales: {
                    y: {
                        beginAtZero: true,
                        title: { display: true, text: 'Commits / Day' },
                        position: 'left'
                    },
                    y1: {
                        beginAtZero: true,
                        title: { display: true, text: 'Lines / Commit' },
                        position: 'right',
                        grid: { drawOnChartArea: false }
                    }
                }
            }
        });
    }
}

function updateContextSwitchChart(filteredData, startDate, endDate) {
    const dateMap = new Map();
    let curr = new Date(startDate);
    const end = new Date(endDate);
    const displayDates = [];
    while (curr <= end) {
        const dStr = curr.toISOString().split('T')[0];
        displayDates.push(dStr);
        dateMap.set(dStr, { dirs: 0, unrelated: 0 });
        curr.setDate(curr.getDate() + 1);
    }

    // dashboardData.daily_dir_counts is global, but we want to filter by selected users
    // Let's calculate from filteredData instead
    filteredData.forEach(d => {
        if (dateMap.has(d.dateStr)) {
            const entry = dateMap.get(d.dateStr);
            // Approximation for daily_dir_counts from filteredData
            entry.dirs = Math.max(entry.dirs, d.hours.length > 0 ? 1 : 0); // Temporary placeholder
            entry.unrelated += (d.unrelated_switches || 0);
        }
    });

    // Use the global dir count as baseline, and unrelated switches from filtered data
    const globalDirCounts = dashboardData.daily_dir_counts.reduce((acc, dc) => {
        acc[dc.date] = dc.count;
        return acc;
    }, {});

    if (ctxChart) ctxChart.destroy();
    if (ctxSwitchCtx) {
        ctxChart = new Chart(ctxSwitchCtx, {
            type: 'line',
            data: {
                labels: displayDates,
                datasets: [
                    {
                        label: t('label_avg_dirs'),
                        data: displayDates.map(d => globalDirCounts[d] || 0),
                        borderColor: '#9b59b6',
                        backgroundColor: 'rgba(155, 89, 182, 0.1)',
                        fill: true,
                        tension: 0.4
                    },
                    {
                        label: t('label_unrelated_switches'),
                        data: displayDates.map(d => dateMap.get(d).unrelated),
                        borderColor: '#e74c3c',
                        borderDash: [5, 5],
                        fill: false,
                        tension: 0.4
                    },
                    {
                        label: t('label_active_prs'),
                        data: displayDates.map(date => {
                            const daily = filteredData.filter(d => d.dateStr === date);
                            return daily.length > 0 ? Math.max(...daily.map(d => d.active_prs || 0)) : 0;
                        }),
                        type: 'bar',
                        backgroundColor: 'rgba(46, 204, 113, 0.2)',
                        borderColor: 'rgba(46, 204, 113, 0.5)',
                        borderWidth: 1,
                        yAxisID: 'y1'
                    }
                ]
            },
            options: {
                responsive: true,
                maintainAspectRatio: false,
                scales: {
                    y: { beginAtZero: true, title: { display: true, text: 'Switches / Dirs' } },
                    y1: { beginAtZero: true, position: 'right', title: { display: true, text: 'Active PRs' }, grid: { drawOnChartArea: false } }
                }
            }
        });
    }
}

function updateContextSwitchTrendChart(filteredData, startDate, endDate) {
    const dateMap = new Map();
    let curr = new Date(startDate);
    const end = new Date(endDate);
    const displayDates = [];
    while (curr <= end) {
        const dStr = curr.toISOString().split('T')[0];
        displayDates.push(dStr);
        dateMap.set(dStr, { switches: 0, intervals: [] });
        curr.setDate(curr.getDate() + 1);
    }

    filteredData.forEach(d => {
        if (dateMap.has(d.dateStr)) {
            const entry = dateMap.get(d.dateStr);
            entry.switches += (d.unrelated_switches || 0);
            if (d.commit_intervals) {
                entry.intervals.push(...d.commit_intervals);
            }
        }
    });

    const dailySwitches = displayDates.map(d => dateMap.get(d).switches);
    const dailyAvgIntervals = displayDates.map(d => {
        const ints = dateMap.get(d).intervals;
        return ints.length > 0 ? ints.reduce((a, b) => a + b, 0) / ints.length : 0;
    });

    const movingSwitches = calculateMovingAverage(dailySwitches, 7);
    const movingIntervals = calculateMovingAverage(dailyAvgIntervals, 7);

    if (ctxSwitchTrendChart) ctxSwitchTrendChart.destroy();
    if (ctxSwitchTrendCtx) {
        ctxSwitchTrendChart = new Chart(ctxSwitchTrendCtx, {
            type: 'bar',
            data: {
                labels: displayDates,
                datasets: [
                    {
                        label: t('label_unrelated_switches'),
                        data: movingSwitches,
                        backgroundColor: 'rgba(231, 76, 60, 0.2)',
                        borderColor: '#e74c3c',
                        borderWidth: 1,
                        yAxisID: 'y'
                    },
                    {
                        label: 'Avg Commit Interval (min)',
                        data: movingIntervals,
                        type: 'line',
                        borderColor: '#1abc9c',
                        backgroundColor: 'transparent',
                        yAxisID: 'y1',
                        tension: 0.4,
                        pointRadius: 0
                    }
                ]
            },
            options: {
                responsive: true,
                maintainAspectRatio: false,
                scales: {
                    y: { beginAtZero: true, title: { display: true, text: 'Unrelated Switches' }, position: 'left' },
                    y1: { beginAtZero: true, title: { display: true, text: 'Minutes between Commits' }, position: 'right', grid: { drawOnChartArea: false } }
                }
            }
        });
    }
}

function updateFragmentationChart(filteredData, startDate, endDate) {
    const intervals = filteredData.flatMap(d => d.commit_intervals || []);

    const buckets = [
        { label: '<15m', min: 0, max: 15 },
        { label: '15-30m', min: 15, max: 30 },
        { label: '30-60m', min: 30, max: 60 },
        { label: '1-2h', min: 60, max: 120 },
        { label: '2-4h', min: 120, max: 240 },
        { label: '>4h', min: 240, max: Infinity }
    ];

    const bins = buckets.map(b => ({ label: b.label, count: 0, min: b.min, max: b.max }));
    intervals.forEach(v => {
        const bin = bins.find(b => v >= b.min && v < b.max);
        if (bin) bin.count++;
        else if (v >= buckets[buckets.length - 1].max) bins[bins.length - 1].count++;
    });

    if (fragmentationChart) fragmentationChart.destroy();
    if (fragmentationCtx) {
        fragmentationChart = new Chart(fragmentationCtx, {
            type: 'bar',
            data: {
                labels: bins.map(b => b.label),
                datasets: [{
                    label: t('label_commit_count'),
                    data: bins.map(b => b.count),
                    backgroundColor: '#1abc9c99'
                }]
            },
            options: {
                responsive: true,
                maintainAspectRatio: false,
                plugins: { legend: { display: false } },
                scales: {
                    y: { beginAtZero: true, title: { display: true, text: t('label_commit_count') } },
                    x: { title: { display: true, text: t('label_minutes') } }
                }
            }
        });
    } else {
        fragmentationChart = null;
    }
}

function generateInsights(filteredData, startDate, endDate) {
    const container = document.getElementById('insightsGrid');
    container.innerHTML = '';
    if (filteredData.length === 0) { document.getElementById('insightsContainer').style.display = 'none'; return; }
    document.getElementById('insightsContainer').style.display = 'block';
    // (Simple insight implementation)
    const totalChanges = filteredData.reduce((a, d) => a + d.total_changes, 0);
    const totalChurn = filteredData.reduce((a, d) => a + d.churn, 0);
    const churnRate = totalChanges > 0 ? (totalChurn / totalChanges) * 100 : 0;
    if (churnRate > 30) {
        const card = document.createElement('div');
        card.className = 'insight-card warning';
        card.innerHTML = `<div class="insight-icon">📉</div><div class="insight-body"><div class="insight-title">${t('insight_unstable_title')}</div><div class="insight-desc">${t('insight_unstable_desc').replace('{value}', churnRate.toFixed(1))}</div></div>`;
        container.appendChild(card);
    }
}

function updatePredictiveDashboard(filteredData) {
    if (!filteredData) {
        const startDate = document.getElementById('startDate').value;
        const endDate = document.getElementById('endDate').value;
        filteredData = data.filter(d => d.dateStr >= startDate && d.dateStr <= endDate && selectedUsers.has(d.author));
    }

    const weeklyStats = getWeeklyStats(filteredData);
    if (weeklyStats.length < 2) {
        ['currentVelocityValue', 'velocityTrendValue', 'projectedThroughputValue', 'estCompletionValue'].forEach(id => {
            const el = document.getElementById(id);
            if (el) el.textContent = '-';
        });
        const rangeEl = document.getElementById('estCompletionRange');
        if (rangeEl) rangeEl.textContent = '';
        if (forecastChart) forecastChart.destroy();

        _currentWeeklyMean = undefined;
        _currentWeeklyStdev = undefined;
        _currentVelocity = undefined;
        return;
    }

    const history = weeklyStats.map(w => w.commits);
    const sum = history.reduce((a, b) => a + b, 0);
    const mean = sum / history.length;
    const variance = history.reduce((a, b) => a + Math.pow(b - mean, 2), 0) / history.length;
    const stdev = Math.sqrt(variance);

    const last4Weeks = weeklyStats.slice(-4).reverse();
    const currentVelocity = last4Weeks.length > 0 ? last4Weeks.reduce((acc, w) => acc + w.commits, 0) / last4Weeks.length : 0;

    // Use recent (last-4 weeks) standard deviation for short-term probability estimates
    // (more responsive to recent changes). Fall back to historical stdev when needed.
    let stdevRecent = stdev;
    if (last4Weeks.length >= 2) {
        const meanRecent = last4Weeks.reduce((a, b) => a + b.commits, 0) / last4Weeks.length;
        const varianceRecent = last4Weeks.reduce((acc, w) => acc + Math.pow(w.commits - meanRecent, 2), 0) / last4Weeks.length;
        stdevRecent = Math.sqrt(varianceRecent);
    }

    _currentWeeklyMean = mean; // keep historical mean for reference
    _currentWeeklyStdev = stdevRecent; // use recent stdev for forecasts
    _currentVelocity = currentVelocity;

    const cov = stdev / (mean || 1);
    const confidence = cov < 0.2 ? 'High' : cov < 0.5 ? 'Medium' : 'Low';
    const confidenceColor = confidence === 'High' ? '#27ae60' : confidence === 'Medium' ? '#f39c12' : '#e74c3c';

    const recentAvg = (last4Weeks.length > 0 ? last4Weeks[0].commits : 0 + (last4Weeks.length > 1 ? last4Weeks[1].commits : (last4Weeks.length > 0 ? last4Weeks[0].commits : 0))) / 2;
    const prevAvg = last4Weeks.length >= 4
        ? (last4Weeks[2].commits + last4Weeks[3].commits) / 2
        : (last4Weeks.length > 2 ? last4Weeks[2].commits : recentAvg);

    const trend = prevAvg > 0 ? ((recentAvg - prevAvg) / prevAvg) * 100 : 0;
    const trendEl = document.getElementById('velocityTrendValue');
    if (trendEl) {
        trendEl.textContent = `${trend >= 0 ? '▲' : '▼'} ${Math.abs(trend).toFixed(1)}%`;
        trendEl.className = `forecast-trend ${trend >= 0 ? 'up' : 'down'}`;
    }

    const velocityEl = document.getElementById('currentVelocityValue');
    if (velocityEl) velocityEl.innerHTML = `${currentVelocity.toFixed(1)} ${t('label_commits')}/week <span style="font-size: 12px; color: ${confidenceColor}; font-weight: normal;">(Confidence: ${confidence})</span>`;

    // Populate Velocity History (last 4 weeks)
    const historyEl = document.getElementById('velocityHistory');
    if (historyEl) {
        historyEl.innerHTML = '';
        const recentWeeks = weeklyStats.slice(-4); // oldest to newest
        recentWeeks.forEach(w => {
            const bar = document.createElement('div');
            bar.title = `${w.week_start}: ${w.commits} commits`;
            bar.style.width = '20px';
            // Use historical mean as 100% height (capped or scaled)
            const height = Math.min(40, (w.commits / (mean || 1)) * 20);
            bar.style.height = Math.max(2, height) + 'px';
            bar.style.backgroundColor = '#3498db88';
            bar.style.borderRadius = '2px';
            bar.setAttribute('data-tooltip', `${w.week_start}: ${w.commits} commits`);
            historyEl.appendChild(bar);
        });
    }

    const projected60 = Math.round(currentVelocity * (60 / 7));
    const projectedEl = document.getElementById('projectedThroughputValue');
    if (projectedEl) projectedEl.textContent = `${projected60.toLocaleString()} ${t('label_commits')}`;

    updateCompletionEstimate();
    updateWeeklyCommitmentForecast();

    updateForecastChart(weeklyStats, currentVelocity, stdev);
}

function updateCompletionEstimate() {
    if (_currentVelocity === undefined) return;

    const remaining = parseInt(document.getElementById('remainingWorkInput').value) || 0;
    const estEl = document.getElementById('estCompletionValue');
    const rangeEl = document.getElementById('estCompletionRange');
    const insightsContainer = document.getElementById('insightsGrid');

    // Clear previous predictive insight
    const existingInsight = document.getElementById('predictive-insight-card');
    if (existingInsight) existingInsight.remove();

    if (remaining > 0 && _currentVelocity > 0) {
        function calcDate(v) {
            const weeks = remaining / Math.max(v, 0.1);
            const d = new Date();
            d.setDate(d.getDate() + (weeks * 7));
            return d.toLocaleDateString(currentLang === 'ja' ? 'ja-JP' : 'en-US', { month: 'short', day: 'numeric' });
        }

        const likelyDate = calcDate(_currentVelocity);
        const optimisticDate = calcDate(_currentVelocity + _currentWeeklyStdev);
        const pessimisticDate = calcDate(Math.max(_currentVelocity - _currentWeeklyStdev, 0.5));

        if (estEl) estEl.textContent = likelyDate;
        if (rangeEl) rangeEl.innerHTML = `🚀 Optimistic: ${optimisticDate}<br>🐢 Pessimistic: ${pessimisticDate}`;

        if (insightsContainer) {
            const card = document.createElement('div');
            card.className = 'insight-card positive';
            card.id = 'predictive-insight-card';
            card.innerHTML = `
                        <div class="insight-icon">🎯</div>
                        <div class="insight-body">
                            <div class="insight-title">${t('insight_predicted_goal_title')}</div>
                            <div class="insight-desc">${t('insight_predicted_goal_desc').replace('{remaining}', remaining).replace('{date}', likelyDate)}</div>
                        </div>
                    `;
            insightsContainer.prepend(card);
        }

    } else {
        if (estEl) estEl.textContent = remaining <= 0 ? t('status.work_complete') || 'Work Complete!' : '-';
        if (rangeEl) rangeEl.innerHTML = '';
    }
}

function updateWeeklyCommitmentForecast() {
    // Require computed recent velocity
    if (_currentVelocity === undefined) return;

    const weeklyGoal = parseInt(document.getElementById('weeklyGoalInput').value) || 0;

    // Use the recent velocity (last-4 weeks average) + recent stdev for short-term probability
    const meanForProb = _currentVelocity;
    const stdForProb = (_currentWeeklyStdev !== undefined ? _currentWeeklyStdev : 0.0001);

    let probability = (1 - normalCDF(weeklyGoal - 0.5, meanForProb, stdForProb)) * 100;
    probability = Math.max(0, Math.min(100, probability)); // clamp to [0,100]

    const safeCommitment = Math.max(0, Math.floor(meanForProb - 1.28 * stdForProb));

    const insightDiv = document.getElementById('commitmentInsight');
    if (insightDiv) {
        insightDiv.innerHTML = t('msg_weekly_forecast')
            .replace('{goal}', weeklyGoal)
            .replace('{prob}', Math.round(probability)) +
            "<br><br>" +
            t('msg_safe_commitment').replace('{safe}', safeCommitment);
    }
}


function updateDistributionChart(resTimes, leadTimes) {
    const distBox = document.getElementById('distBox');
    if (resTimes.length === 0 && leadTimes.length === 0) {
        if (distBox) distBox.style.display = 'none';
        return;
    }
    if (distBox) distBox.style.display = 'grid';

    function createCustomHistogram(data, buckets) {
        const bins = buckets.map(b => ({ label: b.label, count: 0, min: b.min, max: b.max }));
        data.forEach(v => {
            const bin = bins.find(b => v >= b.min && v < b.max);
            if (bin) bin.count++;
            else if (v >= buckets[buckets.length - 1].max) bins[bins.length - 1].count++;
        });
        return bins;
    }

    // Response Time Buckets (Hours)
    const resBuckets = [
        { label: '<1h', min: 0, max: 1 },
        { label: '1-4h', min: 1, max: 4 },
        { label: '4-8h', min: 4, max: 8 },
        { label: '8-24h', min: 8, max: 24 },
        { label: '1-3d', min: 24, max: 72 },
        { label: '3-7d', min: 72, max: 168 },
        { label: '>7d', min: 168, max: Infinity }
    ];

    const resBins = createCustomHistogram(resTimes, resBuckets);

    if (resDistChart) resDistChart.destroy();
    if (resDistCtx) {
        resDistChart = new Chart(resDistCtx, {
            type: 'bar',
            data: {
                labels: resBins.map(b => b.label),
                datasets: [{
                    label: t('chart_res_dist'),
                    data: resBins.map(b => b.count),
                    backgroundColor: 'rgba(230, 126, 34, 0.6)'
                }]
            },
            options: {
                responsive: true, maintainAspectRatio: false,
                plugins: { legend: { display: false } },
                scales: {
                    y: { beginAtZero: true, title: { display: true, text: t('label_mod_count') } }
                }
            }
        });
    } else {
        resDistChart = null;
    }

    // Lead Time Buckets (Days, but granular for short durations)
    // leadTimes are in days, so we multiply by 24 to compare with hour-based thresholds if needed,
    // or just use fractional days.
    const leadBuckets = [
        { label: '<4h', min: 0, max: 4 / 24 },
        { label: '4-12h', min: 4 / 24, max: 12 / 24 },
        { label: '12-24h', min: 12 / 24, max: 1 },
        { label: '1-3d', min: 1, max: 3 },
        { label: '3-7d', min: 3, max: 7 },
        { label: '7-14d', min: 7, max: 14 },
        { label: '>14d', min: 14, max: Infinity }
    ];

    const leadBins = createCustomHistogram(leadTimes, leadBuckets);

    if (leadDistChart) leadDistChart.destroy();
    leadDistChart = new Chart(leadDistCtx, {
        type: 'bar',
        data: {
            labels: leadBins.map(b => b.label),
            datasets: [{
                label: t('chart_lead_dist'),
                data: leadBins.map(b => b.count),
                backgroundColor: 'rgba(52, 152, 219, 0.6)'
            }]
        },
        options: {
            responsive: true, maintainAspectRatio: false,
            plugins: { legend: { display: false } },
            scales: {
                y: { beginAtZero: true, title: { display: true, text: t('label_mod_count') } }
            }
        }
    });
}

function getDetailedStats(values) {
    if (!values || values.length === 0) return { avg: 0, median: 0, min: 0, max: 0, p90: 0, count: 0 };
    const sorted = [...values].sort((a, b) => a - b);
    const avg = values.reduce((a, b) => a + b, 0) / values.length;
    const median = sorted[Math.floor(sorted.length * 0.5)];
    const p90 = sorted[Math.floor(sorted.length * 0.9)];
    return {
        avg,
        median,
        p90,
        min: sorted[0],
        max: sorted[sorted.length - 1],
        count: values.length
    };
}

function getWeeklyStats(filteredData) {
    const weeklyMap = {};
    filteredData.forEach(d => {
        const date = new Date(d.dateStr);
        const day = date.getDay();
        const diff = date.getDate() - day + (day === 0 ? -6 : 1); // Monday
        const monday = new Date(date.setDate(diff));
        const weekStart = monday.toISOString().split('T')[0];

        if (!weeklyMap[weekStart]) {
            weeklyMap[weekStart] = { week_start: weekStart, commits: 0, added: 0, deleted: 0 };
        }
        weeklyMap[weekStart].commits += d.commit_count;
        weeklyMap[weekStart].added += d.added;
        weeklyMap[weekStart].deleted += d.deleted;
    });
    return Object.values(weeklyMap).sort((a, b) => a.week_start.localeCompare(b.week_start));
}

function updateForecastChart(weeklyStats, currentVelocity, stdev) {
    if (forecastChart) forecastChart.destroy();

    const labels = weeklyStats.map(w => w.week_start);
    const dataPoint = weeklyStats.map(w => w.commits);

    // Projections (next 4 weeks)
    const projectionData = new Array(labels.length - 1).fill(null);
    projectionData.push(dataPoint[dataPoint.length - 1]); // connector

    const upperData = [...projectionData];
    const lowerData = [...projectionData];

    const lastDate = new Date(labels[labels.length - 1]);
    for (let i = 1; i <= 4; i++) {
        const nextDate = new Date(lastDate);
        nextDate.setDate(lastDate.getDate() + (i * 7));
        const nextDateStr = nextDate.toISOString().split('T')[0];
        labels.push(nextDateStr);
        projectionData.push(currentVelocity);
        upperData.push(currentVelocity + stdev);
        lowerData.push(Math.max(currentVelocity - stdev, 0));
    }

    forecastChart = new Chart(forecastCtx, {
        type: 'line',
        data: {
            labels: labels,
            datasets: [
                {
                    label: t('forecast_chart_title') + ' (History)',
                    data: dataPoint,
                    borderColor: '#3498db',
                    backgroundColor: '#3498db22',
                    fill: true,
                    tension: 0.3
                },
                {
                    label: t('forecast_chart_title') + ' (Projected)',
                    data: projectionData,
                    borderColor: '#3498db',
                    borderDash: [5, 5],
                    pointRadius: 0,
                    fill: false,
                    tension: 0
                },
                {
                    label: 'Range (Confidence)',
                    data: upperData,
                    borderColor: 'transparent',
                    backgroundColor: '#3498db11',
                    pointRadius: 0,
                    fill: '+1', // Fill down to lowerData (index 3)
                    tension: 0
                },
                {
                    label: 'Lower Bound',
                    data: lowerData,
                    borderColor: 'transparent',
                    pointRadius: 0,
                    fill: false,
                    tension: 0
                }
            ]
        },
        options: {
            responsive: true, maintainAspectRatio: false,
            scales: {
                y: { beginAtZero: true, title: { display: true, text: t('header_commits') } }
            }
        }
    });
}

function updateImpactAssessment(eventIdx) {
    const impactSection = document.getElementById('impactSection');
    const impactTableBody = document.getElementById('impactTableBody');
    const eventSelect = document.getElementById('eventSelect');

    if (!dashboardData.events || dashboardData.events.length === 0 || !dashboardData.github_prs || dashboardData.github_prs.length === 0) {
        if (impactSection) impactSection.style.display = 'none';
        return;
    }

    if (impactSection) impactSection.style.display = 'block';

    // Initialize event selector if empty
    if (eventSelect && eventSelect.options.length === 0) {
        // allow a "no initiative" state so users can hide event markers on all charts
        const noneOpt = document.createElement('option');
        noneOpt.value = 'none';
        noneOpt.textContent = t('btn_select_none');
        eventSelect.appendChild(noneOpt);

        dashboardData.events.forEach((e, idx) => {
            const opt = document.createElement('option');
            opt.value = idx;
            opt.textContent = `${e.name} (${e.date})`;
            eventSelect.appendChild(opt);
        });
        // Default to last event (preserve previous behavior)
        eventSelect.value = String(dashboardData.events.length - 1);
        eventIdx = dashboardData.events.length - 1;
    }

    // If eventIdx is not provided (called from updateDashboard), use current selector value
    if (eventIdx === undefined && eventSelect) {
        eventIdx = eventSelect.value; // keep raw value to detect 'none'
    }

    // If user selected the special "none" option (or an invalid value), hide assessment and remove event markers
    if (eventIdx === 'none' || eventIdx === null || isNaN(parseInt(eventIdx))) {
        const desc = document.getElementById('impactDescription');
        if (impactTableBody) impactTableBody.innerHTML = '';
        if (desc) desc.innerHTML = `<small>No initiative selected — event markers hidden on charts.</small>`;
        updateAllChartsWithEvents(false);
        return;
    }

    eventIdx = parseInt(eventIdx);
    const event = dashboardData.events[eventIdx];
    if (!event) return;

    const eventDate = new Date(event.date);
    if (impactTableBody) impactTableBody.innerHTML = '';

    const ninetyDaysBefore = new Date(eventDate);
    ninetyDaysBefore.setDate(eventDate.getDate() - 90);

    // Use ALL PRs for repository-wide initiative assessment
    // This ensures the assessment works even if GitHub-to-Git user mapping (aliases) is not set up.
    const relevantPRs = dashboardData.github_prs;

    const beforePRs = relevantPRs.filter(pr => {
        const d = new Date(pr.created_at);
        return d >= ninetyDaysBefore && d < eventDate;
    });

    const afterPRs = relevantPRs.filter(pr => {
        const d = new Date(pr.created_at);
        return d >= eventDate;
    });

    if (beforePRs.length === 0 || afterPRs.length === 0) {
        let reason = "";
        if (beforePRs.length === 0 && afterPRs.length === 0) reason = "No PRs found in the total history.";
        else if (beforePRs.length === 0) reason = `No PRs found in the 90 days prior to ${event.date}.`;
        else reason = `No PRs found on or after ${event.date}.`;

        const desc = document.getElementById('impactDescription');
        if (desc) desc.innerHTML = `<span style="color: #e74c3c;">⚠️ <strong>Assessment Unavailable:</strong> ${reason}</span><br><small>Total PRs in data: ${relevantPRs.length}. If you see PRs in other charts but not here, check if the initiative date (${event.date}) matches your PR history. Note: This section analyzes the entire repository to measure process changes.</small>`;
        return;
    }

    function getStats(prs, periodWeeks, isBefore) {
        // For throughput/lead-time, we care about when things were MERGED
        const mergedPRs = prs.filter(pr => pr.merged_at && (isBefore ? new Date(pr.merged_at) < eventDate : true));
        const leadTimeValues = mergedPRs.map(pr => (new Date(pr.merged_at) - new Date(pr.created_at)) / (1000 * 60 * 60 * 24));
        const lt = getDetailedStats(leadTimeValues);

        const throughput = mergedPRs.length / (periodWeeks || 1);

        const reworkCount = prs.filter(pr => {
            const hasRequestChanges = pr.reviews && pr.reviews.some(r => r.state === 'CHANGES_REQUESTED' && !isBot(r.user));
            const cycles = pr.reviews ? new Set(pr.reviews.filter(r => r.state !== 'COMMENTED' && !isBot(r.user)).map(r => r.submitted_at.split('T')[0])).size : 0;
            return hasRequestChanges || cycles > 1;
        }).length;
        const reworkRate = (reworkCount / (prs.length || 1)) * 100;

        const resTimeValues = prs.filter(pr => pr.reviews && pr.reviews.length > 0).map(pr => {
            const startTime = pr.first_assigned_at ? new Date(pr.first_assigned_at) : new Date(pr.created_at);
            const humanReviews = pr.reviews
                .filter(r => !isBot(r.user))
                .sort((a, b) => a.submitted_at.localeCompare(b.submitted_at));

            if (humanReviews.length > 0) {
                return (new Date(humanReviews[0].submitted_at) - startTime) / (1000 * 60 * 60);
            }
            return null;
        }).filter(v => v !== null && v > 0);
        const res = getDetailedStats(resTimeValues);

        const depthValues = prs.map(pr => pr.total_comments || 0);
        const depth = getDetailedStats(depthValues);

        const iterationValues = prs.map(pr => {
            const humanReviews = pr.reviews ? pr.reviews.filter(r => !isBot(r.user)) : [];
            return Math.max(1, humanReviews.length);
        });
        const iters = getDetailedStats(iterationValues);

        const stdDev = Math.sqrt(leadTimeValues.reduce((a, b) => a + Math.pow(b - lt.avg, 2), 0) / (leadTimeValues.length || 1));

        // Calculate Steps (Lines Added) and Test Ratio from daily_file_type_stats
        const fileStats = dashboardData.daily_file_type_stats || [];
        let totalAdded = 0;
        let testAdded = 0;

        fileStats.forEach(s => {
            const d = new Date(s.date);
            const inPeriod = isBefore ? (d >= ninetyDaysBefore && d < eventDate) : (d >= eventDate);
            if (inPeriod) {
                totalAdded += s.added;
                if (s.extension === 'test') {
                    testAdded += s.added;
                }
            }
        });
        const testRatio = totalAdded > 0 ? (testAdded / totalAdded) * 100 : 0;
        const stepsPerWeek = totalAdded / (periodWeeks || 1);

        // Calculate Commit Density
        const commitsInPeriod = dashboardData.commits.filter(c => {
            const d = new Date(c.date);
            return isBefore ? (d >= ninetyDaysBefore && d < eventDate) : (d >= eventDate);
        }).length;
        const commitDensity = commitsInPeriod / (periodWeeks || 1);

        // Average commit size (lines added per commit) for this period
        const avgCommitSize = commitsInPeriod > 0 ? (totalAdded / commitsInPeriod) : 0;

        // collect per-commit added counts so we can compute P90 for commit size
        const commitSizes = (dashboardData.commits || []).filter(c => {
            const d = new Date(c.date);
            return isBefore ? (d >= ninetyDaysBefore && d < eventDate) : (d >= eventDate);
        }).map(c => c.added || 0);
        const commitSizeStats = getDetailedStats(commitSizes);
        const commitSizeP90 = commitSizeStats.p90 || 0;

        return {
            throughput,
            commitDensity,
            avgCommitSize,
            commitSizeP90,
            median: lt.median,
            p90: lt.p90 || 0, // Fallback
            max: lt.max,
            avg: lt.avg,
            stdDev,
            reworkRate,
            responseTime: res.avg,
            responseTimeP90: res.p90,
            responseMedian: res.median,
            reviewDepth: depth.avg,
            iterations: iters.avg,
            testRatio,
            stepsPerWeek
        };
    }

    const now = allDates.length > 0 ? new Date(allDates[allDates.length - 1]) : new Date();
    const beforeWeeks = 90 / 7;
    const diffDays = Math.max(1, (now - eventDate) / (1000 * 60 * 60 * 24));
    const afterWeeks = diffDays / 7;

    const before = getStats(beforePRs, beforeWeeks, true);
    const after = getStats(afterPRs, afterWeeks, false);

    const metrics = [
        { id: 'metric_throughput', b: before.throughput, a: after.throughput, unit: ' PRs/week', lowerIsBetter: false, descr: 'desc_throughput' },
        { id: 'metric_commit_density', b: before.commitDensity, a: after.commitDensity, unit: ' commits/week', lowerIsBetter: false, descr: 'desc_commit_density' },
        { id: 'metric_commit_size', b: before.avgCommitSize, a: after.avgCommitSize, unit: ' lines/commit', lowerIsBetter: true, descr: 'desc_commit_size' },
        { id: 'metric_commit_size_p90', b: before.commitSizeP90, a: after.commitSizeP90, unit: ' lines/commit', lowerIsBetter: true, descr: 'desc_commit_size_p90' },
        { id: 'metric_lead_time_p50', b: before.median, a: after.median, unit: ' days', lowerIsBetter: true, descr: 'desc_lead_time_p50' },
        { id: 'metric_lead_time_p90', b: before.p90, a: after.p90, unit: ' days', lowerIsBetter: true, descr: 'desc_lead_time_p90' },
        { id: 'metric_stability', b: before.stdDev, a: after.stdDev, unit: '', lowerIsBetter: true, descr: 'desc_stability' },
        { id: 'metric_rework_rate', b: before.reworkRate, a: after.reworkRate, unit: '%', lowerIsBetter: true, descr: 'desc_rework_rate' },
        { id: 'metric_response_time', b: before.responseTime, a: after.responseTime, unit: 'h', lowerIsBetter: true, descr: 'desc_impact_response_time' },
        { id: 'metric_response_time_p90', b: before.responseTimeP90, a: after.responseTimeP90, unit: 'h', lowerIsBetter: true, descr: 'desc_impact_response_time_p90' },
        { id: 'metric_review_depth', b: before.reviewDepth, a: after.reviewDepth, unit: '', lowerIsBetter: false, descr: 'desc_review_depth' },
        { id: 'metric_iterations', b: before.iterations, a: after.iterations, unit: '', lowerIsBetter: true, descr: 'desc_iterations' },
        { id: 'metric_test_ratio', b: before.testRatio, a: after.testRatio, unit: '%', lowerIsBetter: false, descr: 'desc_test_ratio' },
        { id: 'metric_steps', b: before.stepsPerWeek, a: after.stepsPerWeek, unit: ' lines/week', lowerIsBetter: false, descr: 'desc_steps' }
    ];

    metrics.forEach(m => {
        const diff = m.b > 0 ? ((m.a - m.b) / m.b) * 100 : 0;
        const isImproved = m.lowerIsBetter ? m.a < m.b : m.a > m.b;
        const status = Math.abs(diff) < 5 ? t('status_stable') : (isImproved ? t('status_improved') : t('status_declined'));
        const statusColor = Math.abs(diff) < 5 ? '#7f8c8d' : (isImproved ? '#27ae60' : '#e74c3c');

        const tr = document.createElement('tr');
        tr.innerHTML = `
                    <td>
                        <strong data-tooltip="${t(m.descr)}">${t(m.id)}</strong>
                        <span class="info-icon" style="font-size: 10px; margin-left: 4px;" data-tooltip="${t(m.descr)}">i</span>
                    </td>
                    <td>${m.b.toFixed(2)}${m.unit}</td>
                    <td>${m.a.toFixed(2)}${m.unit}</td>
                    <td style="color: ${statusColor}; font-weight: bold;">${diff > 0 ? '+' : ''}${diff.toFixed(1)}%</td>
                    <td><span class="badge" style="background: ${statusColor}22; color: ${statusColor}">${status}</span></td>
                `;
        impactTableBody.appendChild(tr);
    });

    document.getElementById('impactDescription').innerHTML = `Assessment of initiative: <strong>${event.name}</strong> (Started ${event.date})`;

    // Update all charts with vertical lines
    updateAllChartsWithEvents();
}

function updateAllChartsWithEvents(showEvents = true) {
    if (!dashboardData.events) return;

    const charts = [
        mainChart, leadTimeTrendChart, fileTypeTrendChart,
        velocitySizeChart, ctxSwitchTrendChart, ctxChart,
        reviewActivityChart, healthChart
    ];

    // If caller requests no event markers, clear annotations and return
    if (!showEvents) {
        charts.forEach(chart => {
            if (chart) {
                chart.options.plugins.annotation = { annotations: {} };
                chart.update();
            }
        });
        return;
    }

    const annotations = {};
    dashboardData.events.forEach((event, idx) => {
        annotations['line' + idx] = {
            type: 'line',
            xMin: event.date,
            xMax: event.date,
            borderColor: '#9b59b6',
            borderWidth: 2,
            borderDash: [6, 6],
            label: {
                display: true,
                content: event.name,
                position: 'start',
                backgroundColor: '#9b59b6',
                color: '#fff',
                font: { size: 10 }
            }
        };
    });

    charts.forEach(chart => {
        if (chart) {
            chart.options.plugins.annotation = { annotations };
            chart.update();
        }
    });
}

function updateUserList(filteredData) {
    if (!filteredData) {
        const startDate = document.getElementById('startDate').value;
        const endDate = document.getElementById('endDate').value;
        filteredData = data.filter(d =>
            d.dateStr >= startDate && d.dateStr <= endDate && selectedUsers.has(d.author)
        );
    }
    const userStats = {};
    filteredData.forEach(d => {
        if (!userStats[d.author]) {
            userStats[d.author] = {
                commits: 0, added: 0, deleted: 0, churn: 0, activeDays: new Set(),
                reviewsAssigned: 0, commentsGiven: 0, reviewLeadTimes: [], leadTimes: []
            };
        }
        userStats[d.author].commits += d.commit_count;
        userStats[d.author].added += d.added;
        userStats[d.author].deleted += d.deleted;
        userStats[d.author].churn += d.churn;
        userStats[d.author].activeDays.add(d.dateStr);
    });
    const currentUsers = new Set(Object.keys(userStats));

    // Aggregate Branch Lead Time from merge events
    dashboardData.merge_events.forEach(me => {
        if (currentUsers.has(me.author)) {
            userStats[me.author].leadTimes.push(me.days);
        }
    });

    // Aggregate GitHub PR data using date filters
    const startDate = document.getElementById('startDate').value;
    const endDate = document.getElementById('endDate').value;

    if (dashboardData.github_prs && dashboardData.github_prs.length > 0) {
        dashboardData.github_prs.forEach(pr => {
            const prDate = pr.created_at.split('T')[0];
            if (prDate < startDate || prDate > endDate) return;

            // Unified set of all people tasked with reviewing this PR
            const assignedSet = new Set();
            const author = normalizeAuthor(pr.author);

            // 1. People currently requested
            if (pr.review_requests) {
                pr.review_requests.forEach(req => assignedSet.add(normalizeAuthor(req)));
            }
            // 2. People who have already submitted a review
            if (pr.reviews) {
                pr.reviews.forEach(rev => {
                    if (!isBot(rev.user)) assignedSet.add(normalizeAuthor(rev.user));
                });
            }

            // Count each unique reviewer once per PR (excluding author)
            assignedSet.forEach(user => {
                if (user !== author && userStats[user]) {
                    userStats[user].reviewsAssigned++;
                }
            });

            // Review Comments (Points made) - Counted by comment date
            if (pr.review_comments) {
                pr.review_comments.forEach(comm => {
                    if (isBot(comm.user)) return;
                    const commDate = comm.created_at.split('T')[0];
                    if (commDate >= startDate && commDate <= endDate) {
                        const norm = normalizeAuthor(comm.user);
                        if (userStats[norm]) {
                            userStats[norm].commentsGiven++;
                        }
                    }
                });
            }

            // Review Lead Time (First comment to Merge)
            if (pr.merged_at && pr.review_comments && pr.review_comments.length > 0) {
                const prDate = pr.created_at.split('T')[0];
                if (prDate >= startDate && prDate <= endDate) {
                    const sortedComms = [...pr.review_comments].sort((a, b) => a.created_at.localeCompare(b.created_at));
                    const firstComm = new Date(sortedComms[0].created_at);
                    const mergedAt = new Date(pr.merged_at);
                    const diffMs = mergedAt - firstComm;
                    if (diffMs > 0) {
                        const diffDays = diffMs / (1000 * 60 * 60 * 24);
                        const prAuthor = normalizeAuthor(pr.author);
                        if (userStats[prAuthor]) {
                            userStats[prAuthor].reviewLeadTimes.push(diffDays);
                        }
                    }
                }
            }
        });
    }

    const tbody = document.getElementById('userTableBody');
    tbody.innerHTML = '';

    // Calculate aggregated metrics for sorting
    const rows = Object.entries(userStats).map(([user, s]) => {
        const totalChanges = s.added + s.deleted;
        const churn_rate = totalChanges > 0 ? (s.churn / totalChanges) * 100 : 0;
        const avgReviewLeadTime = s.reviewLeadTimes && s.reviewLeadTimes.length > 0
            ? s.reviewLeadTimes.reduce((a, b) => a + b, 0) / s.reviewLeadTimes.length
            : -1;
        const avgLeadTime = s.leadTimes.length > 0
            ? s.leadTimes.reduce((a, b) => a + b, 0) / s.leadTimes.length
            : -1;

        return {
            user,
            commits: s.commits,
            added: s.added,
            deleted: s.deleted,
            total_changes: totalChanges,
            churn_rate,
            reviewsAssigned: s.reviewsAssigned,
            commentsGiven: s.commentsGiven,
            avgReviewLeadTime,
            avgLeadTime,
            activeDays: s.activeDays.size
        };
    });

    // Sort data
    rows.sort((a, b) => {
        let vA = a[currentSort.column];
        let vB = b[currentSort.column];

        if (typeof vA === 'string') {
            const sA = vA.toLowerCase();
            const sB = vB.toLowerCase();
            return currentSort.direction === 'asc' ? sA.localeCompare(sB) : sB.localeCompare(sA);
        }

        // Push non-existent data (-1) to the bottom
        if (vA === -1 && vB === -1) return 0;
        if (vA === -1) return 1;
        if (vB === -1) return -1;

        return currentSort.direction === 'asc' ? vA - vB : vB - vA;
    });

    // Update UI headers
    document.querySelectorAll('.user-table th').forEach(th => {
        th.classList.remove('sort-asc', 'sort-desc');
    });
    const activeTh = document.getElementById('th-' + currentSort.column);
    if (activeTh) activeTh.classList.add(currentSort.direction === 'asc' ? 'sort-asc' : 'sort-desc');

    rows.forEach(r => {
        const tr = document.createElement('tr');
        tr.style.cursor = 'pointer';
        tr.onclick = () => showCommitDetails(r.user);

        const tr_churn = r.total_changes > 0 ? r.churn_rate.toFixed(1) : '0.0';
        const tr_review_lead = r.avgReviewLeadTime >= 0 ? r.avgReviewLeadTime.toFixed(1) + 'd' : '-';
        const tr_branch_lead = r.avgLeadTime >= 0 ? r.avgLeadTime.toFixed(1) + 'd' : '-';

        tr.innerHTML = `
                    <td><div class="user-info"><div class="user-avatar" style="background-color: ${stringToColor(r.user)}"></div><strong>${r.user}</strong></div></td>
                    <td>${r.commits}</td>
                    <td><span class="badge added">+${r.added.toLocaleString()}</span></td>
                    <td><span class="badge deleted">-${r.deleted.toLocaleString()}</span></td>
                    <td>${r.total_changes.toLocaleString()}</td>
                    <td><span class="badge" style="background: ${r.churn_rate > 50 ? '#fdf2f2' : '#f8f9fa'}; color: ${r.churn_rate > 50 ? '#e74c3c' : '#666'}">${tr_churn}%</span></td>
                    <td>${r.reviewsAssigned}</td>
                    <td>${r.commentsGiven}</td>
                    <td>${tr_review_lead}</td>
                    <td>${tr_branch_lead}</td>
                    <td>${r.activeDays}</td>
                `;
        tbody.appendChild(tr);
    });
}

function showCommitDetails(user) {
    const startDate = document.getElementById('startDate').value;
    const endDate = document.getElementById('endDate').value;
    const detailsDiv = document.getElementById('commitDetails');
    const detailsContent = document.getElementById('detailsContent');
    const detailsTitle = document.getElementById('detailsTitle');

    // Filter raw commits from dashboardData
    const userCommits = dashboardData.commits.filter(c => {
        const norm = normalizeAuthor(c.author);
        const date = c.date.split('T')[0];
        return norm === user && date >= startDate && date <= endDate;
    }).sort((a, b) => b.date.localeCompare(a.date));

    detailsTitle.innerHTML = `${t('label_commits_by')} <strong>${user}</strong> (${userCommits.length})`;
    detailsDiv.style.display = 'block';

    if (userCommits.length === 0) {
        detailsContent.innerHTML = '<p>No commits found for the selected period.</p>';
        return;
    }

    function escapeHtml(text) {
        const div = document.createElement('div');
        div.textContent = text;
        return div.innerHTML;
    }

    detailsContent.innerHTML = `
                <table class="user-table" style="font-size: 12px;">
                    <thead>
                        <tr>
                            <th>${t('header_hash')}</th>
                            <th>${t('header_date')}</th>
                            <th>${t('header_message')}</th>
                            <th>${t('header_added')}</th>
                            <th>${t('header_deleted')}</th>
                            <th>${t('header_files')}</th>
                        </tr>
                    </thead>
                    <tbody>
                        ${userCommits.map(c => `
                            <tr style="${c.is_merge ? 'background-color: #fcfaff;' : ''}">
                                <td style="font-family: monospace; color: #7f8c8d;">${c.hash.substring(0, 7)}</td>
                                <td style="white-space: nowrap;">${c.date.split('T')[0]}</td>
                                <td style="max-width: 400px; overflow: hidden; text-overflow: ellipsis;" title="${escapeHtml(c.message)}">
                                    ${c.is_merge ? '🔀 ' : ''}${escapeHtml(c.message)}
                                </td>
                                <td class="badge added">+${c.added}</td>
                                <td class="badge deleted">-${c.deleted}</td>
                                <td style="font-size: 10px; color: #666;">
                                    ${c.files ? c.files.slice(0, 5).map(fidx => {
        const path = filePaths[fidx] || '';
        const parts = path.split('/');
        return `<span title="${path}">${parts.pop()}</span>`;
    }).join(', ') : '-'}
                                    ${c.files && c.files.length > 5 ? '...' : ''}
                                </td>
                            </tr>
                        `).join('')}
                    </tbody>
                </table>
            `;
    detailsDiv.scrollIntoView({ behavior: 'smooth' });
}
