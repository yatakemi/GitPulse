pub const HTML_TEMPLATE: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Git Productivity Report</title>
    <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
    <script src="https://cdn.jsdelivr.net/npm/chartjs-chart-matrix@2.0.1"></script>
    <script src="https://cdn.jsdelivr.net/npm/chartjs-plugin-annotation@3.0.1"></script>
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

        /* Insight Cards */
        .insights-section { margin-bottom: 25px; }
        .insights-section h2 { font-size: 18px; color: #2c3e50; margin-bottom: 15px; display: flex; align-items: center; gap: 8px; }
        .insights-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(350px, 1fr)); gap: 15px; }
        .insight-card {
            padding: 16px 20px; border-radius: 10px; display: flex; gap: 12px; align-items: flex-start;
            box-shadow: 0 2px 4px rgba(0,0,0,0.05); border-left: 4px solid;
        }
        .insight-card.warning { background: #fef9e7; border-color: #f39c12; }
        .insight-card.info { background: #eaf2f8; border-color: #3498db; }
        .insight-card.positive { background: #eafaf1; border-color: #27ae60; }
        .insight-icon { font-size: 24px; flex-shrink: 0; line-height: 1; }
        .insight-body { flex: 1; }
        .insight-title { font-weight: 600; font-size: 14px; color: #2c3e50; margin-bottom: 4px; }
        .insight-desc { font-size: 13px; color: #555; line-height: 1.5; }
        .insight-value { font-weight: 700; color: #2c3e50; }
        
        .user-table {
            width: 100%;
            border-collapse: collapse;
            margin-top: 10px;
        }
        .user-table th, .user-table td {
            padding: 12px;
            text-align: left;
            border-bottom: 1px solid #eee;
        }
        .user-table th {
            font-weight: 600;
            color: #7f8c8d;
            font-size: 0.85rem;
            text-transform: uppercase;
            cursor: pointer;
            user-select: none;
            position: relative;
            padding-right: 20px;
        }
        .user-table th:hover { background-color: #f1f1f1; color: #2c3e50; }
        .user-table th::after {
            content: '↕';
            position: absolute;
            right: 8px;
            opacity: 0.3;
            font-size: 0.7rem;
        }
        .user-table th.sort-asc::after { content: '▲'; opacity: 1; color: #3498db; }
        .user-table th.sort-desc::after { content: '▼'; opacity: 1; color: #3498db; }
        
        .user-table tr:hover {
            background-color: #f9f9f9;
        }
        .user-table .user-info {
            display: flex;
            align-items: center;
        }
        .user-table .user-avatar {
            width: 24px;
            height: 24px;
            border-radius: 50%;
            margin-right: 10px;
        }
        .badge {
            padding: 2px 8px;
            border-radius: 12px;
            font-size: 0.75rem;
            font-weight: 600;
        }
        .badge.added { background: #ecfaf2; color: #27ae60; }
        .badge.deleted { background: #fdf2f2; color: #e74c3c; }
        
        /* User Selection Styles */
        .user-selection-area {
            background: white; padding: 20px; border-radius: 12px; box-shadow: 0 4px 6px rgba(0,0,0,0.05); 
            margin-bottom: 25px;
        }
        .user-selection-header {
            display: flex; justify-content: space-between; align-items: center; margin-bottom: 15px;
        }
        .user-selection-header h2 { font-size: 16px; margin: 0; color: #2c3e50; }
        .bulk-controls { display: flex; gap: 10px; }
        .btn-small {
            padding: 4px 10px; font-size: 12px; border-radius: 4px; border: 1px solid #ddd;
            background: #f8f9fa; cursor: pointer; transition: all 0.2s;
        }
        .btn-small:hover { background: #e9ecef; border-color: #ced4da; }
        
        .user-checkbox-grid {
            display: grid; grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
            gap: 10px; max-height: 150px; overflow-y: auto; padding: 5px;
        }
        .user-checkbox-item {
            display: flex; align-items: center; gap: 8px; font-size: 13px;
            padding: 4px 8px; border-radius: 6px; cursor: pointer;
            transition: background 0.2s;
        }
        .user-checkbox-item:hover { background: #f1f8ff; }
        .user-checkbox-item input { margin: 0; cursor: pointer; }
        .user-checkbox-item .color-dot { width: 8px; height: 8px; border-radius: 50%; }

        /* Forecast Styles */
        .forecast-grid {
            display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
            gap: 20px; margin-bottom: 25px;
        }
        .forecast-card {
            background: linear-gradient(135deg, #f8f9fa 0%, #e9ecef 100%);
            padding: 20px; border-radius: 12px; border-left: 4px solid #3498db;
        }
        .forecast-value { font-size: 24px; font-weight: bold; color: #2c3e50; margin: 10px 0; }
        .forecast-label { font-size: 14px; color: #7f8c8d; }
        .forecast-trend { font-size: 14px; font-weight: bold; }
        .forecast-trend.up { color: #27ae60; }
        .forecast-trend.down { color: #e74c3c; }
        
        .goal-setter {
            background: #fff; padding: 15px; border-radius: 8px; border: 1px solid #eee;
            margin-top: 10px; display: flex; align-items: center; gap: 10px;
        }
        .goal-setter input {
            padding: 5px 10px; border: 1px solid #ddd; border-radius: 4px; width: 80px;
        }
        
    </style>
</head>
<body>
    <div class="container">
        <h1>Git Productivity Report</h1>
        
        <div class="controls">
            <div class="control-group">
                <label data-i18n="language">Language:</label>
                <select id="langSelect" onchange="updateLanguage(this.value)">
                    <option value="en">English</option>
                    <option value="ja">日本語</option>
                </select>
            </div>
            <div class="control-group">
                <label data-i18n="metric">Metric:</label>
                <select id="metricSelect" onchange="updateDashboard()">
                    <option value="total_changes" data-i18n="metric_total">Total Changes</option>
                    <option value="added" data-i18n="metric_added">Added Lines</option>
                    <option value="deleted" data-i18n="metric_deleted">Deleted Lines</option>
                    <option value="commit_count" data-i18n="metric_commits">Commit Count</option>
                    <option value="churn" data-i18n="metric_churn">Code Churn (Volatility)</option>
                </select>
            </div>
            
            <div class="control-group">
                <label data-i18n="chart_type">Chart:</label>
                <select id="chartTypeSelect" onchange="updateDashboard()">
                    <option value="line" data-i18n="chart_line">Line Chart</option>
                    <option value="bar" data-i18n="chart_bar">Stacked Bar</option>
                </select>
            </div>
            <div class="control-group">
                <label data-i18n="start">Start:</label>
                <input type="date" id="startDate" onchange="updateDashboard()">
            </div>
            <div class="control-group">
                <label data-i18n="end">End:</label>
                <input type="date" id="endDate" onchange="updateDashboard()">
            </div>
            <div class="control-group">
                <input type="checkbox" id="showTrend" onchange="updateDashboard()">
                <label for="showTrend" data-i18n="trend">7-Day Trend</label>
            </div>
        </div>

        <div class="summary-cards">
             <div class="card">
                <h3 id="summaryTitle" data-i18n="sum_total">Total</h3>
                <div class="value" id="summaryValue">-</div>
                <div class="diff" id="summaryDiff">-</div>
            </div>
            <div class="card">
                <h3 data-i18n="sum_merge">Merge Commits</h3>
                <div class="value" id="mergeCommitsValue">-</div>
             </div>
             <div class="card">
                <h3 data-i18n="sum_churn">Churn Rate</h3>
                <div class="value" id="churnRateValue">-</div>
             </div>
            <div class="card">
                <h3 data-i18n="sum_active">Active Days</h3>
                <div class="value" id="activeDaysValue">-</div>
            </div>
            <div class="card">
                <h3 data-i18n="sum_avg">Avg / Day</h3>
                <div class="value" id="avgPerDayValue">-</div>
            </div>
        </div>

        <!-- NEW: Advanced GitHub Summary -->
        <div class="summary-cards" id="githubAdvancedSummary" style="display: none; margin-top: -10px; margin-bottom: 30px;">
            <div class="card" style="border-top: 4px solid #e67e22;">
                <h3 style="display: flex; align-items: center; justify-content: center; gap: 5px;">
                    <span data-i18n="sum_rework_rate">Rework Rate</span>
                    <span class="info-icon" data-i18n-tooltip="tooltip_rework_rate" data-tooltip="Percentage of PRs that received a 'Request Changes' status. Indicates how often work needs to be redone.">i</span>
                </h3>
                <div class="value" id="reworkRateValue">-</div>
                <div style="font-size: 11px; color: #7f8c8d;" data-i18n="desc_rework_prs">% of PRs with Request Changes</div>
            </div>
            <div class="card" style="border-top: 4px solid #e67e22;">
                <h3 style="display: flex; align-items: center; justify-content: center; gap: 5px;">
                    <span data-i18n="sum_review_depth">Review Depth</span>
                    <span class="info-icon" data-i18n-tooltip="tooltip_review_depth" data-tooltip="Average number of comments per PR. Measures the thoroughness of the review process.">i</span>
                </h3>
                <div class="value" id="reviewDepthValue">-</div>
                <div style="font-size: 11px; color: #7f8c8d;" data-i18n="desc_avg_comments">Avg Comments / PR</div>
            </div>
            <div class="card" style="border-top: 4px solid #e67e22;">
                <h3 style="display: flex; align-items: center; justify-content: center; gap: 5px;">
                    <span data-i18n="sum_response_time">Avg Response Time</span>
                    <span class="info-icon" data-i18n-tooltip="tooltip_response_time" data-tooltip="Average time from PR creation to the very first review or comment. Measures waiting time for developers.">i</span>
                </h3>
                <div class="value" id="avgResponseTimeValue">-</div>
                <div style="font-size: 11px; color: #7f8c8d;" data-i18n="desc_first_reaction">Time to first reaction</div>
            </div>
            <div class="card" style="border-top: 4px solid #e67e22;">
                <h3 style="display: flex; align-items: center; justify-content: center; gap: 5px;">
                    <span data-i18n="sum_iterations">Avg Iterations</span>
                    <span class="info-icon" data-i18n-tooltip="tooltip_iterations" data-tooltip="Average number of review-and-fix cycles per PR. High iterations suggest complex tasks or unclear requirements.">i</span>
                </h3>
                <div class="value" id="avgIterationsValue">-</div>
                <div style="font-size: 11px; color: #7f8c8d;" data-i18n="desc_review_cycles">Review-to-Merge cycles</div>
            </div>
        </div>

        <div class="user-selection-area">
            <div class="user-selection-header">
                <h2><span data-i18n="title_user_selection">Filter by Users</span></h2>
                <div class="bulk-controls">
                    <button class="btn-small" onclick="selectAllUsers(true)" data-i18n="btn_select_all">Select All</button>
                    <button class="btn-small" onclick="selectAllUsers(false)" data-i18n="btn_select_none">Select None</button>
                </div>
            </div>
            <div class="user-checkbox-grid" id="userCheckboxes">
                <!-- Populated by JS -->
            </div>
        </div>

        <!-- File Type Analysis Section -->
        <div class="charts-grid">
            <div class="chart-box">
                <div class="chart-title">
                    <span data-i18n="chart_file_type_share">File Type Share</span>
                    <span class="info-icon" data-tooltip="Distribution of work across different file extensions.">i</span>
                </div>
                <canvas id="fileTypeChart"></canvas>
            </div>
            <div class="chart-box" style="height: auto; overflow-y: auto;">
                <div class="chart-title">
                    <span data-i18n="title_file_type_list">File Type Details</span>
                </div>
                <div style="margin-top: 40px;">
                    <table class="user-table" style="font-size: 12px;">
                        <thead>
                            <tr>
                                <th data-i18n="header_ext">Ext</th>
                                <th data-i18n="header_added">Added</th>
                                <th data-i18n="header_deleted">Deleted</th>
                                <th data-i18n="header_churn_rate">Churn%</th>
                            </tr>
                        </thead>
                        <tbody id="fileTypeTableBody">
                            <!-- Populated by JS -->
                        </tbody>
                    </table>
                </div>
            </div>
        </div>

        <!-- Impact Assessment Section -->
        <div id="impactSection" class="card" style="max-width: none; margin-bottom: 25px; border-top: 4px solid #9b59b6; display: none;">
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 15px;">
                <h2 style="font-size: 18px; color: #2c3e50; margin: 0;">🚀 <span data-i18n="title_impact_assessment">Initiative Impact Assessment</span></h2>
                <div id="eventSelectorContainer" style="display: flex; align-items: center; gap: 10px;">
                    <label style="font-size: 13px; color: #7f8c8d;">Select Initiative:</label>
                    <select id="eventSelect" onchange="updateImpactAssessment(this.value)" style="padding: 4px 8px; font-size: 13px;"></select>
                </div>
            </div>
            <div style="overflow-x: auto;">
                <table class="user-table" id="impactTable">
                    <thead>
                        <tr>
                            <th data-i18n="header_metric">Metric</th>
                            <th data-i18n="header_before">Before Initiative</th>
                            <th data-i18n="header_after">After Initiative</th>
                            <th data-i18n="header_change">Change (Δ%)</th>
                            <th data-i18n="header_status">Status</th>
                        </tr>
                    </thead>
                    <tbody id="impactTableBody">
                        <!-- Populated by JS -->
                    </tbody>
                </table>
            </div>
            <div id="impactDescription" style="margin-top: 15px; font-size: 13px; color: #666; line-height: 1.6;"></div>
            
            <div style="margin-top: 20px; padding-top: 15px; border-top: 1px solid #eee; display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 15px; font-size: 12px; color: #7f8c8d;">
                <div>
                    <strong data-i18n="label_throughput">Throughput</strong>: <span data-i18n="desc_throughput">Measures volume. Higher means the team is delivering more completed work.</span>
                </div>
                <div>
                    <strong data-i18n="label_p90">P90 Lead Time</strong>: <span data-i18n="desc_p90">Worst-case scenario. Lower means fewer PRs are being "stuck" or forgotten.</span>
                </div>
                <div>
                    <strong data-i18n="label_stability">Stability</strong>: <span data-i18n="desc_stability">Standard deviation of Lead Time. Lower means the process is consistent and predictable.</span>
                </div>
                <div>
                    <strong data-i18n="label_rework_rate_label">Rework Rate</strong>: <span data-i18n="desc_rework">Percentage of PRs needing fixes. Lower suggests better alignment before code is written.</span>
                </div>
            </div>
        </div>

        <!-- Predictive Analysis Section -->
        <div class="card" style="max-width: none; margin-bottom: 25px;">
            <h2 style="font-size: 18px; color: #2c3e50; margin-bottom: 20px;">🔮 <span data-i18n="title_predictive_analysis">Predictive Analysis</span></h2>
            <div class="forecast-grid">
                <div class="forecast-card">
                    <div class="forecast-label" data-i18n="label_current_velocity">Current Velocity</div>
                    <div class="forecast-value" id="currentVelocityValue">-</div>
                    <div class="forecast-trend" id="velocityTrendValue">-</div>
                </div>
                <div class="forecast-card">
                    <div class="forecast-label" data-i18n="label_projected_throughput">Projected 60-Day Throughput</div>
                    <div class="forecast-value" id="projectedThroughputValue">-</div>
                </div>
                <div class="forecast-card">
                    <div class="forecast-label" data-i18n="label_est_completion">Estimated Completion Date</div>
                    <div class="forecast-value" id="estCompletionValue">-</div>
                    <div id="estCompletionRange" style="font-size: 12px; color: #7f8c8d; margin-top: 5px;"></div>
                    <div class="goal-setter">
                        <span data-i18n="label_remaining_work">Remaining Work</span>
                        <input type="number" id="remainingWorkInput" value="100" onchange="updateDashboard()">
                        <span style="font-size: 12px; color: #7f8c8d;">commits</span>
                    </div>
                </div>
            </div>
            <div class="chart-box full-width" style="box-shadow: none; padding: 0;">
                <div class="chart-title">
                    <span data-i18n="forecast_chart_title">Velocity Forecasting</span>
                    <span class="info-icon" data-i18n-tooltip="tooltip_forecast" data-tooltip="Predicts future output based on the last 4 weeks of velocity. The dotted line shows the projected trend. Change the 'Remaining Work' to see an estimated completion date.">i</span>
                </div>
                <canvas id="forecastChart" style="height: 300px;"></canvas>
            </div>
        </div>

        <div class="insights-section" id="insightsContainer">
            <h2>💡 <span data-i18n="insights_title">Insights</span></h2>
            <div class="insights-grid" id="insightsGrid"></div>
        </div>

        <div class="charts-grid">
            <div class="chart-box full-width">
                <div class="chart-title">
                    <span data-i18n="chart_timeline">Timeline</span> 
                    <span class="info-icon" data-i18n-tooltip="tooltip_timeline" data-tooltip="Shows activity trends over time.">i</span>
                </div>
                <canvas id="productivityChart"></canvas>
            </div>
            <div class="chart-box">
                <div class="chart-title">
                    <span data-i18n="chart_share">User Share</span>
                    <span class="info-icon" data-i18n-tooltip="tooltip_share" data-tooltip="Distribution of contributions.">i</span>
                </div>
                <canvas id="shareChart"></canvas>
            </div>
            <div class="chart-box">
                <div class="chart-title">
                    <span data-i18n="chart_dow">Day of Week Activity</span>
                    <span class="info-icon" data-i18n-tooltip="tooltip_dow" data-tooltip="Weekly rhythm.">i</span>
                </div>
                <canvas id="dayOfWeekChart"></canvas>
            </div>
            <div class="chart-box full-width">
                <div class="chart-title">
                    <span data-i18n="chart_heatmap">Activity Heatmap (Hour vs Day)</span>
                    <span class="info-icon" data-i18n-tooltip="tooltip_heatmap" data-tooltip="Identifies core working hours.">i</span>
                </div>
                <canvas id="heatmapChart"></canvas>
            </div>
             <div class="chart-box full-width">
                <div class="chart-title">
                    <span data-i18n="chart_size">Commit Size Distribution</span>
                    <span class="info-icon" data-i18n-tooltip="tooltip_size" data-tooltip="Breakdown of commit sizes.">i</span>
                </div>
                <canvas id="sizeDistChart"></canvas>
            </div>
            <div class="chart-box full-width">
                <div class="chart-title">
                    <span data-i18n="chart_duration">Est. Daily Work Duration</span>
                    <span class="info-icon" data-i18n-tooltip="tooltip_duration" data-tooltip="Time between first and last commit of the day.">i</span>
                </div>
                <canvas id="workDurationChart"></canvas>
            </div>
             <div class="chart-box full-width">
                <div class="chart-title">
                    <span data-i18n="chart_health">Team Health Trends</span>
                    <span class="info-icon" data-i18n-tooltip="tooltip_health" data-tooltip="Red: Churn Rate (Rework). Calculated as 2 * min(added, deleted) / total changes. Purple: Avg Duration. Rising trend in both indicates technical debt or crunch.">i</span>
                </div>
                <canvas id="healthTrendChart"></canvas>
            </div>
            <div class="chart-box full-width">
                <div class="chart-title">
                    <span data-i18n="chart_ownership">Code Ownership (Top 15 Files)</span>
                    <span class="info-icon" data-i18n-tooltip="tooltip_ownership" data-tooltip="Shows who contributes to which files.">i</span>
                </div>
                <canvas id="ownershipChart"></canvas>
            </div>
            <div class="chart-box full-width" style="height: auto; min-height: 200px;">
                <div class="chart-title">
                    <span data-i18n="title_isolated_files">Isolated Files (Bus Factor Risk)</span>
                    <span class="info-icon" data-i18n-tooltip="tooltip_isolated" data-tooltip="Files that have been modified by only one person.">i</span>
                </div>
                <div style="margin-top: 45px; overflow-x: auto;">
                    <table class="user-table" style="font-size: 12px;">
                        <thead>
                            <tr>
                                <th data-i18n="header_file_path">File Path</th>
                                <th data-i18n="header_sole_contributor">Sole Contributor</th>
                                <th data-i18n="header_mod_count">Modifications</th>
                            </tr>
                        </thead>
                        <tbody id="isolatedFilesTableBody">
                            <!-- Populated by JS -->
                        </tbody>
                    </table>
                </div>
            </div>
            <div class="chart-box full-width">
                <div class="chart-title">
                    <span data-i18n="chart_leadtime">Branch Lead Time</span>
                    <span class="info-icon" data-i18n-tooltip="tooltip_leadtime" data-tooltip="Time span of merged branches (from the very first commit in the branch to the merge commit). Sync merges from base branches (main/develop) are excluded.">i</span>
                </div>
                <canvas id="leadTimeChart" style="margin-bottom: 40px;"></canvas>
                <div id="leadTimeStats" style="position: absolute; bottom: 15px; left: 20px; right: 20px; display: flex; justify-content: center; gap: 30px; font-size: 13px; color: #7f8c8d; border-top: 1px solid #f5f5f5; padding-top: 10px;">
                    <!-- Populated by JS -->
                </div>
            </div>
            <div class="chart-box full-width">
                <div class="chart-title">
                    <span>Review Activity (Comments Given)</span>
                    <span class="info-icon" data-tooltip="Number of review comments (initial thread comments) given over time.">i</span>
                </div>
                <canvas id="reviewActivityChart"></canvas>
            </div>
            <!-- Advanced GitHub Visuals -->
            <div class="chart-box full-width" id="reciprocityBox" style="display: none;">
                <div class="chart-title">
                    <span data-i18n="chart_reciprocity">Review Reciprocity Matrix</span>
                    <span class="info-icon" data-tooltip="Who reviews whom. Vertical axis: Author, Horizontal axis: Reviewer. High numbers indicate strong review pairings. Useful for spotting silos.">i</span>
                </div>
                <canvas id="reciprocityChart"></canvas>
            </div>
            <div class="chart-box full-width" id="scatterBox" style="display: none;">
                <div class="chart-title">
                    <span data-i18n="chart_scatter">PR Size vs Lead Time</span>
                    <span class="info-icon" data-tooltip="Correlation between PR size (Additions + Deletions) and Lead Time. Ideally, smaller PRs should have lower lead times.">i</span>
                </div>
                <canvas id="scatterChart"></canvas>
            </div>
            <!-- Distribution Analysis -->
            <div class="charts-grid" id="distBox" style="display: none;">
                <div class="chart-box">
                    <div class="chart-title">
                        <span data-i18n="chart_res_dist">Response Time Distribution</span>
                        <span class="info-icon" data-tooltip="Frequency of Response Times (in hours). Most values should be on the left (low waiting time).">i</span>
                    </div>
                    <canvas id="resDistChart"></canvas>
                </div>
                <div class="chart-box">
                    <div class="chart-title">
                        <span data-i18n="chart_lead_dist">Lead Time Distribution</span>
                        <span class="info-icon" data-tooltip="Frequency of Branch Lead Times (in days). Identifies consistency of the development cycle.">i</span>
                    </div>
                    <canvas id="leadDistChart"></canvas>
                </div>
            </div>
            <div class="chart-box full-width">
                <div class="chart-title">
                    <span data-i18n="chart_ctxswitch">Context Switching (Daily Directory Diversity)</span>
                    <span class="info-icon" data-i18n-tooltip="tooltip_ctxswitch" data-tooltip="Number of distinct directories touched per day.">i</span>
                </div>
                <canvas id="ctxSwitchChart"></canvas>
            </div>
        </div>

        <!-- User List Section -->
        <div class="card" style="max-width: none; margin-bottom: 30px;">
            <h2 data-i18n="title_user_list" style="font-size: 18px; color: #2c3e50; margin-bottom: 15px; text-align: left;">User Activity Details</h2>
            <div style="overflow-x: auto;">
                <table class="user-table" id="userTable">
                    <thead>
                        <tr>
                            <th onclick="toggleSort('author')" id="th-author" data-i18n="header_author">Author</th>
                            <th onclick="toggleSort('commits')" id="th-commits">
                                <span data-i18n="header_commits">Commits</span>
                                <span class="info-icon" data-i18n-tooltip="tooltip_user_commits" data-tooltip="Total number of commits.">i</span>
                            </th>
                            <th onclick="toggleSort('added')" id="th-added">
                                <span data-i18n="header_added">Added</span>
                                <span class="info-icon" data-i18n-tooltip="tooltip_user_added" data-tooltip="Total lines added.">i</span>
                            </th>
                            <th onclick="toggleSort('deleted')" id="th-deleted">
                                <span data-i18n="header_deleted">Deleted</span>
                                <span class="info-icon" data-i18n-tooltip="tooltip_user_deleted" data-tooltip="Total lines deleted.">i</span>
                            </th>
                            <th onclick="toggleSort('total_changes')" id="th-total_changes" data-i18n="header_total_changes">Total Changes</th>
                            <th onclick="toggleSort('churn_rate')" id="th-churn_rate">
                                <span data-i18n="sum_churn">Churn Rate</span>
                                <span class="info-icon" data-i18n-tooltip="tooltip_user_churn" data-tooltip="Rework rate.">i</span>
                            </th>
                            <th onclick="toggleSort('reviewsAssigned')" id="th-reviewsAssigned">
                                <span data-i18n="header_reviews">Reviews (Assigned)</span>
                                <span class="info-icon" data-i18n-tooltip="tooltip_user_reviews" data-tooltip="PRs assigned or reviewed.">i</span>
                            </th>
                            <th onclick="toggleSort('commentsGiven')" id="th-commentsGiven">
                                <span data-i18n="header_comments">Review Comments</span>
                                <span class="info-icon" data-i18n-tooltip="tooltip_user_comments" data-tooltip="Points made in reviews.">i</span>
                            </th>
                            <th onclick="toggleSort('avgReviewLeadTime')" id="th-avgReviewLeadTime">
                                <span data-i18n="header_review_lead_time">Review Lead Time</span>
                                <span class="info-icon" data-i18n-tooltip="tooltip_user_review_lead" data-tooltip="Avg days from first comment to merge.">i</span>
                            </th>
                            <th onclick="toggleSort('avgLeadTime')" id="th-avgLeadTime">
                                <span data-i18n="header_avg_lead_time">Branch Lead Time</span>
                                <span class="info-icon" data-i18n-tooltip="tooltip_user_branch_lead" data-tooltip="Avg days from first commit to merge.">i</span>
                            </th>
                            <th onclick="toggleSort('activeDays')" id="th-activeDays">
                                <span data-i18n="header_active_days">Active Days</span>
                                <span class="info-icon" data-i18n-tooltip="tooltip_user_active_days" data-tooltip="Total days active.">i</span>
                            </th>
                        </tr>
                    </thead>
                    <tbody id="userTableBody">
                        <!-- Populated by JS -->
                    </tbody>
                </table>
            </div>
            <!-- Details section for commits -->
            <div id="commitDetails" style="margin-top: 25px; display: none; background: #fdfdfd; padding: 20px; border-radius: 12px; border: 1px solid #eee;">
                <h3 id="detailsTitle" style="font-size: 16px; margin-top: 0;" data-i18n="title_commit_details">Commit Details</h3>
                <div id="detailsContent" style="max-height: 400px; overflow-y: auto;"></div>
            </div>
        </div>
    </div>

    <script>
        const translations = {
            en: {
                title: "Git Productivity Report",
                language: "Language:",
                metric: "Metric:",
                metric_total: "Total Changes",
                metric_added: "Added Lines",
                metric_deleted: "Deleted Lines",
                metric_commits: "Commit Count",
                metric_churn: "Code Churn (Volatility)",
                chart_type: "Chart:",
                chart_line: "Line Chart",
                chart_bar: "Stacked Bar",
                start: "Start:",
                end: "End:",
                trend: "7-Day Trend",
                sum_total: "Total",
                sum_merge: "Merge Commits",
                sum_churn: "Churn Rate",
                sum_active: "Active Days",
                sum_avg: "Avg / Day",
                chart_timeline: "Timeline",
                chart_share: "User Share",
                chart_dow: "Day of Week Activity",
                chart_heatmap: "Activity Heatmap (Hour vs Day)",
                chart_size: "Commit Size Distribution",
                chart_duration: "Est. Daily Work Duration",
                chart_health: "Team Health Trends",
                tooltip_timeline: "Shows activity trends over time. Look for spikes (sprints/releases) or gaps (blockers/downtime). Ideally, activity should be consistent. Spike in deletions might indicate cleanup/refactoring.",
                tooltip_share: "Distribution of contributions. Helps identify 'Bus Factor' (reliance on single dev). A highly skewed chart suggests high risk if the top contributor is unavailable.",
                tooltip_dow: "Weekly rhythm. Most teams peak Tue-Thu. High weekend activity might indicate crunch time, unhealthy work habits, or upcoming release pressure.",
                tooltip_heatmap: "Identifies core working hours. Look for clusters outside normal hours (e.g. late nights), which suggests overtime or burnout risk. Inconsistent heatmaps might indicate lack of overlapping hours for collaboration.",
                tooltip_size: "Breakdown of commit sizes. XS: <10, S: 10-50, M: 50-200, L: 200-500, XL: >500 lines. 'XS'/'S' are ideal (atomic commits). Too many 'XL' suggests large, risky changes that are hard to review and more likely to contain bugs.",
                tooltip_duration: "Time between first and last commit of the day. NOTE: Not actual work hours, but indicates the span of activity. Long spans (>8h) consistently may suggest burnout risk.",
                tooltip_health: "Red: Churn Rate (Rework/Volatility). High = Unstable/Refactoring. Calculated as 2 * min(added, deleted) / total changes. Purple: Avg Duration. Rising trend in both often indicates 'Technical Debt' or 'Crunch Time'.",
                tooltip_ownership: "Shows who contributes to which files. The top 15 most frequently modified files (hotspots) are shown. These are potential architectural bottlenecks, 'God Classes', or unstable modules needing refactoring or better tests. Files with only one contributor are a 'Bus Factor' risk. Balanced ownership improves team resilience and knowledge sharing.",
                tooltip_leadtime: "Time span of merged branches (from first commit to merge). Shorter lead times indicate faster delivery. Long-lived branches increase merge complexity and risk.",
                tooltip_ctxswitch: "Number of distinct directories touched per day. High values indicate frequent context switching, which reduces focus and deep work productivity.",
                tooltip_forecast: "Predicts future output based on the last 4 weeks of velocity. The dotted line shows the projected trend. Change the 'Target Goal' to see an estimated completion date.",
                label_activity: "Activity", 
                label_commit_count: "Commit Count",
                label_mod_count: "Modification Count",
                label_days_count: "Days Count",
                label_churn_rate: "Churn Rate (%)",
                label_avg_duration: "Avg Work Duration (Hours)",
                diff_new: "New Activity",
                diff_prev: "vs prev",
                insights_title: "Insights",
                insight_burnout_title: "Burnout Risk",
                insight_burnout_desc: "Average work span in the last 7 days is {value} hours.",
                insight_unstable_title: "Code Instability",
                insight_unstable_desc: "Churn Rate is {value}%.",
                insight_busfactor_title: "Bus Factor Risk",
                insight_busfactor_desc: "{name} accounts for {value}% of commits.",
                insight_largecommit_title: "Large Commit Pattern",
                insight_largecommit_desc: "{value}% of commits are XL.",
                insight_hotspot_title: "Hotspot Concentration",
                insight_hotspot_desc: "Top 3 files account for {value}% of all changes.",
                insight_weekend_title: "Weekend Work",
                insight_weekend_desc: "{value}% of commits are on weekends.",
                insight_stable_title: "Stable Pace",
                insight_stable_desc: "Active on {value}% of days.",
                insight_smallcommit_title: "Good Commit Habits",
                insight_smallcommit_desc: "{value}% of commits are XS/S size.",
                insight_latenight_title: "Late Night Activity",
                insight_latenight_desc: "{value}% of commits are between 10PM-5AM.",
                chart_ownership: "Code Ownership (Top 15 Files)",
                tooltip_ownership: "Shows who contributes to which files.",
                label_commits: "commits",
                insight_isolated_title: "Isolated Files",
                insight_isolated_desc: "{value} file(s) are only touched by one person.",
                chart_leadtime: "Branch Lead Time",
                tooltip_leadtime: "Time span of merged branches (from the very first commit in the branch to the merge commit). Sync merges from base branches (main/develop) are excluded to ensure accuracy. Shorter lead times indicate faster delivery. Long-lived branches increase merge complexity.",
                label_leadtime_avg: "Average",
                label_leadtime_median: "Median",
                label_leadtime_p90: "P90 (Worst)",
                label_days: "days",
                label_branch: "Branch",
                label_leadtime_days: "Lead Time (Days)",
                chart_ctxswitch: "Context Switching",
                tooltip_ctxswitch: "Distinct directories touched per day.",
                label_avg_dirs: "Avg Directories / Day",
                insight_ctxswitch_title: "Frequent Context Switching",
                insight_ctxswitch_desc: "Average {value} directories touched per day.",
                insight_longlived_title: "Long-lived Branches",
                insight_longlived_desc: "{value} branch(es) lived longer than 7 days.",
                header_active_days: "Active Days",
                header_total_changes: "Total Changes",
                header_reviews: "Reviews (Assigned)",
                header_comments: "Review Comments",
                header_review_lead_time: "Review Lead Time",
                header_top_dirs: "Top Dirs",
                header_avg_lead_time: "Branch Lead Time",
                btn_select_all: "Select All",
                title_user_selection: "Filter by Users",
                title_predictive_analysis: "Predictive Analysis (BETA)",
                label_current_velocity: "Current Velocity",
                label_projected_throughput: "Projected 60-Day Throughput",
                label_remaining_work: "Remaining Work",
                label_est_completion: "Estimated Completion Date",
                forecast_chart_title: "Velocity Forecasting",
                insight_predicted_goal_title: "Target Forecast",
                insight_predicted_goal_desc: "You will complete the remaining {remaining} commits by {date}.",
                sum_rework_rate: "Rework Rate",
                sum_review_depth: "Review Depth",
                sum_response_time: "Avg Response Time",
                sum_iterations: "Avg Iterations",
                title_isolated_files: "Isolated Files (Bus Factor Risk)",
                tooltip_isolated: "Files that have been modified by only one person in the repository. These represent potential knowledge silos.",
                header_file_path: "File Path",
                header_sole_contributor: "Sole Contributor",
                header_mod_count: "Modifications",
                title_commit_details: "Commit Details",
                label_commits_by: "Commits by",
                header_date: "Date",
                header_message: "Message",
                header_hash: "Hash",
                header_files: "Files",
                desc_rework_prs: "% of PRs with Request Changes",
                desc_avg_comments: "Avg Comments / PR",
                desc_first_reaction: "Time to first reaction",
                desc_review_cycles: "Review-to-Merge cycles",
                tooltip_rework_rate: "Percentage of PRs that received a 'Request Changes' status. Indicates how often work needs to be redone.",
                tooltip_review_depth: "Average number of comments per PR. Measures the thoroughness of the review process.",
                tooltip_response_time: "Average time from PR creation to the very first review or comment. Measures waiting time for developers.",
                tooltip_iterations: "Average number of review-and-fix cycles per PR. High iterations suggest complex tasks or unclear requirements.",
                tooltip_user_commits: "Total number of commits made by the user in the selected period.",
                tooltip_user_added: "Total lines of code added.",
                tooltip_user_deleted: "Total lines of code deleted.",
                tooltip_user_churn: "Rework rate. Higher values indicate more frequent changes to the same code blocks.",
                tooltip_user_reviews: "Number of Pull Requests where the user was assigned as a reviewer or submitted a review.",
                tooltip_user_comments: "Number of initial review comments (points made). Replies are excluded.",
                tooltip_user_review_lead: "Average days from the user's first comment to the PR being merged.",
                tooltip_user_branch_lead: "Average days from the user's first commit in a branch to the final merge.",
                tooltip_user_active_days: "Total number of unique days the user made at least one commit.",
                header_ext: "Ext",
                header_churn_rate: "Churn%",
                label_throughput: "Throughput",
                label_p90: "P90 Lead Time",
                label_stability: "Stability",
                label_rework_rate_label: "Rework Rate",
                chart_reciprocity: "Review Reciprocity Matrix",
                chart_scatter: "PR Size vs Lead Time",
                chart_res_dist: "Response Time Distribution",
                chart_lead_dist: "Lead Time Distribution",
                chart_file_type_share: "File Type Share",
                title_file_type_list: "File Type Details",
                title_impact_assessment: "Initiative Impact Assessment",
                header_metric: "Metric",
                header_before: "Before Initiative",
                header_after: "After Initiative",
                header_change: "Change (Δ%)",
                header_status: "Status",
                metric_throughput: "Throughput (Merged PRs/Week)",
                metric_lead_time_p50: "Median Lead Time",
                metric_lead_time_p90: "90th Percentile Lead Time",
                metric_stability: "Process Stability (Lead Time StdDev)",
                metric_rework_rate: "Rework Rate (%)",
                metric_response_time: "Avg Response Time",
                metric_review_depth: "Review Depth (Comments/PR)",
                metric_iterations: "Avg Review Iterations",
                status_improved: "Improved",
                status_declined: "Declined",
                status_stable: "Stable",
                desc_throughput: "Measures delivery volume. Formula: [Merged PRs] / [Weeks in period]. Higher means the team is completing more tasks.",
                desc_p90: "Worst-case delivery speed. Formula: The threshold under which 90% of PRs are merged. Lowering this means fewer PRs are 'stuck'.",
                desc_stability: "Measures predictability. Formula: Standard Deviation of Lead Time. Lower means delivery is consistent regardless of author or task.",
                desc_rework: "Measures quality of alignment. Formula: [PRs with 'Changes Requested' OR Iterations > 1] / [Total PRs]. This captures rework even if teams use regular comments for feedback."
            },
            ja: {
                title: "Git生産性レポート",
                language: "言語:",
                metric: "指標:",
                metric_total: "変更行数 (合計)",
                metric_added: "追加行数",
                metric_deleted: "削除行数",
                metric_commits: "コミット数",
                metric_churn: "コードチャーン (手戻り)",
                chart_type: "グラフ種類:",
                chart_line: "折れ線",
                chart_bar: "積み上げ棒",
                start: "開始日:",
                end: "終了日:",
                trend: "7日移動平均",
                sum_total: "合計",
                sum_merge: "マージコミット",
                sum_churn: "チャーン率",
                sum_active: "活動日数",
                sum_avg: "1日平均",
                chart_timeline: "タイムライン",
                chart_share: "ユーザー別シェア",
                chart_dow: "曜日別アクティビティ",
                chart_heatmap: "時間帯ヒートマップ",
                chart_size: "コミットサイズ分布",
                chart_duration: "推定稼働時間",
                chart_health: "チーム健全性トレンド",
                tooltip_timeline: "活動の推移を表示します。スパイク（リリース前）やギャップ（停滞）を確認できます。活動が一定であることが理想的です。削除行のスパイクはコードの整理やリファクタリングを示唆します。",
                tooltip_share: "貢献度の分布です。「バス係数」（特定の開発者への依存度）を特定します。極端に偏っている場合は、その人が不在の際のリスクが高いことを示します。",
                tooltip_dow: "チームの週ごとのリズムです。多くのチームは火〜木にピークを迎えます。週末の活動が多い場合は、デスマーチや不健全な働き方、リリース前のプレッシャーを示唆します。",
                tooltip_heatmap: "コアタイムを特定します。深夜など通常の時間外にクラスターがある場合は、残業やバーンアウトのリスクを示唆します。疎らなヒートマップは非同期作業が多く協力時間が不足している可能性があります。",
                tooltip_size: "コミットサイズの内訳です。XS: 10行未満, S: 10-50行, M: 50-200行, L: 200-500行, XL: 500行以上。「XS」「S」が理想的（アトミックなコミット）です。「XL」が多すぎる場合は、レビューが困難でバグが混入しやすい巨大な変更を示唆します。",
                tooltip_duration: "その日の最初と最後のコミット間の時間です。注：実際の労働時間ではありませんが活動の幅を示します。8時間超が続く場合はバーンアウトのリスクに注意が必要です。",
                tooltip_health: "赤: 手戻り率（Volatility）。高い＝不安定/リファクタリング中。算出式: 2 * min(追加, 削除) / 総変更行数。紫: 平均活動幅。両方が上昇傾向にある場合は、技術負債やデスマーチの兆候である可能性が高いです。",
                tooltip_ownership: "どのファイルに誰が貢献しているかを示します。変更頻度が高い上位15ファイル（ホットスポット）を表示しています。これらはアーキテクチャ上のボトルネック、『神クラス』、またはテスト強化が必要な不安定なモジュールである可能性があります。1人だけが触っているファイルは『バス係数』のリスクがあります。バランスの良いオーナーシップはチームの回復力と知識共有を高めます。",
                tooltip_leadtime: "マージされたブランチの寿命（最初のコミット〜マージ）。短いリードタイムは迅速なデリバリーを、長いリードタイムはマージの複雑化とリスク増大を示します。",
                tooltip_ctxswitch: "1日に触れたディレクトリ数。高い値は頻繁なコンテキストスイッチが発生していることを示し、集中力とディープワークの生産性を低下させます。",
                tooltip_forecast: "過去4週間のベロシティに基づき将来の出力を予測します。点線は予測トレンドです。『目標コミット数』を変更すると、達成予測日が表示されます。",
                label_commit_count: "コミット数",
                label_mod_count: "変更回数",
                label_days_count: "日数",
                label_churn_rate: "チャーン率 (%)",
                label_avg_duration: "平均稼働時間 (時間)",
                diff_new: "新規",
                diff_prev: "前回比",
                insights_title: "インサイト",
                insight_burnout_title: "🔥 バーンアウトリスク",
                insight_burnout_desc: "平均活動スパンが{value}時間です。",
                insight_unstable_title: "📉 コード不安定",
                insight_unstable_desc: "チャーン率が{value}%と高い水準です。",
                insight_busfactor_title: "🚌 バス係数リスク",
                insight_busfactor_desc: "{name}がコミットの{value}%を占めています。",
                insight_largecommit_title: "📦 巨大コミット傾向",
                insight_largecommit_desc: "コミットの{value}%がXLサイズです。",
                insight_hotspot_title: "📁 ホットスポット集中",
                insight_hotspot_desc: "上位3ファイルが{value}%を占めています。",
                insight_weekend_title: "📅 週末労働",
                insight_weekend_desc: "コミットの{value}%が週末に行われています。",
                insight_stable_title: "✅ 安定したペース",
                insight_stable_desc: "日数の{value}%で活動があります。",
                insight_smallcommit_title: "✅ 良好なコミット習慣",
                insight_smallcommit_desc: "コミットの{value}%がXS/Sサイズです。",
                insight_latenight_title: "🌙 深夜作業",
                insight_latenight_desc: "コミットの{value}%が22時〜5時の間です。",
                chart_ownership: "コードオーナーシップ",
                tooltip_ownership: "ファイルへの貢献度を示します。",
                label_commits: "コミット",
                insight_isolated_title: "📋 孤立ファイル",
                insight_isolated_desc: "{value}個のファイルが1人のみによって変更されています。",
                chart_leadtime: "ブランチリードタイム",
                tooltip_leadtime: "マージされたブランチの寿命（ブランチ独自の最初のコミットからマージまで）。main/develop等のベースブランチからの同期目的のマージは除外されます。短いリードタイムは迅速なデリバリーを、長いリードタイムはマージの複雑化とリスク増大を示します。",
                label_leadtime_avg: "平均",
                label_leadtime_median: "中央値",
                label_leadtime_p90: "90%点 (最悪)",
                label_days: "日",
                label_branch: "ブランチ",
                label_leadtime_days: "リードタイム (日)",
                chart_ctxswitch: "コンテキストスイッチ",
                tooltip_ctxswitch: "1日に触れたディレクトリ数。",
                label_avg_dirs: "平均ディレクトリ / 日",
                insight_ctxswitch_title: "🔀 頻繁なコンテキストスイッチ",
                insight_ctxswitch_desc: "1日平均{value}ディレクトリです。",
                insight_longlived_title: "🔄 長命ブランチ",
                insight_longlived_desc: "{value}個のブランチが7日以上存続しています。",
                header_active_days: "稼働日数",
                header_total_changes: "合計変更",
                header_reviews: "レビュー割当回数",
                header_comments: "指摘コメント数",
                header_review_lead_time: "レビューリードタイム",
                header_top_dirs: "得意ディレクトリ",
                header_avg_lead_time: "ブランチリードタイム",
                btn_select_all: "すべて選択",
                title_user_selection: "ユーザー別フィルター",
                title_predictive_analysis: "予測分析（ベータ版）",
                label_current_velocity: "現在のベロシティ",
                label_projected_throughput: "今後60日間の予測作業量",
                label_remaining_work: "残りの作業量",
                label_est_completion: "予測完了日",
                forecast_chart_title: "ベロシティ予測",
                insight_predicted_goal_title: "🎯 目標予測",
                insight_predicted_goal_desc: "残り{remaining}コミットは{date}に完了する見込みです。",
                sum_rework_rate: "修正依頼率",
                sum_review_depth: "レビュー密度",
                sum_response_time: "平均反応時間",
                sum_iterations: "平均イテレーション",
                title_isolated_files: "孤立ファイル (属人化リスク)",
                tooltip_isolated: "リポジトリ内で特定の1人しか変更していないファイルです。ナレッジが共有されていない潜在的なリスクを示します。",
                header_file_path: "ファイルパス",
                header_sole_contributor: "唯一の担当者",
                header_mod_count: "変更回数",
                title_commit_details: "コミット詳細",
                label_commits_by: "コミット履歴:",
                header_date: "日付",
                header_message: "メッセージ",
                header_hash: "ハッシュ",
                header_files: "ファイル",
                chart_reciprocity: "レビュー相互関係マトリクス",
                chart_scatter: "PRサイズ vs リードタイム",
                chart_res_dist: "反応時間の分布",
                chart_lead_dist: "リードタイムの分布",
                chart_file_type_share: "ファイル種別シェア",
                title_file_type_list: "ファイル種別詳細",
                title_impact_assessment: "施策インパクト評価",
                header_metric: "指標",
                header_before: "施策前",
                header_after: "施策後",
                header_change: "変化率 (Δ%)",
                header_status: "状態",
                metric_throughput: "スループット (マージ数/週)",
                metric_lead_time_p50: "リードタイム (中央値)",
                metric_lead_time_p90: "リードタイム (90パーセンタイル)",
                metric_stability: "プロセスの安定性 (標準偏差)",
                metric_rework_rate: "修正依頼率 (%)",
                metric_response_time: "平均反応時間",
                metric_review_depth: "レビュー密度 (コメント数/PR)",
                metric_iterations: "平均イテレーション",
                status_improved: "改善",
                status_declined: "低下",
                status_stable: "安定",
                desc_throughput: "チームのデリバリー量を測定。算出式: [期間内のマージPR総数] / [期間の週数]。数値が高いほど、より多くの成果物を完成させていることを示します。",
                desc_p90: "ワーストケースのデリバリー速度。PR全体の90%が含まれる範囲の日数を示します。この数値が改善（低下）しているほど、『放置されるPR』や『異常に難航するタスク』が減っていることを意味します。",
                desc_stability: "開発サイクルの予測可能性を測定。算出式: リードタイムの標準偏差。数値が低いほど、タスクの難易度や担当者に左右されず、安定してデリバリーされていることを示します。",
                desc_rework: "実装前の合意形成の質を測定。算出式: [修正依頼が発生、またはレビュー往復が2回以上あったPR数] / [PR総数]。GitHub公式の『Request Changes』を使わない、コメントベースの修正指示も『実質的な手戻り』として捕捉します。",
                desc_rework_prs: "修正依頼、または2回以上のレビューサイクルを要したPRの割合",
                desc_avg_comments: "1PRあたりの平均コメント数（議論の活発さ・レビューの丁寧さ）",
                desc_first_reaction: "人間による最初のレビュー依頼から、最初の反応があるまでの平均経過時間",
                desc_review_cycles: "1PRあたりのレビュー往復回数。算出式: PRごとにレビューがあった延べ日数（同日内は1回）を合計し平均化。",
                tooltip_rework_rate: "実質修正依頼率。算出式: [Changes Requested、またはレビュー往復が2回以上のPR] / [PR総数]。公式機能を使わないコメントベースの修正指示も手戻りとして捕捉する、より正確な指標です。",
                tooltip_review_depth: "レビュー密度。算出式: [コメント総数] / [PR総数]。議論の質を測定します。極端に低い場合はレビューが形骸化しているリスクがあります。",
                tooltip_response_time: "平均反応時間。算出式: [最初の人間によるレビューまたは承認] - [最初の人間によるレビュー依頼時刻]。開発者の『待ち時間』を測定します。",
                tooltip_iterations: "平均イテレーション。算出式: 1つのPRがマージされるまでに発生した『レビュー→修正』の往復回数（同日の活動は1回と集計）。設計の複雑さやコミュニケーション効率を示します。",
                tooltip_user_commits: "期間中に行われた総コミット数（マージを含む）。",
                tooltip_user_added: "期間中に追加された総行数。",
                tooltip_user_deleted: "期間中に削除された総行数。",
                tooltip_user_churn: "手戻り率。高いほど同じ箇所の修正やリワークが多いことを示します。",
                tooltip_user_reviews: "レビュワーとして割り当てられた、または実際にレビューを投稿したPRの数。",
                tooltip_user_comments: "レビューで指摘（スレッド開始）をした回数。返信は含みません。",
                tooltip_user_review_lead: "最初の指摘からマージされるまでの平均日数。",
                tooltip_user_branch_lead: "ブランチで自身の最初のコミットからマージされるまでの平均日数。",
                tooltip_user_active_days: "期間中に1回以上コミットがあった日数の合計。",
                header_ext: "拡張子",
                header_added: "追加",
                header_deleted: "削除",
                header_churn_rate: "手戻り率",
                label_throughput: "生産量",
                label_p90: "最悪リードタイム",
                label_stability: "プロセスの安定性",
                label_rework_rate_label: "修正依頼率"
            }
        };

        let currentLang = 'en';

        function t(key) {
            return translations[currentLang][key] || key;
        }

        function updateLanguage(lang) {
            currentLang = lang;
            document.getElementById('langSelect').value = lang;
            applyTranslations();
            updateDashboard();
        }

        function applyTranslations() {
            const lang = currentLang;
            document.querySelectorAll('[data-i18n]').forEach(el => {
                const key = el.getAttribute('data-i18n');
                if (translations[lang][key]) el.textContent = translations[lang][key];
            });
            document.querySelectorAll('[data-tooltip]').forEach(el => {
                 const key = el.getAttribute('data-i18n-tooltip');
                 if (key && translations[lang][key]) el.setAttribute('data-tooltip', translations[lang][key]);
            });
        }

        const dashboardData = {{ data | json_encode() | safe }};
        const aliases = {{ aliases | json_encode() | safe }};
        const filePaths = dashboardData.file_paths;
        
        function normalizeAuthor(name) {
            if (aliases && aliases[name]) return aliases[name];
            
            // Handle GitHub noreply emails if name happens to be an email
            if (name && name.endsWith('@users.noreply.github.com')) {
                const localPart = name.split('@')[0];
                const plusPos = localPart.indexOf('+');
                if (plusPos !== -1) return localPart.substring(plusPos + 1);
            }
            return name;
        }

        function isBot(user) {
            return user && user.toLowerCase().endsWith('[bot]');
        }
        
        const data = dashboardData.daily_stats.map(d => {
            const dateObj = new Date(d.date);
            return {
                ...d,
                dateObj: dateObj,
                dateStr: d.date,
                dayOfWeek: dateObj.getDay(),
                total_changes: d.added + d.deleted,
                commit_count: d.commits
            };
        });

        const ctx = document.getElementById('productivityChart').getContext('2d');
        const pieCtx = document.getElementById('shareChart').getContext('2d');
        const fileTypeCtx = document.getElementById('fileTypeChart').getContext('2d');
        const dowCtx = document.getElementById('dayOfWeekChart').getContext('2d');
        const heatmapCtx = document.getElementById('heatmapChart').getContext('2d');
        const sizeCtx = document.getElementById('sizeDistChart').getContext('2d');
        const durCtx = document.getElementById('workDurationChart').getContext('2d');
        const healthCtx = document.getElementById('healthTrendChart').getContext('2d');
        const ownerCtx = document.getElementById('ownershipChart').getContext('2d');
        const leadCtx = document.getElementById('leadTimeChart').getContext('2d');
        const reviewActivityCtx = document.getElementById('reviewActivityChart').getContext('2d');
        const reciprocityCtx = document.getElementById('reciprocityChart').getContext('2d');
        const scatterCtx = document.getElementById('scatterChart').getContext('2d');
        const resDistCtx = document.getElementById('resDistChart').getContext('2d');
        const leadDistCtx = document.getElementById('leadDistChart').getContext('2d');
        const ctxSwitchCtx = document.getElementById('ctxSwitchChart').getContext('2d');
        const forecastCtx = document.getElementById('forecastChart').getContext('2d');

        let mainChart, pieChart, fileTypeChart, dowChart, heatmapChart, sizeChart, durChart, healthChart, ownerChart, leadChart, reviewActivityChart, reciprocityChart, scatterChart, resDistChart, leadDistChart, ctxChart, forecastChart;

        const allUsers = [...new Set(data.map(d => d.author))].sort();
        let selectedUsers = new Set(allUsers);
        const allDates = [...new Set(data.map(d => d.dateStr))].sort();

        let currentSort = { column: 'commits', direction: 'desc' };

        function toggleSort(column) {
            if (currentSort.column === column) {
                currentSort.direction = currentSort.direction === 'desc' ? 'asc' : 'desc';
            } else {
                currentSort.column = column;
                currentSort.direction = 'desc'; // Default to desc for new column
            }
            updateUserList();
        }

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
            const result = [];
            for (let i = 0; i < values.length; i++) {
                const start = Math.max(0, i - windowSize + 1);
                const subset = values.slice(start, i + 1);
                const sum = subset.reduce((a, b) => a + b, 0);
                result.push(sum / subset.length);
            }
            return result;
        }

        function selectAllUsers(selected) {
            selectedUsers = selected ? new Set(allUsers) : new Set();
            document.querySelectorAll('.user-checkbox').forEach(cb => {
                cb.checked = selected;
            });
            updateDashboard();
        }

        function renderUserCheckboxes() {
            const container = document.getElementById('userCheckboxes');
            container.innerHTML = '';
            allUsers.forEach(user => {
                const label = document.createElement('label');
                label.className = 'user-checkbox-item';
                label.innerHTML = `
                    <input type="checkbox" class="user-checkbox" value="${user}" ${selectedUsers.has(user) ? 'checked' : ''} onchange="toggleUser('${user}', this.checked)">
                    <div class="color-dot" style="background-color: ${stringToColor(user)}"></div>
                    ${user}
                `;
                container.appendChild(label);
            });
        }

        function toggleUser(user, checked) {
            if (checked) selectedUsers.add(user);
            else selectedUsers.delete(user);
            updateDashboard();
        }

        function syncStateToUrl() {
            const params = new URLSearchParams();
            params.set('lang', currentLang);
            params.set('metric', document.getElementById('metricSelect').value);
            params.set('chart', document.getElementById('chartTypeSelect').value);
            params.set('start', document.getElementById('startDate').value);
            params.set('end', document.getElementById('endDate').value);
            params.set('trend', document.getElementById('showTrend').checked);
            params.set('users', Array.from(selectedUsers).join(','));
            const newUrl = window.location.pathname + '?' + params.toString();
            window.history.replaceState({}, '', newUrl);
        }

        function loadStateFromUrl() {
            const params = new URLSearchParams(window.location.search);
            if (params.has('lang')) {
                currentLang = params.get('lang');
                document.getElementById('langSelect').value = currentLang;
            }
            if (params.has('metric')) document.getElementById('metricSelect').value = params.get('metric');
            if (params.has('chart')) document.getElementById('chartTypeSelect').value = params.get('chart');
            if (params.has('start')) document.getElementById('startDate').value = params.get('start');
            if (params.has('end')) document.getElementById('endDate').value = params.get('end');
            if (params.has('trend')) document.getElementById('showTrend').checked = params.get('trend') === 'true';
            if (params.has('users')) {
                const users = params.get('users').split(',').filter(u => u.length > 0);
                selectedUsers = new Set(users);
            }
            applyTranslations();
        }

        function updateDashboard() {
            const metric = document.getElementById('metricSelect').value;
            const chartType = document.getElementById('chartTypeSelect').value;
            const startDate = document.getElementById('startDate').value;
            const endDate = document.getElementById('endDate').value;
            const showTrend = document.getElementById('showTrend').checked;
            syncStateToUrl();
            const filteredData = data.filter(d => 
                d.dateStr >= startDate && d.dateStr <= endDate && selectedUsers.has(d.author)
            );
            updateSummary(filteredData, metric, startDate, endDate);
            updateTimelineChart(filteredData, metric, chartType, showTrend, startDate, endDate);
            updatePieChart(filteredData, metric);
            updateDayOfWeekChart(filteredData, metric);
            updateHeatmapChart(filteredData, metric);
            updateSizeDistChart(filteredData);
            updateWorkDurationChart(filteredData);
            updateHealthTrendChart(filteredData, startDate, endDate);
            updateOwnershipChart(filteredData, startDate, endDate);
            updateLeadTimeChart(filteredData, startDate, endDate);
            updateReviewActivityChart(startDate, endDate);
            updateGitHubAdvancedMetrics(startDate, endDate);
            updateImpactAssessment();
            updateContextSwitchChart(filteredData, startDate, endDate);
            updateFileTypeChart(filteredData);
            updateIsolatedFiles(filteredData);
            generateInsights(filteredData, startDate, endDate);
            updateUserList(filteredData);
            updatePredictiveDashboard(filteredData);
        }

        function updateIsolatedFiles(filteredData) {
            const tbody = document.getElementById('isolatedFilesTableBody');
            if (!tbody) return;
            tbody.innerHTML = '';

            const authorsPerFile = {}; // file_idx -> Set of authors
            const modsPerFile = {};    // file_idx -> total count

            // Use all available file stats to detect absolute isolation
            dashboardData.file_stats.forEach(fs => {
                if (!authorsPerFile[fs.file_idx]) {
                    authorsPerFile[fs.file_idx] = new Set();
                    modsPerFile[fs.file_idx] = 0;
                }
                authorsPerFile[fs.file_idx].add(normalizeAuthor(fs.author));
                modsPerFile[fs.file_idx] += fs.count;
            });

            const isolated = [];
            Object.keys(authorsPerFile).forEach(fidx => {
                const authors = authorsPerFile[fidx];
                if (authors.size === 1) {
                    isolated.push({
                        path: filePaths[fidx] || `Unknown (${fidx})`,
                        author: [...authors][0],
                        count: modsPerFile[fidx]
                    });
                }
            });

            // Sort by impact (modification count)
            isolated.sort((a, b) => b.count - a.count);

            const topIsolated = isolated.slice(0, 20); // Show top 20 risky files

            if (topIsolated.length === 0) {
                tbody.innerHTML = '<tr><td colspan="3" style="text-align: center; color: #7f8c8d; padding: 20px;">No isolated files detected.</td></tr>';
                return;
            }

            topIsolated.forEach(f => {
                const tr = document.createElement('tr');
                tr.innerHTML = `
                    <td style="font-family: monospace; font-size: 11px;">${f.path}</td>
                    <td><div class="user-info"><div class="user-avatar" style="width:16px; height:16px; background-color: ${stringToColor(f.author)}"></div>${f.author}</div></td>
                    <td>${f.count}</td>
                `;
                tbody.appendChild(tr);
            });
        }

        function updateFileTypeChart(filteredData) {
            const extMap = {};
            filteredData.forEach(c => {
                const total = c.added + c.deleted;
                const churn = (c.added + c.deleted) - Math.abs(c.added - c.deleted);
                
                const commitExts = new Set();
                if (c.files && Array.isArray(c.files)) {
                    c.files.forEach(fidx => {
                        const path = filePaths[fidx];
                        if (path && typeof path === 'string') {
                            const parts = path.split('/');
                            const filename = parts.pop() || "";
                            if (filename.includes('.')) {
                                const extParts = filename.split('.');
                                const ext = extParts.pop().toLowerCase();
                                // Handle cases like ".gitignore" where first part is empty
                                if (ext && ext.length < 8) {
                                    commitExts.add(ext);
                                } else {
                                    commitExts.add('no-ext');
                                }
                            } else {
                                commitExts.add('no-ext');
                            }
                        }
                    });
                }

                if (commitExts.size === 0) return;
                
                commitExts.forEach(ext => {
                    if (!extMap[ext]) extMap[ext] = { ext, added: 0, deleted: 0, churn: 0, commits: 0 };
                    extMap[ext].added += Math.round(c.added / commitExts.size);
                    extMap[ext].deleted += Math.round(c.deleted / commitExts.size);
                    extMap[ext].churn += Math.round(churn / commitExts.size);
                    extMap[ext].commits += 1;
                });
            });

            // Sort by total changes (descending)
            // Sort by total changes (descending), then by commit count
            const sortedExts = Object.values(extMap).sort((a, b) => {
                const totalA = a.added + a.deleted;
                const totalB = b.added + b.deleted;
                if (totalA !== totalB) return totalB - totalA;
                return b.commits - a.commits;
            });
            const topExts = sortedExts.slice(0, 15);

            if (fileTypeChart) fileTypeChart.destroy();
            
            if (topExts.length === 0) {
                const tbody = document.getElementById('fileTypeTableBody');
                if (tbody) tbody.innerHTML = '<tr><td colspan="4" style="text-align: center; color: #7f8c8d; padding: 20px;">No file data found for the selected filters.</td></tr>';
                return;
            }
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

            const tbody = document.getElementById('fileTypeTableBody');
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
                
                // Iterations
                const distinctReviewCycles = pr.reviews ? new Set(pr.reviews.filter(r => r.state !== 'COMMENTED' && !isBot(r.user)).map(r => r.submitted_at.split('T')[0])).size : 0;
                const iterations = Math.max(1, distinctReviewCycles);
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

            // 3. Render Scatter Chart (Size vs Lead Time)
            if (scatterChart) scatterChart.destroy();
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
        }

        function updateSummary(currentData, metric, startDate, endDate) {
            const currentTotal = currentData.reduce((acc, d) => acc + d[metric], 0);
            const activeDays = new Set(currentData.map(d => d.dateStr)).size;
            const avgPerDay = activeDays > 0 ? (currentTotal / activeDays).toFixed(1) : 0;
            const totalChanges = currentData.reduce((acc, d) => acc + d.total_changes, 0);
            const totalChurn = currentData.reduce((acc, d) => acc + d.churn, 0);
            const totalMerges = currentData.reduce((acc, d) => acc + d.merges, 0);
            const churnRate = totalChanges > 0 ? ((totalChurn / totalChanges) * 100).toFixed(1) : '0.0';
            
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
            
            // Safety limit to 2 years of daily data
            let safety = 0;
            while (curr <= end && safety < 730) {
                const dStr = curr.toISOString().split('T')[0];
                displayDates.push(dStr);
                dateMap.set(dStr, {});
                curr.setDate(curr.getDate() + 1);
                safety++;
            }
            filteredData.forEach(d => {
                if (!dateMap.has(d.dateStr)) return;
                const daily = dateMap.get(d.dateStr);
                daily[d.author] = (daily[d.author] || 0) + (d[metric] || 0);
            });
            const datasets = allUsers.map(user => ({
                label: user,
                data: displayDates.map(date => dateMap.get(date)[user] || 0),
                fill: chartType === 'bar',
                borderColor: stringToColor(user),
                backgroundColor: stringToColor(user),
                tension: 0.1,
                borderWidth: chartType === 'bar' ? 0 : 2
            }));
            if (mainChart) mainChart.destroy();
            mainChart = new Chart(ctx, {
                type: chartType,
                data: { labels: displayDates, datasets },
                options: { responsive: true, maintainAspectRatio: false, scales: { x: { stacked: chartType === 'bar' }, y: { stacked: chartType === 'bar', beginAtZero: true } } }
            });
        }

        function updatePieChart(filteredData, metric) {
            const userTotals = {};
            filteredData.forEach(d => { userTotals[d.author] = (userTotals[d.author] || 0) + (d[metric] || 0); });
            const labels = Object.keys(userTotals);
            const values = Object.values(userTotals);
            if (pieChart) pieChart.destroy();
            pieChart = new Chart(pieCtx, {
                type: 'doughnut',
                data: { labels, datasets: [{ data: values, backgroundColor: labels.map(u => stringToColor(u)) }] },
                options: { responsive: true, maintainAspectRatio: false, plugins: { legend: { position: 'right' } } }
            });
        }

        function updateDayOfWeekChart(filteredData, metric) {
            const days = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];
            const dayTotals = new Array(7).fill(0);
            filteredData.forEach(d => { dayTotals[d.dayOfWeek] += (d[metric] || 0); });
            if (dowChart) dowChart.destroy();
            dowChart = new Chart(dowCtx, {
                type: 'bar',
                data: { labels: days, datasets: [{ label: t('label_activity'), data: dayTotals, backgroundColor: '#3498db99' }] },
                options: { responsive: true, maintainAspectRatio: false, plugins: { legend: { display: false } } }
            });
        }

        function updateHeatmapChart(filteredData, metric) {
            const heatmapData = [];
            const counts = {};
            filteredData.forEach(d => { if (d.hours) d.hours.forEach(h => { const key = `${d.dayOfWeek}-${h}`; counts[key] = (counts[key] || 0) + 1; }); });
            for (let d = 0; d < 7; d++) for (let h = 0; h < 24; h++) heatmapData.push({ x: h, y: d, v: counts[`${d}-${h}`] || 0 });
            
            if (heatmapChart) heatmapChart.destroy();
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

        function updateSizeDistChart(filteredData) {
            const counts = [0, 0, 0, 0, 0];
            filteredData.forEach(d => { if (d.commit_sizes) d.commit_sizes.forEach(s => { if (s < 10) counts[0]++; else if (s < 50) counts[1]++; else if (s < 200) counts[2]++; else if (s < 500) counts[3]++; else counts[4]++; }); });
            if (sizeChart) sizeChart.destroy();
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
        }

        function updateWorkDurationChart(filteredData) {
            const userDatasets = {};
            filteredData.forEach(d => { if (d.hours && d.hours.length > 1) { if (!userDatasets[d.author]) userDatasets[d.author] = [0, 0, 0, 0]; const dur = Math.max(...d.hours) - Math.min(...d.hours); if (dur < 1) userDatasets[d.author][0]++; else if (dur < 4) userDatasets[d.author][1]++; else if (dur < 8) userDatasets[d.author][2]++; else userDatasets[d.author][3]++; } });
            const datasets = Object.entries(userDatasets).map(([user, bins]) => ({ label: user, data: bins, backgroundColor: stringToColor(user) }));
            if (durChart) durChart.destroy();
            durChart = new Chart(durCtx, {
                type: 'bar',
                data: { labels: ['<1h', '1-4h', '4-8h', '8h+'], datasets },
                options: { responsive: true, maintainAspectRatio: false, scales: { x: { stacked: true }, y: { stacked: true } } }
            });
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
        }

        function updateOwnershipChart(filteredData, startDate, endDate) {
            const filteredAuthors = new Set(filteredData.map(d => d.author));
            const fileUserMap = {};
            dashboardData.file_stats.forEach(fs => { if (filteredAuthors.has(fs.author)) { const fName = filePaths[fs.file_idx] || fs.file_idx; if (!fileUserMap[fName]) fileUserMap[fName] = {}; fileUserMap[fName][fs.author] = (fileUserMap[fName][fs.author] || 0) + fs.count; } });
            const fileTotals = Object.entries(fileUserMap).map(([f, users]) => ({ file: f, total: Object.values(users).reduce((a, b) => a + b, 0), users })).sort((a, b) => b.total - a.total).slice(0, 15).reverse();
            const ownerUsers = [...new Set(fileTotals.flatMap(f => Object.keys(f.users)))];
            const datasets = ownerUsers.map(user => ({ label: user, data: fileTotals.map(f => f.users[user] || 0), backgroundColor: stringToColor(user) }));
            if (ownerChart) ownerChart.destroy();
            ownerChart = new Chart(ownerCtx, {
                type: 'bar',
                data: { labels: fileTotals.map(f => f.file), datasets },
                options: { indexAxis: 'y', responsive: true, maintainAspectRatio: false, scales: { x: { stacked: true }, y: { stacked: true } } }
            });
        }

        function updateLeadTimeChart(filteredData, startDate, endDate) {
            const allFilteredMerges = dashboardData.merge_events.filter(me => me.date >= startDate && me.date <= endDate);
            const branches = allFilteredMerges.slice(0, 15).reverse();
            
            if (leadChart) leadChart.destroy();
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

            // Update stats summary
            const statsContainer = document.getElementById('leadTimeStats');
            if (allFilteredMerges.length > 0) {
                const days = allFilteredMerges.map(m => m.days).sort((a, b) => a - b);
                const avg = days.reduce((a, b) => a + b, 0) / days.length;
                const median = days[Math.floor(days.length * 0.5)];
                const p90 = days[Math.floor(days.length * 0.9)];
                
                statsContainer.innerHTML = `
                    <span><strong>${t('label_leadtime_avg')}:</strong> ${avg.toFixed(1)}${t('label_days')}</span>
                    <span><strong>${t('label_leadtime_median')}:</strong> ${median}${t('label_days')}</span>
                    <span><strong>${t('label_leadtime_p90')}:</strong> ${p90}${t('label_days')}</span>
                `;
            } else {
                statsContainer.innerHTML = `<span>No merge data for this period</span>`;
            }
        }

        function updateContextSwitchChart(filteredData, startDate, endDate) {
            const relevantCounts = dashboardData.daily_dir_counts.filter(dc => dc.date >= startDate && dc.date <= endDate).sort((a, b) => a.date.localeCompare(b.date));
            if (ctxChart) ctxChart.destroy();
            ctxChart = new Chart(ctxSwitchCtx, {
                type: 'line',
                data: { labels: relevantCounts.map(dc => dc.date), datasets: [{ data: relevantCounts.map(dc => dc.count), borderColor: '#9b59b6', fill: true }] },
                options: { responsive: true, maintainAspectRatio: false }
            });
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
                ['currentVelocityValue', 'velocityTrendValue', 'projectedThroughputValue', 'estCompletionValue'].forEach(id => document.getElementById(id).textContent = '-');
                document.getElementById('estCompletionRange').textContent = '';
                if (forecastChart) forecastChart.destroy();
                return;
            }

            const history = weeklyStats.map(w => w.commits);
            const sum = history.reduce((a, b) => a + b, 0);
            const mean = sum / history.length;
            const variance = history.reduce((a, b) => a + Math.pow(b - mean, 2), 0) / history.length;
            const stdev = Math.sqrt(variance);
            
            // Confidence: Lower CoV (Coefficient of Variation) = Higher confidence
            const cov = stdev / (mean || 1);
            const confidence = cov < 0.2 ? 'High' : cov < 0.5 ? 'Medium' : 'Low';
            const confidenceColor = confidence === 'High' ? '#27ae60' : confidence === 'Medium' ? '#f39c12' : '#e74c3c';

            const last4Weeks = weeklyStats.slice(-4).reverse();
            const currentVelocity = last4Weeks.reduce((acc, w) => acc + w.commits, 0) / last4Weeks.length;
            
            // Trend
            const recentAvg = (last4Weeks[0].commits + (last4Weeks[1] ? last4Weeks[1].commits : last4Weeks[0].commits)) / 2;
            const prevAvg = last4Weeks.length >= 4 
                ? (last4Weeks[2].commits + last4Weeks[3].commits) / 2
                : (last4Weeks[2] ? last4Weeks[2].commits : recentAvg);
            
            const trend = prevAvg > 0 ? ((recentAvg - prevAvg) / prevAvg) * 100 : 0;
            const trendEl = document.getElementById('velocityTrendValue');
            trendEl.textContent = `${trend >= 0 ? '▲' : '▼'} ${Math.abs(trend).toFixed(1)}%`;
            trendEl.className = `forecast-trend ${trend >= 0 ? 'up' : 'down'}`;

            document.getElementById('currentVelocityValue').innerHTML = `${currentVelocity.toFixed(1)} ${t('label_commits')}/week <span style="font-size: 12px; color: ${confidenceColor}; font-weight: normal;">(Confidence: ${confidence})</span>`;
            
            const projected60 = Math.round(currentVelocity * (60/7));
            if (document.getElementById('projectedThroughputValue')) document.getElementById('projectedThroughputValue').textContent = `${projected60.toLocaleString()} ${t('label_commits')}`;

            // Goal Estimation (Using REMAINING work directly)
            const remaining = parseInt(document.getElementById('remainingWorkInput').value) || 0;
            
            if (remaining > 0 && currentVelocity > 0) {
                function calcDate(v) {
                    const weeks = remaining / Math.max(v, 0.1);
                    const d = new Date();
                    d.setDate(d.getDate() + (weeks * 7));
                    return d.toLocaleDateString(currentLang === 'ja' ? 'ja-JP' : 'en-US', { month: 'short', day: 'numeric' });
                }

                const likelyDate = calcDate(currentVelocity);
                const optimisticDate = calcDate(currentVelocity + stdev);
                const pessimisticDate = calcDate(Math.max(currentVelocity - stdev, 0.5));

                if (document.getElementById('estCompletionValue')) document.getElementById('estCompletionValue').textContent = likelyDate;
                if (document.getElementById('estCompletionRange')) document.getElementById('estCompletionRange').innerHTML = 
                    `🚀 Optimistic: ${optimisticDate}<br>🐢 Pessimistic: ${pessimisticDate}`;
                
                // Add predictive insight
                const insightsContainer = document.getElementById('insightsGrid');
                if (insightsContainer) {
                    const card = document.createElement('div');
                    card.className = 'insight-card positive';
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
                if (document.getElementById('estCompletionValue')) document.getElementById('estCompletionValue').textContent = remaining <= 0 ? 'Work Complete!' : '-';
                if (document.getElementById('estCompletionRange')) document.getElementById('estCompletionRange').textContent = '';
            }

            updateForecastChart(weeklyStats, currentVelocity, stdev);
        }

        function updateDistributionChart(resTimes, leadTimes) {
            const distBox = document.getElementById('distBox');
            if (resTimes.length === 0 && leadTimes.length === 0) {
                if (distBox) distBox.style.display = 'none';
                return;
            }
            if (distBox) distBox.style.display = 'grid'; // Note: grid instead of block

            function createHistogram(data, bucketSize) {
                const bins = {};
                data.forEach(v => {
                    const bin = Math.floor(v / bucketSize) * bucketSize;
                    bins[bin] = (bins[bin] || 0) + 1;
                });
                return bins;
            }

            const resBins = createHistogram(resTimes, 4); 
            const resLabels = Object.keys(resBins).sort((a,b) => a-b);

            if (resDistChart) resDistChart.destroy();
            resDistChart = new Chart(resDistCtx, {
                type: 'bar',
                data: {
                    labels: resLabels,
                    datasets: [{
                        label: t('chart_res_dist'),
                        data: resLabels.map(l => resBins[l]),
                        backgroundColor: 'rgba(230, 126, 34, 0.6)'
                    }]
                },
                options: {
                    responsive: true, maintainAspectRatio: false,
                    scales: {
                        x: { title: { display: true, text: 'Hours' } },
                        y: { beginAtZero: true, title: { display: true, text: t('label_mod_count') } }
                    }
                }
            });

            const leadBins = createHistogram(leadTimes, 1);
            const leadLabels = Object.keys(leadBins).sort((a,b) => a-b);

            if (leadDistChart) leadDistChart.destroy();
            leadDistChart = new Chart(leadDistCtx, {
                type: 'bar',
                data: {
                    labels: leadLabels,
                    datasets: [{
                        label: t('chart_lead_dist'),
                        data: leadLabels.map(l => leadBins[l]),
                        backgroundColor: 'rgba(52, 152, 219, 0.6)'
                    }]
                },
                options: {
                    responsive: true, maintainAspectRatio: false,
                    scales: {
                        x: { title: { display: true, text: 'Days' } },
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
                dashboardData.events.forEach((e, idx) => {
                    const opt = document.createElement('option');
                    opt.value = idx;
                    opt.textContent = `${e.name} (${e.date})`;
                    eventSelect.appendChild(opt);
                });
                eventSelect.value = dashboardData.events.length - 1; // Default to last
                eventIdx = dashboardData.events.length - 1;
            }

            // If eventIdx is not provided (called from updateDashboard), use current selector value
            if (eventIdx === undefined && eventSelect) {
                eventIdx = parseInt(eventSelect.value);
            }

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
                    const cycles = pr.reviews ? new Set(pr.reviews.filter(r => r.state !== 'COMMENTED').map(r => r.submitted_at.split('T')[0])).size : 0;
                    return Math.max(1, cycles);
                });
                const iters = getDetailedStats(iterationValues);

                const stdDev = Math.sqrt(leadTimeValues.reduce((a, b) => a + Math.pow(b - lt.avg, 2), 0) / (leadTimeValues.length || 1));

                return { 
                    throughput, 
                    median: lt.median, 
                    p90: lt.p90 || 0, // Fallback
                    max: lt.max,
                    avg: lt.avg,
                    stdDev, 
                    reworkRate, 
                    responseTime: res.avg, 
                    responseMedian: res.median,
                    reviewDepth: depth.avg, 
                    iterations: iters.avg 
                };
            }

            const now = allDates.length > 0 ? new Date(allDates[allDates.length - 1]) : new Date();
            const beforeWeeks = 90 / 7;
            const diffDays = Math.max(1, (now - eventDate) / (1000 * 60 * 60 * 24));
            const afterWeeks = diffDays / 7;

            const before = getStats(beforePRs, beforeWeeks, true);
            const after = getStats(afterPRs, afterWeeks, false);

            const metrics = [
                { id: 'metric_throughput', b: before.throughput, a: after.throughput, unit: ' PRs/week', lowerIsBetter: false },
                { id: 'metric_lead_time_p50', b: before.median, a: after.median, unit: ' days', lowerIsBetter: true },
                { id: 'metric_lead_time_p90', b: before.p90, a: after.p90, unit: ' days', lowerIsBetter: true },
                { id: 'metric_stability', b: before.stdDev, a: after.stdDev, unit: '', lowerIsBetter: true },
                { id: 'metric_rework_rate', b: before.reworkRate, a: after.reworkRate, unit: '%', lowerIsBetter: true },
                { id: 'metric_response_time', b: before.responseTime, a: after.responseTime, unit: 'h', lowerIsBetter: true },
                { id: 'metric_review_depth', b: before.reviewDepth, a: after.reviewDepth, unit: '', lowerIsBetter: false },
                { id: 'metric_iterations', b: before.iterations, a: after.iterations, unit: '', lowerIsBetter: true }
            ];

            metrics.forEach(m => {
                const diff = m.b > 0 ? ((m.a - m.b) / m.b) * 100 : 0;
                const isImproved = m.lowerIsBetter ? m.a < m.b : m.a > m.b;
                const status = Math.abs(diff) < 5 ? t('status_stable') : (isImproved ? t('status_improved') : t('status_declined'));
                const statusColor = Math.abs(diff) < 5 ? '#7f8c8d' : (isImproved ? '#27ae60' : '#e74c3c');

                const tr = document.createElement('tr');
                tr.innerHTML = `
                    <td><strong>${t(m.id)}</strong></td>
                    <td>${m.b.toFixed(2)}${m.unit}</td>
                    <td>${m.a.toFixed(2)}${m.unit}</td>
                    <td style="color: ${statusColor}; font-weight: bold;">${diff > 0 ? '+' : ''}${diff.toFixed(1)}%</td>
                    <td><span class="badge" style="background: ${statusColor}22; color: ${statusColor}">${status}</span></td>
                `;
                impactTableBody.appendChild(tr);
            });

            document.getElementById('impactDescription').innerHTML = `Assessment of initiative: <strong>${event.name}</strong> (Started ${event.date})`;
            
            // Update Timeline with vertical lines
            updateTimelineWithEvents();
        }

        function updateTimelineWithEvents() {
            if (!mainChart || !dashboardData.events) return;
            
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

            mainChart.options.plugins.annotation = { annotations };
            mainChart.update();
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

        loadStateFromUrl();
        renderUserCheckboxes();
        updateDashboard();
    </script>
</body>
</html>
"#;
