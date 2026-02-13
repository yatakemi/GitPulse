pub const HTML_TEMPLATE: &str = concat!(
    r#"
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
        .chart-title { position: absolute; top: 15px; left: 20px; font-size: 16px; font-weight: 600; color: #34495e; z-index: 10; display: flex; align-items: center; gap: 8px; width: calc(100% - 40px); }
        .chart-controls { margin-left: auto; display: flex; align-items: center; gap: 10px; }
        .chart-controls select { padding: 4px 8px; font-size: 12px; height: auto; }
        
        /* Tooltip Styles */
        .info-icon {
            display: inline-flex; justify-content: center; align-items: center;
            width: 18px; height: 18px; border-radius: 50%; background: #bdc3c7; color: white;
            font-size: 12px; font-weight: bold; cursor: help; position: relative;
        }
        .info-icon:hover { background: #3498db; }
        .info-icon:hover::after {
            content: attr(data-tooltip);
            position: absolute; top: 100%; left: 50%; transform: translateX(-50%);
            background: #34495e; color: white; padding: 8px 12px; border-radius: 6px;
            font-size: 12px; font-weight: 400; white-space: pre-wrap; width: 250px; text-align: left;
            box-shadow: 0 4px 6px rgba(0,0,0,0.1); z-index: 100; margin-top: 8px;
            line-height: 1.4;
        }
        .info-icon:hover::before {
            content: ''; position: absolute; top: 100%; left: 50%; margin-left: -5px; margin-top: 3px;
            border-width: 5px; border-style: solid; border-color: transparent transparent #34495e transparent;
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
        
        .user-table th, .user-table td {
            padding: 12px;
            text-align: left;
            border-bottom: 1px solid #eee;
        }
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
                    <strong data-i18n="label_throughput">Throughput</strong>: <span data-i18n="desc_throughput">Measures delivery volume. Formula: [Merged PRs] / [Weeks in period]. Higher means the team is completing more tasks.</span>
                </div>
                <div>
                    <strong data-i18n="label_p90">P90 Lead Time</strong>: <span data-i18n="desc_p90">Worst-case delivery speed. Formula: The threshold under which 90% of PRs are merged. Lowering this means fewer PRs are 'stuck'.</span>
                </div>
                <div>
                    <strong data-i18n="label_stability">Stability</strong>: <span data-i18n="desc_stability">Measures predictability. Formula: Standard Deviation of Lead Time. Lower means delivery is consistent regardless of author or task.</span>
                </div>
                <div>
                    <strong data-i18n="label_rework_rate_label">Rework Rate</strong>: <span data-i18n="desc_rework">Measures quality of alignment. Formula: [PRs with 'Changes Requested' OR Iterations > 1] / [Total PRs]. This captures rework even if teams use regular comments for feedback.</span>
                </div>
                <div>
                    <strong data-i18n="metric_steps">Avg Lines Added / Week</strong>: <span data-i18n="desc_steps">Measures code volume. Formula: [Total Lines Added] / [Weeks in period]. Helps track implementation effort trends.</span>
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
                    <span class="info-icon" data-i18n-tooltip="tooltip_timeline" data-tooltip="Shows activity trends over time. Look for spikes (sprints/releases) or gaps (blockers/downtime). Ideally, activity should be consistent. Spike in deletions might indicate cleanup/refactoring.">i</span>
                    <div class="chart-controls">
                        <select id="chartTypeSelect" onchange="updateDashboard()">
                            <option value="line" data-i18n="chart_line">Line Chart</option>
                            <option value="bar" data-i18n="chart_bar">Stacked Bar</option>
                        </select>
                    </div>
                </div>
                <canvas id="productivityChart"></canvas>
            </div>
            <div class="chart-box full-width">
                <div class="chart-title">
                    <span data-i18n="chart_lead_time_trend">Lead Time Trend (Time Series)</span>
                    <span class="info-icon" data-i18n-tooltip="tooltip_lead_time_trend" data-tooltip="Shows the daily average branch lead time over time. Lower is better. Spikes indicate periods where branches stayed open longer.">i</span>
                </div>
                <canvas id="leadTimeTrendChart"></canvas>
            </div>
            <div class="chart-box full-width">
                <div class="chart-title">
                    <span data-i18n="chart_file_type_trend">File Type Activity Trend</span>
                    <span class="info-icon" data-i18n-tooltip="tooltip_file_type_trend" data-tooltip="Shows the time-series change of lines added per file type (especially 'test'). Use this to track if testing activity increases after certain initiatives.">i</span>
                </div>
                <canvas id="fileTypeTrendChart"></canvas>
            </div>
            <div class="chart-box full-width">
                <div class="chart-title">
                    <span data-i18n="chart_velocity_size_correlation">Commit Velocity vs. Size Trend</span>
                    <span class="info-icon" data-i18n-tooltip="tooltip_velocity_size" data-tooltip="Correlates commit frequency with commit size. \n\nInsights:\n1. Style Shift: If commits increase while size decreases, the team is moving towards 'Atomic Commits'.\n2. Real Productivity: If both increase, the actual delivery volume is growing.\n3. Risk: Large size with low frequency often indicates high-risk PRs that are hard to review.">i</span>
                </div>
                <canvas id="velocitySizeChart"></canvas>
            </div>
            <div class="chart-box">
                <div class="chart-title">
                    <span data-i18n="chart_share">User Share</span>
                    <span class="info-icon" data-i18n-tooltip="tooltip_share" data-tooltip="Distribution of contributions. Helps identify 'Bus Factor' (reliance on single dev). A highly skewed chart suggests high risk if the top contributor is unavailable.">i</span>
                </div>
                <canvas id="shareChart"></canvas>
            </div>
            <div class="chart-box">
                <div class="chart-title">
                    <span data-i18n="chart_dow">Day of Week Activity</span>
                    <span class="info-icon" data-i18n-tooltip="tooltip_dow" data-tooltip="Weekly rhythm. Most teams peak Tue-Thu. High weekend activity might indicate crunch time, unhealthy work habits, or upcoming release pressure.">i</span>
                </div>
                <canvas id="dayOfWeekChart"></canvas>
            </div>
            <div class="chart-box full-width">
                <div class="chart-title">
                    <span data-i18n="chart_heatmap">Activity Heatmap (Hour vs Day)</span>
                    <span class="info-icon" data-i18n-tooltip="tooltip_heatmap" data-tooltip="Identifies core working hours. Look for clusters outside normal hours (e.g. late nights), which suggests overtime or burnout risk. Inconsistent heatmaps might indicate lack of overlapping hours for collaboration.">i</span>
                </div>
                <canvas id="heatmapChart"></canvas>
            </div>
             <div class="chart-box full-width">
                <div class="chart-title">
                    <span data-i18n="chart_size">Commit Size Distribution</span>
                    <span class="info-icon" data-i18n-tooltip="tooltip_size" data-tooltip="Breakdown of commit sizes. XS: 10行未満, S: 10-50行, M: 50-200行, L: 200-500行, XL: 500行以上。「XS」「S」が理想的（アトミックなコミット）です。「XL」が多すぎる場合は、レビューが困難でバグが混入しやすい巨大な変更を示唆します。">i</span>
                </div>
                <canvas id="sizeDistChart"></canvas>
            </div>
            <div class="chart-box full-width">
                <div class="chart-title">
                    <span data-i18n="chart_duration">Est. Daily Work Duration</span>
                    <span class="info-icon" data-i18n-tooltip="tooltip_duration" data-tooltip="その日の最初と最後のコミット間の時間です。注：実際の労働時間ではありませんが活動の幅を示します。8時間超が続く場合はバーンアウトのリスクに注意が必要です。">i</span>
                </div>
                <canvas id="workDurationChart"></canvas>
            </div>
             <div class="chart-box full-width">
                <div class="chart-title">
                    <span data-i18n="chart_health">Team Health Trends</span>
                    <span class="info-icon" data-i18n-tooltip="tooltip_health" data-tooltip="赤: 手戻り率（Volatility）。高い＝不安定/リファクタリング中。算出式: 2 * min(追加, 削除) / 総変更行数。紫: 平均活動幅。両方が上昇傾向にある場合は、技術負債やデスマーチの兆候である可能性が高いです。">i</span>
                </div>
                <canvas id="healthTrendChart"></canvas>
            </div>
            <div class="chart-box full-width">
                <div class="chart-title">
                    <span data-i18n="chart_ownership">Code Ownership (Top 15 Files)</span>
                    <span class="info-icon" data-i18n-tooltip="tooltip_ownership" data-tooltip="どのファイルに誰が貢献しているかを示します。変更頻度が高い上位15ファイル（ホットスポット）を表示しています。これらはアーキテクチャ上のボトルネック、『神クラス』、またはテスト強化が必要な不安定なモジュールである可能性があります。1人だけが触っているファイルは『バス係数』のリスクがあります。バランスの良いオーナーシップはチームの回復力と知識共有を高めます。">i</span>
                </div>
                <canvas id="ownershipChart"></canvas>
            </div>
            <div class="chart-box full-width" style="height: auto; min-height: 200px;">
                <div class="chart-title">
                    <span data-i18n="title_isolated_files">Isolated Files (Bus Factor Risk)</span>
                    <span class="info-icon" data-i18n-tooltip="tooltip_isolated" data-tooltip="リポジトリ内で特定の1人しか変更していないファイルです。ナレッジが共有されていない潜在的なリスクを示します。">i</span>
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
                    <span class="info-icon" data-i18n-tooltip="tooltip_leadtime" data-tooltip="マージされたブランチの寿命（ブランチ独自の最初のコミットからマージまで）。main/develop等のベースブランチからの同期目的のマージは除外されます。短いリードタイムは迅速なデリバリーを、長いリードタイムはマージの複雑化とリスク増大を示します。">i</span>
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
            <div id="distBox" style="display: none; margin-bottom: 25px;">
                <div class="charts-grid" style="margin-bottom: 0;">
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
            </div>
            <div class="chart-box full-width">
                <div class="chart-title">
                    <span data-i18n="chart_ctxswitch">Context Switching (Daily Directory Diversity)</span>
                    <span class="info-icon" data-i18n-tooltip="tooltip_ctxswitch" data-tooltip="1日に触れたディレクトリ数。高い値は頻繁なコンテキストスイッチが発生していることを示し、集中力とディープワークの生産性を低下させます。">i</span>
                </div>
                <canvas id="ctxSwitchChart"></canvas>
            </div>
            <div class="chart-box full-width">
                <div class="chart-title">
                    <span data-i18n="chart_fragmentation">Time Fragmentation (Inter-commit Intervals)</span>
                    <span class="info-icon" data-i18n-tooltip="tooltip_fragmentation" data-tooltip="連続するコミット間の経過時間を表示します。短い間隔はマルチタスクや頻繁な割り込みを示唆し、長い間隔は深い集中状態（ディープワーク）が確保できていることを示します。">i</span>
                </div>
                <canvas id="fragmentationChart"></canvas>
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
                                <span class="info-icon" data-i18n-tooltip="tooltip_user_commits" data-tooltip="期間中に行われた総コミット数（マージを含む）。">i</span>
                            </th>
                            <th onclick="toggleSort('added')" id="th-added">
                                <span data-i18n="header_added">Added</span>
                                <span class="info-icon" data-i18n-tooltip="tooltip_user_added" data-tooltip="期間中に追加された総行数。">i</span>
                            </th>
                            <th onclick="toggleSort('deleted')" id="th-deleted">
                                <span data-i18n="header_deleted">Deleted</span>
                                <span class="info-icon" data-i18n-tooltip="tooltip_user_deleted" data-tooltip="期間中に削除された総行数。">i</span>
                            </th>
                            <th onclick="toggleSort('total_changes')" id="th-total_changes" data-i18n="header_total_changes">Total Changes</th>
                            <th onclick="toggleSort('churn_rate')" id="th-churn_rate">
                                <span data-i18n="sum_churn">Churn Rate</span>
                                <span class="info-icon" data-i18n-tooltip="tooltip_user_churn" data-tooltip="手戻り率。高いほど同じ箇所の修正やリワークが多いことを示します。">i</span>
                            </th>
                            <th onclick="toggleSort('reviewsAssigned')" id="th-reviewsAssigned">
                                <span data-i18n="header_reviews">Reviews (Assigned)</span>
                                <span class="info-icon" data-i18n-tooltip="tooltip_user_reviews" data-tooltip="レビュワーとして割り当てられた、または実際にレビューを投稿したPRの数。">i</span>
                            </th>
                            <th onclick="toggleSort('commentsGiven')" id="th-commentsGiven">
                                <span data-i18n="header_comments">Review Comments</span>
                                <span class="info-icon" data-i18n-tooltip="tooltip_user_comments" data-tooltip="レビューで指摘（スレッド開始）をした回数。返信は含みません。">i</span>
                            </th>
                            <th onclick="toggleSort('avgReviewLeadTime')" id="th-avgReviewLeadTime">
                                <span data-i18n="header_review_lead_time">Review Lead Time</span>
                                <span class="info-icon" data-i18n-tooltip="tooltip_user_review_lead" data-tooltip="最初の指摘からマージされるまでの平均日数。">i</span>
                            </th>
                            <th onclick="toggleSort('avgLeadTime')" id="th-avgLeadTime">
                                <span data-i18n="header_avg_lead_time">Branch Lead Time</span>
                                <span class="info-icon" data-i18n-tooltip="tooltip_user_branch_lead" data-tooltip="ブランチで自身の最初のコミットからマージされるまでの平均日数。">i</span>
                            </th>
                            <th onclick="toggleSort('activeDays')" id="th-activeDays">
                                <span data-i18n="header_active_days">Active Days</span>
                                <span class="info-icon" data-i18n-tooltip="tooltip_user_active_days" data-tooltip="期間中に1回以上コミットがあった日数の合計。">i</span>
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
"#,
    include_str!("report.js"),
    r#"
    </script>
</body>
</html>
"#
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html_template_not_empty() {
        assert!(!HTML_TEMPLATE.is_empty());
        assert!(HTML_TEMPLATE.contains("<!DOCTYPE html>"));
        assert!(HTML_TEMPLATE.contains("<script>"));
    }
}
