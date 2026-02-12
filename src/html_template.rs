pub const HTML_TEMPLATE: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Git Productivity Report</title>
    <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
    <script src="https://cdn.jsdelivr.net/npm/chartjs-chart-matrix@2.0.1"></script>
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
                    <div class="goal-setter">
                        <span data-i18n="label_target_goal">Target Goal</span>
                        <input type="number" id="targetGoalInput" value="1000" onchange="updateDashboard()">
                    </div>
                </div>
            </div>
            <div class="chart-box full-width" style="box-shadow: none; padding: 0;">
                <div class="chart-title" data-i18n="forecast_chart_title">Velocity Forecasting</div>
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
                    <span data-i18n="chart_hotspots">File Hotspots (Top 20 Modified)</span>
                    <span class="info-icon" data-i18n-tooltip="tooltip_hotspots" data-tooltip="Most frequently changed files.">i</span>
                </div>
                <canvas id="hotspotsChart"></canvas>
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
                    <span class="info-icon" data-i18n-tooltip="tooltip_health" data-tooltip="Churn Rate & Avg Duration.">i</span>
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
            <div class="chart-box full-width">
                <div class="chart-title">
                    <span data-i18n="chart_leadtime">Branch Lead Time</span>
                    <span class="info-icon" data-i18n-tooltip="tooltip_leadtime" data-tooltip="Time span of merged branches.">i</span>
                </div>
                <canvas id="leadTimeChart"></canvas>
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
                            <th data-i18n="header_author">Author</th>
                            <th data-i18n="header_commits">Commits</th>
                            <th data-i18n="header_added">Added</th>
                            <th data-i18n="header_deleted">Deleted</th>
                            <th data-i18n="header_total_changes">Total Changes</th>
                            <th data-i18n="header_reviews">Reviews (Given)</th>
                            <th data-i18n="header_avg_lead_time">Avg Lead Time</th>
                            <th data-i18n="header_active_days">Active Days</th>
                            <th data-i18n="header_top_dirs">Top Dirs</th>
                        </tr>
                    </thead>
                    <tbody id="userTableBody">
                        <!-- Populated by JS -->
                    </tbody>
                </table>
            </div>
        </div>

        <div class="card" id="githubSection" style="max-width: none; margin-bottom: 30px; display: none;">
            <h2 style="font-size: 18px; color: #2c3e50; margin-bottom: 15px; text-align: left;">🐙 GitHub Review Activity (Top 100 PRs)</h2>
            <div class="table-container">
                <table class="user-table">
                    <thead>
                        <tr>
                            <th>PR</th>
                            <th>Author</th>
                            <th>Status</th>
                            <th>Reviews</th>
                        </tr>
                    </thead>
                    <tbody id="githubTableBody">
                        <!-- Populated by JS -->
                    </tbody>
                </table>
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
                chart_hotspots: "File Hotspots (Top 20 Modified)",
                chart_duration: "Est. Daily Work Duration",
                chart_health: "Team Health Trends",
                tooltip_timeline: "Shows activity trends over time.",
                tooltip_share: "Distribution of contributions.",
                tooltip_dow: "Weekly rhythm.",
                tooltip_heatmap: "Identifies core working hours.",
                tooltip_size: "Breakdown of commit sizes.",
                tooltip_hotspots: "Most frequently changed files.",
                tooltip_duration: "Time between first and last commit.",
                tooltip_health: "Churn Rate & Avg Duration.",
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
                tooltip_leadtime: "Time span of merged branches.",
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
                header_reviews: "Reviews (Given)",
                header_top_dirs: "Top Dirs",
                header_avg_lead_time: "Avg Lead Time",
                btn_select_all: "Select All",
                title_user_selection: "Filter by Users",
                title_predictive_analysis: "Predictive Analysis (BETA)",
                label_current_velocity: "Current Velocity",
                label_projected_throughput: "Projected 60-Day Throughput",
                label_target_goal: "Target Goal (Commits)",
                label_est_completion: "Estimated Completion Date",
                forecast_chart_title: "Velocity Forecasting",
                insight_predicted_goal_title: "Target Forecast",
                insight_predicted_goal_desc: "You will reach your goal of {target} commits by {date}."
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
                chart_hotspots: "変更頻度ランキング",
                chart_duration: "推定稼働時間",
                chart_health: "チーム健全性トレンド",
                tooltip_timeline: "活動の推移を表示します。",
                tooltip_share: "貢献度の分布です。",
                tooltip_dow: "週ごとのリズムです。",
                tooltip_heatmap: "コアタイムを特定します。",
                tooltip_size: "コミットサイズの内訳です。",
                tooltip_hotspots: "最も頻繁に変更されるファイルです。",
                tooltip_duration: "活動の幅を示します。",
                tooltip_health: "チャーン率 & 稼働時間。",
                label_activity: "アクティビティ",
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
                tooltip_leadtime: "マージされたブランチの寿命。",
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
                header_reviews: "レビュー回数",
                header_top_dirs: "得意ディレクトリ",
                header_avg_lead_time: "平均リードタイム",
                btn_select_all: "すべて選択",
                title_user_selection: "ユーザー別フィルター",
                title_predictive_analysis: "予測分析（ベータ版）",
                label_current_velocity: "現在のベロシティ",
                label_projected_throughput: "今後60日間の予測作業量",
                label_target_goal: "目標コミット数",
                label_est_completion: "予測完了日",
                forecast_chart_title: "ベロシティ予測",
                insight_predicted_goal_title: "🎯 目標予測",
                insight_predicted_goal_desc: "目標の{target}コミットには{date}に到達する見込みです。"
            }
        };

        let currentLang = 'en';

        function t(key) {
            return translations[currentLang][key] || key;
        }

        function updateLanguage(lang) {
            currentLang = lang;
            document.getElementById('langSelect').value = lang;
            document.querySelectorAll('[data-i18n]').forEach(el => {
                const key = el.getAttribute('data-i18n');
                if (translations[lang][key]) el.textContent = translations[lang][key];
            });
            document.querySelectorAll('[data-tooltip]').forEach(el => {
                 const key = el.getAttribute('data-i18n-tooltip');
                 if (key && translations[lang][key]) el.setAttribute('data-tooltip', translations[lang][key]);
            });
            updateDashboard();
        }

        const dashboardData = {{ data | json_encode() | safe }};
        const filePaths = dashboardData.file_paths;
        
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
        const dowCtx = document.getElementById('dayOfWeekChart').getContext('2d');
        const heatmapCtx = document.getElementById('heatmapChart').getContext('2d');
        const sizeCtx = document.getElementById('sizeDistChart').getContext('2d');
        const hotCtx = document.getElementById('hotspotsChart').getContext('2d');
        const durCtx = document.getElementById('workDurationChart').getContext('2d');
        const healthCtx = document.getElementById('healthTrendChart').getContext('2d');
        const ownerCtx = document.getElementById('ownershipChart').getContext('2d');
        const leadCtx = document.getElementById('leadTimeChart').getContext('2d');
        const ctxSwitchCtx = document.getElementById('ctxSwitchChart').getContext('2d');
        const forecastCtx = document.getElementById('forecastChart').getContext('2d');

        let mainChart, pieChart, dowChart, heatmapChart, sizeChart, hotChart, durChart, healthChart, ownerChart, leadChart, ctxChart, forecastChart;

        const allUsers = [...new Set(data.map(d => d.author))].sort();
        let selectedUsers = new Set(allUsers);
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
            updateHotspotsChart(filteredData, startDate, endDate);
            updateWorkDurationChart(filteredData);
            updateHealthTrendChart(filteredData, startDate, endDate);
            updateOwnershipChart(filteredData, startDate, endDate);
            updateLeadTimeChart(filteredData, startDate, endDate);
            updateContextSwitchChart(filteredData, startDate, endDate);
            generateInsights(filteredData, startDate, endDate);
            updateUserList(filteredData);
            updatePredictiveDashboard(filteredData);
        }

        function updateSummary(currentData, metric, startDate, endDate) {
            const currentTotal = currentData.reduce((acc, d) => acc + d[metric], 0);
            const activeDays = new Set(currentData.map(d => d.dateStr)).size;
            const avgPerDay = activeDays > 0 ? (currentTotal / activeDays).toFixed(1) : 0;
            const totalChanges = currentData.reduce((acc, d) => acc + d.total_changes, 0);
            const totalChurn = currentData.reduce((acc, d) => acc + d.churn, 0);
            const totalMerges = currentData.reduce((acc, d) => acc + d.merges, 0);
            const churnRate = totalChanges > 0 ? ((totalChurn / totalChanges) * 100).toFixed(1) : 0;
            document.getElementById('summaryValue').textContent = currentTotal.toLocaleString();
            document.getElementById('mergeCommitsValue').textContent = totalMerges.toLocaleString();
            document.getElementById('churnRateValue').textContent = `${churnRate}%`;
            document.getElementById('activeDaysValue').textContent = activeDays;
            document.getElementById('avgPerDayValue').textContent = Number(avgPerDay).toLocaleString();
        }

        function updateTimelineChart(filteredData, metric, chartType, showTrend, startDate, endDate) {
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
                data: { datasets: [{
                    data: heatmapData,
                    backgroundColor: ctx => `rgba(52, 152, 219, ${Math.min(ctx.dataset.data[ctx.dataIndex].v / 10, 1)})`,
                    width: ({ chart }) => chart.chartArea ? (chart.chartArea.width / 24) - 1 : 0,
                    height: ({ chart }) => chart.chartArea ? (chart.chartArea.height / 7) - 1 : 0
                }]},
                options: { responsive: true, maintainAspectRatio: false, scales: { x: { type: 'linear', min: 0, max: 23 }, y: { type: 'linear', min: 0, max: 6, reverse: true } } }
            });
        }

        function updateSizeDistChart(filteredData) {
            const counts = [0, 0, 0, 0, 0];
            filteredData.forEach(d => { if (d.commit_sizes) d.commit_sizes.forEach(s => { if (s < 10) counts[0]++; else if (s < 50) counts[1]++; else if (s < 200) counts[2]++; else if (s < 500) counts[3]++; else counts[4]++; }); });
            if (sizeChart) sizeChart.destroy();
            sizeChart = new Chart(sizeCtx, {
                type: 'bar',
                data: { labels: ['XS', 'S', 'M', 'L', 'XL'], datasets: [{ data: counts, backgroundColor: '#f1c40f99' }] },
                options: { responsive: true, maintainAspectRatio: false }
            });
        }

        function updateHotspotsChart(filteredData, startDate, endDate) {
            const filteredAuthors = new Set(filteredData.map(d => d.author));
            const fileCounts = {};
            dashboardData.file_stats.forEach(fs => { if (filteredAuthors.has(fs.author)) { const fName = filePaths[fs.file_idx] || fs.file_idx; fileCounts[fName] = (fileCounts[fName] || 0) + fs.count; } });
            const topFiles = Object.entries(fileCounts).sort((a, b) => b[1] - a[1]).slice(0, 15).reverse();
            if (hotChart) hotChart.destroy();
            hotChart = new Chart(hotCtx, {
                type: 'bar',
                data: { labels: topFiles.map(f => f[0]), datasets: [{ data: topFiles.map(f => f[1]), backgroundColor: '#e67e2299' }] },
                options: { indexAxis: 'y', responsive: true, maintainAspectRatio: false }
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
            while (curr <= end) { displayDates.push(curr.toISOString().split('T')[0]); curr.setDate(curr.getDate() + 1); }
            const churnRates = displayDates.map(date => { const dayData = filteredData.filter(d => d.dateStr === date); const changes = dayData.reduce((a, d) => a + d.total_changes, 0); const churn = dayData.reduce((a, d) => a + d.churn, 0); return changes > 0 ? (churn / changes) * 100 : 0; });
            if (healthChart) healthChart.destroy();
            healthChart = new Chart(healthCtx, {
                type: 'line',
                data: { labels: displayDates, datasets: [{ label: 'Churn Rate', data: calculateMovingAverage(churnRates, 7), borderColor: '#e74c3c', fill: true }] },
                options: { responsive: true, maintainAspectRatio: false }
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
            const branches = dashboardData.merge_events.filter(me => me.date >= startDate && me.date <= endDate).slice(0, 15).reverse();
            if (leadChart) leadChart.destroy();
            leadChart = new Chart(leadCtx, {
                type: 'bar',
                data: { labels: branches.map(b => b.branch), datasets: [{ data: branches.map(b => b.days), backgroundColor: '#27ae6099' }] },
                options: { indexAxis: 'y', responsive: true, maintainAspectRatio: false }
            });
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
                if (forecastChart) forecastChart.destroy();
                return;
            }

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

            document.getElementById('currentVelocityValue').textContent = `${currentVelocity.toFixed(1)} ${t('label_commits')}/week`;
            
            const projected60 = Math.round(currentVelocity * (60/7));
            document.getElementById('projectedThroughputValue').textContent = `${projected60.toLocaleString()} ${t('label_commits')}`;

            // Goal Estimation
            const targetGoal = parseInt(document.getElementById('targetGoalInput').value) || 1000;
            const currentTotalCommits = filteredData.reduce((acc, d) => acc + d.commit_count, 0);
            const remaining = targetGoal - currentTotalCommits;
            
            if (remaining > 0 && currentVelocity > 0) {
                const weeksToGoal = remaining / currentVelocity;
                const estDate = new Date();
                estDate.setDate(estDate.getDate() + (weeksToGoal * 7));
                document.getElementById('estCompletionValue').textContent = estDate.toLocaleDateString(currentLang === 'ja' ? 'ja-JP' : 'en-US', { year: 'numeric', month: 'short', day: 'numeric' });
                
                // Add predictive insight
                const insightsContainer = document.getElementById('insightsGrid');
                const card = document.createElement('div');
                card.className = 'insight-card positive';
                card.innerHTML = `
                    <div class="insight-icon">🎯</div>
                    <div class="insight-body">
                        <div class="insight-title">${t('insight_predicted_goal_title')}</div>
                        <div class="insight-desc">${t('insight_predicted_goal_desc').replace('{target}', targetGoal).replace('{date}', estDate.toLocaleDateString())}</div>
                    </div>
                `;
                insightsContainer.prepend(card);
            } else {
                document.getElementById('estCompletionValue').textContent = remaining <= 0 ? 'Goal Reached!' : '-';
            }

            updateForecastChart(weeklyStats, currentVelocity);
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

        function updateForecastChart(weeklyStats, currentVelocity) {
            if (forecastChart) forecastChart.destroy();

            const labels = weeklyStats.map(w => w.week_start);
            const dataPoint = weeklyStats.map(w => w.commits);
            
            // Projections (next 4 weeks)
            const projectionLabels = [];
            const projectionData = new Array(labels.length - 1).fill(null);
            projectionData.push(dataPoint[dataPoint.length - 1]); // connector

            const lastDate = new Date(labels[labels.length - 1]);
            for (let i = 1; i <= 4; i++) {
                const nextDate = new Date(lastDate);
                nextDate.setDate(lastDate.getDate() + (i * 7));
                const nextDateStr = nextDate.toISOString().split('T')[0];
                labels.push(nextDateStr);
                projectionData.push(currentVelocity);
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

        function updateUserList(filteredData) {
            const userStats = {};
            filteredData.forEach(d => {
                if (!userStats[d.author]) userStats[d.author] = { commits: 0, added: 0, deleted: 0, activeDays: new Set(), dirs: {}, leadTimes: [], reviewsGiven: 0 };
                userStats[d.author].commits += d.commit_count;
                userStats[d.author].added += d.added;
                userStats[d.author].deleted += d.deleted;
                userStats[d.author].activeDays.add(d.dateStr);
            });
            const currentUsers = new Set(Object.keys(userStats));
            dashboardData.file_stats.forEach(fs => { if (currentUsers.has(fs.author)) { const path = filePaths[fs.file_idx]; if (path) { const dir = path.includes('/') ? path.split('/')[0] : '(root)'; userStats[fs.author].dirs[dir] = (userStats[fs.author].dirs[dir] || 0) + fs.count; } } });
            dashboardData.merge_events.forEach(me => { if (currentUsers.has(me.author)) userStats[me.author].leadTimes.push(me.days); });
            if (dashboardData.github_prs && dashboardData.github_prs.length > 0) {
                document.getElementById('githubSection').style.display = 'block';
                const ghTbody = document.getElementById('githubTableBody');
                ghTbody.innerHTML = '';
                dashboardData.github_prs.forEach(pr => {
                    pr.reviews.forEach(rev => { if (userStats[rev.user]) userStats[rev.user].reviewsGiven++; });
                    const tr = document.createElement('tr');
                    tr.innerHTML = `<td><a href="${pr.html_url}" target="_blank">#${pr.number} ${pr.title}</a></td><td>${pr.author}</td><td>${pr.reviews.length > 0 ? 'Reviewed' : 'Pending'}</td><td>${pr.reviews.map(r => `<span class="badge" style="background: ${r.state === 'APPROVED' ? '#ecfaf2' : '#fdf2f2'}; color: ${r.state === 'APPROVED' ? '#27ae60' : '#e74c3c'}">${r.user}</span>`).join(' ')}</td>`;
                    ghTbody.appendChild(tr);
                });
            }
            const tbody = document.getElementById('userTableBody');
            tbody.innerHTML = '';
            Object.entries(userStats).sort((a, b) => (b[1].added + b[1].deleted) - (a[1].added + a[1].deleted)).forEach(([user, s]) => {
                const tr = document.createElement('tr');
                tr.innerHTML = `<td><div class="user-info"><div class="user-avatar" style="background-color: ${stringToColor(user)}"></div><strong>${user}</strong></div></td><td>${s.commits}</td><td>${s.added}</td><td>${s.deleted}</td><td>${s.added + s.deleted}</td><td>${s.reviewsGiven}</td><td>${s.leadTimes.length > 0 ? (s.leadTimes.reduce((a, b) => a + b, 0) / s.leadTimes.length).toFixed(1) : '-'}</td><td>${s.activeDays.size}</td><td>${Object.entries(s.dirs).sort((a, b) => b[1] - a[1]).slice(0, 3).map(d => d[0]).join(', ')}</td>`;
                tbody.appendChild(tr);
            });
        }

        loadStateFromUrl();
        renderUserCheckboxes();
        updateDashboard();
    </script>
</body>
</html>
"#;
