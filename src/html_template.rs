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
            
            <!-- ... existing controls ... -->

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
                        <input type="number" id="targetGoalInput" value="1000" onchange="updatePredictiveDashboard()">
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
            <!-- (Existing charts...) -->
            <!-- Note: I'm skipping the repetitive chart blocks in this diff as they are unchanged -->
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
                chart_duration: "Est. Daily Work Duration (First to Last Commit)",
                chart_health: "Team Health Trends (Churn Rate & Work Duration)",
                tooltip_timeline: "Shows activity trends over time. Look for spikes (sprints/releases) or gaps (blockers/downtime). Ideally, activity should be consistent.",
                tooltip_share: "Distribution of contributions. Helps identify 'Bus Factor' (reliance on single dev) or uneven workload distribution.",
                tooltip_dow: "Weekly rhythm. Most teams peak Tue-Thu. High weekend activity might indicate crunch time or unhealthy work habits.",
                tooltip_heatmap: "Identifies core working hours. Look for clusters outside normal hours (e.g. late nights), which suggests overtime or burnout risk.",
                tooltip_size: "Breakdown of commit sizes. 'XS'/'S' are ideal (atomic commits). Too many 'XL' suggests large, risky changes that are hard to review.",
                tooltip_hotspots: "Most frequently changed files. These are potential architectural bottlenecks, 'God Classes', or unstable modules needing refactoring.",
                tooltip_duration: "Time between first and last commit of the day. NOTE: Not actual work hours, but indicates span of activity. Long spans may suggest burnout.",
                tooltip_health: "Red: Churn Rate (Rework/Volatility). High = Unstable/Refactoring.\nPurple: Avg Duration. Rising trend = Potential Overwork.",
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
                insight_burnout_desc: "Average work span in the last 7 days is {value} hours. Long activity spans may indicate overwork.",
                insight_unstable_title: "Code Instability",
                insight_unstable_desc: "Churn Rate is {value}%. High churn suggests frequent rework or unstable code.",
                insight_busfactor_title: "Bus Factor Risk",
                insight_busfactor_desc: "{name} accounts for {value}% of commits. High reliance on a single contributor is risky.",
                insight_largecommit_title: "Large Commit Pattern",
                insight_largecommit_desc: "{value}% of commits are XL (500+ lines). Consider breaking them into smaller, reviewable units.",
                insight_hotspot_title: "Hotspot Concentration",
                insight_hotspot_desc: "Top 3 files account for {value}% of all file changes. These may need refactoring.",
                insight_weekend_title: "Weekend Work",
                insight_weekend_desc: "{value}% of commits are on weekends. This may indicate crunch time or unsustainable pace.",
                insight_stable_title: "Stable Pace",
                insight_stable_desc: "Active on {value}% of days with low churn. The team maintains a healthy, consistent rhythm.",
                insight_smallcommit_title: "Good Commit Habits",
                insight_smallcommit_desc: "{value}% of commits are XS/S size. Atomic commits make reviews easier and reduce risk.",
                insight_latenight_title: "Late Night Activity",
                insight_latenight_desc: "{value}% of commits are between 10PM-5AM. This may affect well-being and code quality.",
                chart_ownership: "Code Ownership (Top 15 Files)",
                tooltip_ownership: "Shows who contributes to which files. Files with only one contributor are a 'Bus Factor' risk. Balanced ownership improves team resilience.",
                label_commits: "commits",
                insight_isolated_title: "Isolated Files",
                insight_isolated_desc: "{value} file(s) are only touched by one person. If that person is unavailable, no one else has context.",
                chart_leadtime: "Branch Lead Time",
                tooltip_leadtime: "Time span of merged branches (from first commit to merge). Shorter lead times indicate faster delivery. Long-lived branches increase merge complexity.",
                label_days: "days",
                label_branch: "Branch",
                label_leadtime_days: "Lead Time (Days)",
                chart_ctxswitch: "Context Switching (Daily Directory Diversity)",
                tooltip_ctxswitch: "Number of distinct directories touched per day. High values indicate frequent context switching, which reduces focus and deep work. Lower is generally better.",
                label_avg_dirs: "Avg Directories / Day",
                insight_ctxswitch_title: "Frequent Context Switching",
                insight_ctxswitch_desc: "Average {value} directories touched per day. Frequent switching between areas reduces deep work and focus.",
                insight_longlived_title: "Long-lived Branches",
                insight_longlived_desc: "{value} branch(es) lived longer than 7 days. Long-lived branches increase merge complexity and risk.",
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
                insight_predicted_goal_desc: "At current velocity, you will reach your goal of {target} commits by {date}."
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
                chart_heatmap: "時間帯ヒートマップ (時 vs 曜日)",
                chart_size: "コミットサイズ分布",
                chart_hotspots: "変更頻度ランキング (Top 20)",
                chart_duration: "推定稼働時間 (最初のコミット〜最後)",
                chart_health: "チーム健全性トレンド (チャーン率 & 稼働時間)",
                tooltip_timeline: "活動の推移を表示します。スパイク（スプリント/リリース）やギャップ（ブロッカー/休暇）を確認できます。活動が一定であることが理想的です。",
                tooltip_share: "貢献度の分布です。「バス係数」（特定の開発者への依存）や作業負荷の偏りを特定するのに役立ちます。",
                tooltip_dow: "週ごとのリズムです。多くのチームは火〜木にピークを迎えます。週末の活動が多い場合は、デスマーチや不健全な働き方を示唆している可能性があります。",
                tooltip_heatmap: "コアタイムを特定します。通常の時間外（深夜など）にクラスターがある場合は、残業やバーンアウトのリスクを示唆します。",
                tooltip_size: "コミットサイズの内訳です。「XS」「S」が理想的（アトミックなコミット）です。「XL」が多すぎる場合は、レビューが困難な大きな変更やリスクを示唆します。",
                tooltip_hotspots: "最も頻繁に変更されるファイルです。これらはアーキテクチャ上のボトルネック、「神クラス」、またはリファクタリングが必要な不安定なモジュールである可能性があります。",
                tooltip_duration: "その日の最初のコミットから最後のコミットまでの時間です。注：実際の労働時間ではありませんが、活動の幅を示します。長い期間はバーンアウトを示唆する可能性があります。",
                tooltip_health: "赤: チャーン率（手戻り/変動）。高い＝不安定/リファクタリング中。\n紫: 平均稼働時間。上昇傾向＝過重労働の可能性。",
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
                insight_burnout_desc: "直近7日間の平均活動スパンが{value}時間です。長時間の活動傾向が見られます。",
                insight_unstable_title: "📉 コード不安定",
                insight_unstable_desc: "チャーン率が{value}%と高い水準です。手戻りやリファクタリングが頻発している可能性があります。",
                insight_busfactor_title: "🚌 バス係数リスク",
                insight_busfactor_desc: "{name}がコミットの{value}%を占めています。特定メンバーへの依存度が高い状態です。",
                insight_largecommit_title: "📦 巨大コミット傾向",
                insight_largecommit_desc: "コミットの{value}%がXL（500行以上）です。レビューしやすい小さな単位に分割することを推奨します。",
                insight_hotspot_title: "📁 ホットスポット集中",
                insight_hotspot_desc: "上位3ファイルがファイル変更の{value}%を占めています。リファクタリングの検討を推奨します。",
                insight_weekend_title: "📅 週末労働",
                insight_weekend_desc: "コミットの{value}%が週末に行われています。デスマーチや持続不可能なペースを示唆する可能性があります。",
                insight_stable_title: "✅ 安定したペース",
                insight_stable_desc: "日数の{value}%で活動があり、チャーン率も低い水準です。健全で安定したリズムを維持しています。",
                insight_smallcommit_title: "✅ 良好なコミット習慣",
                insight_smallcommit_desc: "コミットの{value}%がXS/Sサイズです。アトミックなコミットはレビューを容易にし、リスクを低減します。",
                insight_latenight_title: "🌙 深夜作業",
                insight_latenight_desc: "コミットの{value}%が22時〜5時の間に行われています。健康やコード品質への影響が懸念されます。",
                chart_ownership: "コードオーナーシップ (Top 15ファイル)",
                tooltip_ownership: "誰がどのファイルに貢献しているかを示します。1人だけが触っているファイルは『バス係数』リスクです。バランスの良いオーナーシップがチームの回復力を高めます。",
                label_commits: "コミット",
                insight_isolated_title: "📋 孤立ファイル",
                insight_isolated_desc: "{value}個のファイルが1人のみによって変更されています。その人が不在の場合、誰も文脈を持ちません。",
                chart_leadtime: "ブランチリードタイム",
                tooltip_leadtime: "マージされたブランチの寿命（最初のコミット〜マージ）。短いリードタイムは迅速なデリバリーを示します。長命ブランチはマージの複雑さを増します。",
                label_days: "日",
                label_branch: "ブランチ",
                label_leadtime_days: "リードタイム (日)",
                chart_ctxswitch: "コンテキストスイッチ (日別ディレクトリ多様性)",
                tooltip_ctxswitch: "1日に触れたディレクトリ数。高い値は頻繁なコンテキストスイッチを示し、深い集中を妨げます。低いほうが一般的に良好です。",
                label_avg_dirs: "平均ディレクトリ / 日",
                insight_ctxswitch_title: "🔀 頻繁なコンテキストスイッチ",
                insight_ctxswitch_desc: "1日平均{value}ディレクトリを跨いで作業しています。頻繁な切り替えは集中力と深い作業を妨げます。",
                insight_longlived_title: "🔄 長命ブランチ",
                insight_longlived_desc: "{value}個のブランチが7日以上存続しています。長命ブランチはマージの複雑さとリスクを増大させます。",
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
                insight_predicted_goal_desc: "現在のベロシティを維持した場合、目標の{target}コミットには{date}に到達する見込みです。"
            }
        };

        let currentLang = 'en';

        function t(key) {
            return translations[currentLang][key] || key;
        }

        function updateLanguage(lang) {
            currentLang = lang;
            document.getElementById('langSelect').value = lang;
            
            // Static text updates
            document.querySelectorAll('[data-i18n]').forEach(el => {
                const key = el.getAttribute('data-i18n');
                if (translations[lang][key]) {
                    el.textContent = translations[lang][key];
                }
            });

            // Tooltip updates
            document.querySelectorAll('[data-tooltip]').forEach(el => {
                 // Try to find key from parent's title? 
                 // Actually the tooltip is on specific .info-icon. 
                 // Let's rely on data-i18n-tooltip if we add it, or map manually.
                 // Better: Add data-i18n-tooltip to .info-icon
                 const key = el.getAttribute('data-i18n-tooltip');
                 if (key && translations[lang][key]) {
                     el.setAttribute('data-tooltip', translations[lang][key]);
                 }
            });

            // Re-render dashboard to update chart labels
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
        const forecastCtx = document.getElementById('forecastChart').getContext('2d');

        let mainChart, pieChart, dowChart, heatmapChart, sizeChart, hotChart, durChart, healthChart, ownerChart, leadChart, ctxChart, forecastChart;

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

        const allUsers = [...new Set(data.map(d => d.author))].sort();
        let selectedUsers = new Set(allUsers);
        const allDates = [...new Set(data.map(d => d.dateStr))].sort();

        if (allDates.length > 0) {
            document.getElementById('startDate').value = allDates[0];
            document.getElementById('endDate').value = allDates[allDates.length - 1];
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
                updateLanguageElements();
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

        function updateLanguageElements() {
            document.querySelectorAll('[data-i18n]').forEach(el => {
                const key = el.getAttribute('data-i18n');
                if (translations[currentLang][key]) el.textContent = translations[currentLang][key];
            });
            document.querySelectorAll('[data-tooltip]').forEach(el => {
                const key = el.getAttribute('data-i18n-tooltip');
                if (key && translations[currentLang][key]) el.setAttribute('data-tooltip', translations[currentLang][key]);
            });
        }

        function updateDashboard() {
            const metric = document.getElementById('metricSelect').value;
            const chartType = document.getElementById('chartTypeSelect').value;
            const startDate = document.getElementById('startDate').value;
            const endDate = document.getElementById('endDate').value;
            const showTrend = document.getElementById('showTrend').checked;

            syncStateToUrl();

            const filteredData = data.filter(d => 
                d.dateStr >= startDate && 
                d.dateStr <= endDate && 
                selectedUsers.has(d.author)
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

        // ... (existing updateSummary, updateTimelineChart, etc.) ...

        function generateInsights(filteredData, startDate, endDate) {
            const container = document.getElementById('insightsGrid');
            container.innerHTML = '';
            const insights = [];
            if (filteredData.length === 0) return;

            // Helper
            function addInsight(severity, icon, titleKey, descKey, values) {
                let desc = t(descKey);
                Object.entries(values || {}).forEach(([k, v]) => {
                    desc = desc.replace(`{${k}}`, v);
                });
                insights.push({ severity, icon, title: t(titleKey), desc });
            }

            // --- Rule 1: Burnout Risk ---
            const recentDates = [...new Set(filteredData.map(d => d.dateStr))].sort().slice(-7);
            let totalSpan = 0, spanCount = 0;
            filteredData.filter(d => recentDates.includes(d.dateStr)).forEach(d => {
                if (d.hours && d.hours.length > 1) {
                    const min = Math.min(...d.hours);
                    const max = Math.max(...d.hours);
                    totalSpan += (max - min);
                    spanCount++;
                }
            });
            const avgSpan = spanCount > 0 ? totalSpan / spanCount : 0;
            if (avgSpan > 8) {
                addInsight('warning', '🔥', 'insight_burnout_title', 'insight_burnout_desc', { value: avgSpan.toFixed(1) });
            }

            // --- Rule 2: Code Instability ---
            const totalChanges = filteredData.reduce((a, d) => a + d.total_changes, 0);
            const totalChurn = filteredData.reduce((a, d) => a + d.churn, 0);
            const churnRate = totalChanges > 0 ? (totalChurn / totalChanges) * 100 : 0;
            if (churnRate > 50) {
                addInsight('warning', '📉', 'insight_unstable_title', 'insight_unstable_desc', { value: churnRate.toFixed(1) });
            }

            // --- Rule 3: Bus Factor ---
            const userCommits = {};
            filteredData.forEach(d => { userCommits[d.author] = (userCommits[d.author] || 0) + d.commit_count; });
            const allCommitsCount = Object.values(userCommits).reduce((a, b) => a + b, 0);
            const sortedUsers = Object.entries(userCommits).sort((a, b) => b[1] - a[1]);
            if (sortedUsers.length > 0 && allCommitsCount > 0) {
                const topShare = (sortedUsers[0][1] / allCommitsCount) * 100;
                if (topShare > 70) {
                    addInsight('warning', '🚌', 'insight_busfactor_title', 'insight_busfactor_desc', { name: sortedUsers[0][0], value: topShare.toFixed(0) });
                }
            }

            // --- Rule 4: Large Commits ---
            let xlCount = 0, totalCommits = 0;
            filteredData.forEach(d => {
                xlCount += d.commit_sizes.filter(s => s > 500).length;
                totalCommits += d.commit_count;
            });
            const xlPct = totalCommits > 0 ? (xlCount / totalCommits) * 100 : 0;
            if (xlPct > 20) {
                addInsight('info', '📦', 'insight_largecommit_title', 'insight_largecommit_desc', { value: xlPct.toFixed(0) });
            }

            // --- Rule 5: Hotspot Concentration ---
            const filteredAuthors = new Set(filteredData.map(d => d.author));
            const fileCounts = {};
            dashboardData.file_stats.forEach(fs => {
                if (filteredAuthors.has(fs.author)) {
                    const fName = filePaths[fs.file_idx] || fs.file_idx;
                    fileCounts[fName] = (fileCounts[fName] || 0) + fs.count;
                }
            });
            const sortedFiles = Object.entries(fileCounts).sort((a, b) => b[1] - a[1]);
            const totalFileMods = Object.values(fileCounts).reduce((a, b) => a + b, 0);
            if (sortedFiles.length >= 3 && totalFileMods > 0) {
                const top3Mods = sortedFiles.slice(0, 3).reduce((a, f) => a + f[1], 0);
                const top3Pct = (top3Mods / totalFileMods) * 100;
                if (top3Pct > 30) {
                    addInsight('info', '📁', 'insight_hotspot_title', 'insight_hotspot_desc', { value: top3Pct.toFixed(0) });
                }
            }

            // --- Rule 6: Weekend Work ---
            const weekendCommits = filteredData.filter(d => d.dayOfWeek === 0 || d.dayOfWeek === 6).reduce((a, d) => a + d.commit_count, 0);
            const weekendPct = totalCommits > 0 ? (weekendCommits / totalCommits) * 100 : 0;
            if (weekendPct > 15) {
                addInsight('warning', '📅', 'insight_weekend_title', 'insight_weekend_desc', { value: weekendPct.toFixed(0) });
            }

            // --- Rule 7: Late Night Activity ---
            let lateCount = 0;
            filteredData.forEach(d => {
                lateCount += d.hours.filter(h => h >= 22 || h < 5).length;
            });
            const latePct = totalCommits > 0 ? (lateCount / totalCommits) * 100 : 0;
            if (latePct > 20) {
                addInsight('warning', '🌙', 'insight_latenight_title', 'insight_latenight_desc', { value: latePct.toFixed(0) });
            }

            // --- Rule 8: Stable Pace ---
            const start = new Date(startDate);
            const end = new Date(endDate);
            const totalDays = Math.max(1, Math.round((end - start) / 86400000) + 1);
            const activeDays = new Set(filteredData.map(d => d.dateStr)).size;
            const activePct = (activeDays / totalDays) * 100;
            if (activePct > 60 && churnRate < 30) {
                addInsight('positive', '✅', 'insight_stable_title', 'insight_stable_desc', { value: activePct.toFixed(0) });
            }

            // --- Rule 9: Small Commit Habits ---
            let smallCount = 0;
            filteredData.forEach(d => {
                smallCount += d.commit_sizes.filter(s => s <= 50).length;
            });
            const smallPct = totalCommits > 0 ? (smallCount / totalCommits) * 100 : 0;
            if (smallPct > 70) {
                addInsight('positive', '✅', 'insight_smallcommit_title', 'insight_smallcommit_desc', { value: smallPct.toFixed(0) });
            }

            // --- Rule 10: Isolated Files ---
            const fileOwners = {};
            dashboardData.file_stats.forEach(fs => {
                if (filteredAuthors.has(fs.author)) {
                    const fName = filePaths[fs.file_idx] || fs.file_idx;
                    if (!fileOwners[fName]) fileOwners[fName] = new Set();
                    fileOwners[fName].add(fs.author);
                }
            });
            const isolatedCount = Object.values(fileOwners).filter(s => s.size === 1).length;
            const totalFilesCount = Object.keys(fileOwners).length;
            if (isolatedCount > 0 && totalFilesCount > 3 && (isolatedCount / totalFilesCount) > 0.5) {
                addInsight('info', '📋', 'insight_isolated_title', 'insight_isolated_desc', { value: isolatedCount });
            }

            // --- Rule 11: Frequent Context Switching ---
            const relevantDirCounts = dashboardData.daily_dir_counts.filter(dc => dc.date >= startDate && dc.date <= endDate);
            const avgDirs = relevantDirCounts.length > 0 ? relevantDirCounts.reduce((a, b) => a + b.count, 0) / relevantDirCounts.length : 0;
            if (avgDirs > 3) {
                addInsight('warning', '🔀', 'insight_ctxswitch_title', 'insight_ctxswitch_desc', { value: avgDirs.toFixed(1) });
            }

            // --- Rule 12: Long-lived Branches ---
            const longLivedCount = dashboardData.merge_events.filter(me => 
                me.date >= startDate && me.date <= endDate && me.days > 7
            ).length;
            if (longLivedCount > 0) {
                addInsight('warning', '🔄', 'insight_longlived_title', 'insight_longlived_desc', { value: longLivedCount });
            }

            // Render
            if (insights.length === 0) {
                document.getElementById('insightsContainer').style.display = 'none';
            } else {
                document.getElementById('insightsContainer').style.display = '';
                insights.forEach(ins => {
                    const card = document.createElement('div');
                    card.className = `insight-card ${ins.severity}`;
                    card.innerHTML = `
                        <div class="insight-icon">${ins.icon}</div>
                        <div class="insight-body">
                            <div class="insight-title">${ins.title}</div>
                            <div class="insight-desc">${ins.desc}</div>
                        </div>
                    `;
                    container.appendChild(card);
                });
            }
        }

        function updateOwnershipChart(filteredData, startDate, endDate) {
            if (ownerChart) ownerChart.destroy();

            const filteredAuthors = new Set(filteredData.map(d => d.author));
            const fileUserMap = {};
            dashboardData.file_stats.forEach(fs => {
                if (filteredAuthors.has(fs.author)) {
                    const fName = filePaths[fs.file_idx] || `file_${fs.file_idx}`;
                    if (!fileUserMap[fName]) fileUserMap[fName] = {};
                    fileUserMap[fName][fs.author] = (fileUserMap[fName][fs.author] || 0) + fs.count;
                }
            });

            const fileTotals = Object.entries(fileUserMap).map(([f, users]) => ({
                file: f,
                total: Object.values(users).reduce((a, b) => a + b, 0),
                users
            })).sort((a, b) => b.total - a.total).slice(0, 15).reverse();

            if (fileTotals.length === 0) return;

            const fileLabels = fileTotals.map(f => {
                const parts = f.file.split('/');
                return parts.length > 2 ? '.../' + parts.slice(-2).join('/') : f.file;
            });
            const ownerUsers = [...new Set(fileTotals.flatMap(f => Object.keys(f.users)))];

            const datasets = ownerUsers.map(user => ({
                label: user,
                data: fileTotals.map(f => f.users[user] || 0),
                backgroundColor: stringToColor(user) + 'CC',
                borderWidth: 0
            }));

            ownerChart = new Chart(ownerCtx, {
                type: 'bar',
                data: { labels: fileLabels, datasets },
                options: {
                    indexAxis: 'y',
                    responsive: true, maintainAspectRatio: false,
                    plugins: {
                        legend: { position: 'top', labels: { boxWidth: 12, font: { size: 11 } } },
                        tooltip: {
                            callbacks: {
                                label: ctx => `${ctx.dataset.label}: ${ctx.raw} ${t('label_commits')}`
                            }
                        }
                    },
                    scales: {
                        x: { stacked: true, title: { display: true, text: t('label_commits') } },
                        y: { stacked: true, ticks: { font: { size: 10 } } }
                    }
                }
            });
        }

        function updateLeadTimeChart(filteredData, startDate, endDate) {
            if (leadChart) leadChart.destroy();

            const branches = dashboardData.merge_events
                .filter(me => me.date >= startDate && me.date <= endDate)
                .map(me => ({
                    name: me.branch.length > 25 ? me.branch.substring(0, 22) + '...' : me.branch,
                    days: me.days,
                    mergeDate: me.date
                }))
                .slice(0, 15).reverse();

            if (branches.length === 0) {
                leadChart = new Chart(leadCtx, {
                    type: 'bar',
                    data: { labels: [t('label_branch')], datasets: [{ data: [0], backgroundColor: '#bdc3c7' }] },
                    options: {
                        indexAxis: 'y', responsive: true, maintainAspectRatio: false,
                        plugins: {
                            legend: { display: false },
                            title: { display: true, text: 'No merge commits found', color: '#999' }
                        }
                    }
                });
                return;
            }

            const colors = branches.map(b => b.days > 7 ? '#e74c3c99' : b.days > 3 ? '#f39c1299' : '#27ae6099');

            leadChart = new Chart(leadCtx, {
                type: 'bar',
                data: {
                    labels: branches.map(b => b.name),
                    datasets: [{
                        label: t('label_leadtime_days'),
                        data: branches.map(b => b.days),
                        backgroundColor: colors,
                        borderWidth: 0
                    }]
                },
                options: {
                    indexAxis: 'y',
                    responsive: true, maintainAspectRatio: false,
                    plugins: {
                        legend: { display: false },
                        tooltip: {
                            callbacks: {
                                label: ctx => `${ctx.raw} ${t('label_days')}`
                            }
                        }
                    },
                    scales: {
                        x: { title: { display: true, text: t('label_leadtime_days') } },
                        y: { ticks: { font: { size: 10 } } }
                    }
                }
            });
        }

        function updateContextSwitchChart(filteredData, startDate, endDate) {
            if (ctxChart) ctxChart.destroy();

            const relevantCounts = dashboardData.daily_dir_counts
                .filter(dc => dc.date >= startDate && dc.date <= endDate)
                .sort((a, b) => a.date.localeCompare(b.date));

            const dates = relevantCounts.map(dc => dc.date);
            const dirCounts = relevantCounts.map(dc => dc.count);

            if (dates.length === 0) return;

            ctxChart = new Chart(ctxSwitchCtx, {
                type: 'line',
                data: {
                    labels: dates,
                    datasets: [{
                        label: t('label_avg_dirs'),
                        data: dirCounts,
                        borderColor: '#9b59b6',
                        backgroundColor: '#9b59b633',
                        fill: true,
                        tension: 0.3,
                        pointRadius: 3
                    }]
                },
                options: {
                    responsive: true, maintainAspectRatio: false,
                    plugins: { legend: { display: false } },
                    scales: {
                        x: { ticks: { maxTicksLimit: 15 } },
                        y: { beginAtZero: true, title: { display: true, text: t('label_avg_dirs') } }
                    }
                }
            });
        }


        function updateHealthTrendChart(filteredData, startDate, endDate) {
            // Generate dense date list
            const dateMap = new Map();
            let curr = new Date(startDate);
            const end = new Date(endDate);
            const displayDates = [];
            
            while (curr <= end) {
                const dStr = curr.toISOString().split('T')[0];
                displayDates.push(dStr);
                dateMap.set(dStr, { 
                    total_changes: 0, 
                    total_churn: 0,
                    durations: [] 
                });
                curr.setDate(curr.getDate() + 1);
            }

            filteredData.forEach(d => {
                if (!dateMap.has(d.dateStr)) return;
                const daily = dateMap.get(d.dateStr);
                
                daily.total_changes += d.total_changes;
                daily.total_churn += d.churn;
                
                if (d.hours && d.hours.length > 1) {
                    daily.durations.push(Math.max(...d.hours) - Math.min(...d.hours));
                }
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
                const avgDur = stats.durations.length > 0
                    ? stats.durations.reduce((a, b) => a + b, 0) / stats.durations.length
                    : 0;
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
                            borderColor: '#e74c3c',
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
                            borderColor: '#8e44ad',
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
                    responsive: true, maintainAspectRatio: false,
                    interaction: { mode: 'index', intersect: false },
                    scales: {
                        x: { display: true },
                        y: {
                            type: 'linear', display: true, position: 'left',
                            title: { display: true, text: 'Churn Rate (%)' },
                            beginAtZero: true, max: 100
                        },
                        y1: {
                            type: 'linear', display: true, position: 'right',
                            title: { display: true, text: 'Hours' },
                            grid: { drawOnChartArea: false },
                            beginAtZero: true, max: 24
                        }
                    },
                    plugins: {
                        tooltip: {
                            callbacks: {
                                label: function(context) {
                                    let label = context.dataset.label || '';
                                    if (label) label += ': ';
                                    if (context.parsed.y !== null) label += context.parsed.y.toFixed(1);
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
            const activeDays = new Set(currentData.map(d => d.dateStr)).size;
            const avgPerDay = activeDays > 0 ? (currentTotal / activeDays).toFixed(1) : 0;
            
            const totalChanges = currentData.reduce((acc, d) => acc + d.total_changes, 0);
            const totalChurn = currentData.reduce((acc, d) => acc + d.churn, 0);
            const totalMerges = currentData.reduce((acc, d) => acc + d.merges, 0);
            const churnRate = totalChanges > 0 ? ((totalChurn / totalChanges) * 100).toFixed(1) : 0;

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
            document.getElementById('mergeCommitsValue').textContent = totalMerges.toLocaleString();
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
            const labels = ['< 1h', '1-4h', '4-8h', '8h+'];
            const userDatasets = {};

            filteredData.forEach(d => {
                if (d.hours && d.hours.length > 1) {
                    if (!userDatasets[d.author]) userDatasets[d.author] = [0, 0, 0, 0];
                    const duration = Math.max(...d.hours) - Math.min(...d.hours);
                    if (duration < 1) userDatasets[d.author][0]++;
                    else if (duration < 4) userDatasets[d.author][1]++;
                    else if (duration < 8) userDatasets[d.author][2]++;
                    else userDatasets[d.author][3]++;
                }
            });

            const datasets = Object.entries(userDatasets).map(([user, bins]) => ({
                label: user,
                data: bins,
                backgroundColor: stringToColor(user),
                stack: 'stack1'
            }));

            if (durChart) durChart.destroy();
            durChart = new Chart(durCtx, {
                type: 'bar',
                data: { labels, datasets },
                options: {
                    responsive: true, maintainAspectRatio: false,
                    scales: {
                        x: { stacked: true },
                        y: { stacked: true, beginAtZero: true, title: { display: true, text: t('label_days_count') } }
                    },
                    plugins: { legend: { position: 'top', labels: { boxWidth: 12 } } }
                }
            });
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
            const yLabel = t('metric_' + (metric === 'total_changes' ? 'total' : (metric === 'commit_count' ? 'commits' : metric)));

            mainChart = new Chart(ctx, {
                type: chartType,
                data: { labels: displayDates, datasets: datasets },
                options: {
                    responsive: true, maintainAspectRatio: false,
                    scales: {
                        x: { stacked: chartType === 'bar' },
                        y: { stacked: chartType === 'bar', beginAtZero: true, title: { display: true, text: yLabel } }
                    },
                    plugins: { tooltip: { mode: 'index', intersect: false } },
                    interaction: { mode: 'nearest', axis: 'x', intersect: false }
                }
            });
        }

        function updatePieChart(filteredData, metric) {
            const userTotals = {};
            filteredData.forEach(d => {
                userTotals[d.author] = (userTotals[d.author] || 0) + (d[metric] || 0);
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
                    responsive: true, maintainAspectRatio: false,
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
                dayTotals[d.dayOfWeek] += (d[metric] || 0);
            });

            if (dowChart) dowChart.destroy();
            const yLabel = t('metric_' + (metric === 'total_changes' ? 'total' : (metric === 'commit_count' ? 'commits' : metric)));

            dowChart = new Chart(dowCtx, {
                type: 'bar',
                data: {
                    labels: days,
                    datasets: [{
                        label: t('label_activity'),
                        data: dayTotals,
                        backgroundColor: 'rgba(52, 152, 219, 0.7)',
                        borderColor: 'rgba(52, 152, 219, 1)',
                        borderWidth: 1
                    }]
                },
                options: {
                    responsive: true, maintainAspectRatio: false,
                    plugins: { legend: { display: false } },
                    scales: { y: { beginAtZero: true, title: { display: true, text: yLabel } } }
                }
            });
        }

        function updateHeatmapChart(filteredData, metric) {
            const heatmapData = [];
            const counts = {}; // "dow-hour" -> total metric

            filteredData.forEach(d => {
                if (d.hours) {
                    d.hours.forEach(h => {
                        const key = `${d.dayOfWeek}-${h}`;
                        counts[key] = (counts[key] || 0) + 1; // Heatmap usually shows commit frequency
                    });
                }
            });

            for (let d = 0; d < 7; d++) {
                for (let h = 0; h < 24; h++) {
                    heatmapData.push({ x: h, y: d, v: counts[`${d}-${h}`] || 0 });
                }
            }

            if (heatmapChart) heatmapChart.destroy();
            heatmapChart = new Chart(heatmapCtx, {
                type: 'matrix',
                data: {
                    datasets: [{
                        label: t('label_activity_heatmap'),
                        data: heatmapData,
                        backgroundColor(ctx) {
                            const value = ctx.dataset.data[ctx.dataIndex].v;
                            const alpha = Math.min(value / 10, 1);
                            return `rgba(52, 152, 219, ${alpha})`;
                        },
                        width: ({ chart }) => chart.chartArea ? (chart.chartArea.width / 24) - 1 : 0,
                        height: ({ chart }) => chart.chartArea ? (chart.chartArea.height / 7) - 1 : 0
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
                                    const days = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];
                                    return `${days[d.y]} ${d.x}:00 - ${d.v} ${t('label_commits')}`;
                                }
                            }
                        }
                    },
                    scales: {
                        x: { type: 'linear', min: 0, max: 23, ticks: { stepSize: 1, callback: v => v + ':00' }, grid: { display: false } },
                        y: { type: 'linear', min: 0, max: 6, ticks: { stepSize: 1, callback: v => ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'][v] }, grid: { display: false }, reverse: true }
                    }
                }
            });
        }

        function updateSizeDistChart(filteredData) {
            const bins = ['XS (<10)', 'S (10-50)', 'M (50-200)', 'L (200-500)', 'XL (500+)'];
            const counts = [0, 0, 0, 0, 0];

            filteredData.forEach(d => {
                if (d.commit_sizes) {
                    d.commit_sizes.forEach(s => {
                        if (s < 10) counts[0]++;
                        else if (s < 50) counts[1]++;
                        else if (s < 200) counts[2]++;
                        else if (s < 500) counts[3]++;
                        else counts[4]++;
                    });
                }
            });

            if (sizeChart) sizeChart.destroy();
            sizeChart = new Chart(sizeCtx, {
                type: 'bar',
                data: {
                    labels: bins,
                    datasets: [{
                        label: t('label_commits'),
                        data: counts,
                        backgroundColor: '#f1c40f99',
                        borderColor: '#f1c40f',
                        borderWidth: 1
                    }]
                },
                options: {
                    responsive: true, maintainAspectRatio: false,
                    plugins: { legend: { display: false } },
                    scales: { y: { beginAtZero: true, title: { display: true, text: t('label_commits') } } }
                }
            });
        }

        function updateHotspotsChart(filteredData, startDate, endDate) {
            if (hotChart) hotChart.destroy();

            const filteredAuthors = new Set(filteredData.map(d => d.author));
            const fileCounts = {};
            dashboardData.file_stats.forEach(fs => {
                if (filteredAuthors.has(fs.author)) {
                    const fName = filePaths[fs.file_idx] || `file_${fs.file_idx}`;
                    fileCounts[fName] = (fileCounts[fName] || 0) + fs.count;
                }
            });

            const topFiles = Object.entries(fileCounts)
                .sort((a, b) => b[1] - a[1])
                .slice(0, 15)
                .reverse();

            if (topFiles.length === 0) return;

            const labels = topFiles.map(f => {
                const name = f[0];
                const parts = name.split('/');
                return parts.length > 2 ? '.../' + parts.slice(-2).join('/') : name;
            });

            hotChart = new Chart(hotCtx, {
                type: 'bar',
                data: {
                    labels: labels,
                    datasets: [{
                        label: t('label_commits'),
                        data: topFiles.map(f => f[1]),
                        backgroundColor: '#e67e2299',
                        borderColor: '#e67e22',
                        borderWidth: 1
                    }]
                },
                options: {
                    indexAxis: 'y',
                    responsive: true, maintainAspectRatio: false,
                    plugins: {
                        legend: { display: false },
                        tooltip: {
                            callbacks: {
                                title: (ctx) => topFiles[ctx[0].dataIndex][0],
                                label: (ctx) => `${ctx.raw} ${t('label_commits')}`
                            }
                        }
                    },
                    scales: {
                        x: { beginAtZero: true, title: { display: true, text: t('label_commits') } },
                        y: { ticks: { font: { size: 10 } } }
                    }
                }
            });
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

        function updatePredictiveDashboard(filteredData) {
            const weeklyStats = getWeeklyStats(filteredData);
            if (weeklyStats.length < 2) {
                // Not enough data for prediction
                document.getElementById('currentVelocityValue').textContent = '-';
                document.getElementById('velocityTrendValue').textContent = '-';
                document.getElementById('projectedThroughputValue').textContent = '-';
                document.getElementById('estCompletionValue').textContent = '-';
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

            document.getElementById('currentVelocityValue').textContent = `${currentVelocity.toFixed(1)} ${t('header_commits')}/week`;
            
            const projected60 = Math.round(currentVelocity * (60/7));
            document.getElementById('projectedThroughputValue').textContent = `${projected60.toLocaleString()} ${t('header_commits')}`;

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
            
            // 1. Basic Stats
            filteredData.forEach(d => {
                const user = d.author;
                if (!userStats[user]) {
                    userStats[user] = {
                        commits: 0, added: 0, deleted: 0,
                        activeDays: new Set(),
                        dirs: {},
                        leadTimes: [],
                        reviewsGiven: 0
                    };
                }
                userStats[user].commits += d.commit_count;
                userStats[user].added += d.added;
                userStats[user].deleted += d.deleted;
                userStats[user].activeDays.add(d.dateStr);
            });

            const currentUsers = new Set(Object.keys(userStats));

            // 2. Directory expertise
            dashboardData.file_stats.forEach(fs => {
                if (currentUsers.has(fs.author)) {
                    const path = filePaths[fs.file_idx];
                    if (path) {
                        const dir = path.includes('/') ? path.split('/')[0] : '(root)';
                        userStats[fs.author].dirs[dir] = (userStats[fs.author].dirs[dir] || 0) + fs.count;
                    }
                }
            });

            // 3. Lead times
            dashboardData.merge_events.forEach(me => {
                if (currentUsers.has(me.author)) {
                    userStats[me.author].leadTimes.push(me.days);
                }
            });

            // 4. GitHub Reviews
            if (dashboardData.github_prs && dashboardData.github_prs.length > 0) {
                document.getElementById('githubSection').style.display = 'block';
                const ghTbody = document.getElementById('githubTableBody');
                ghTbody.innerHTML = '';

                dashboardData.github_prs.forEach(pr => {
                    pr.reviews.forEach(rev => {
                        if (userStats[rev.user]) {
                            userStats[rev.user].reviewsGiven++;
                        } else if (currentUsers.size > 0) {
                            // Even if no commits, we might want to track reviewers
                            // but for now we only track reviewers who are also committers in the current view
                        }
                    });

                    const tr = document.createElement('tr');
                    const reviewSummary = pr.reviews.map(r => 
                        `<span class="badge" style="background: ${r.state === 'APPROVED' ? '#ecfaf2' : '#fdf2f2'}; color: ${r.state === 'APPROVED' ? '#27ae60' : '#e74c3c'}; font-size: 10px; padding: 2px 5px;">${r.user}</span>`
                    ).join(' ');

                    tr.innerHTML = `
                        <td><a href="${pr.html_url}" target="_blank">#${pr.number} ${pr.title}</a></td>
                        <td>${pr.author}</td>
                        <td>${pr.reviews.length > 0 ? 'Reviewed' : 'Pending'}</td>
                        <td>${reviewSummary}</td>
                    `;
                    ghTbody.appendChild(tr);
                });
            }

            const tbody = document.getElementById('userTableBody');
            tbody.innerHTML = '';

            const statsArray = Object.entries(userStats).map(([user, stats]) => {
                const totalChanges = stats.added + stats.deleted;
                const avgLeadTime = stats.leadTimes.length > 0 
                    ? (stats.leadTimes.reduce((a, b) => a + b, 0) / stats.leadTimes.length).toFixed(1)
                    : '-';
                
                const topDirs = Object.entries(stats.dirs)
                    .sort((a, b) => b[1] - a[1])
                    .slice(0, 3)
                    .map(d => d[0])
                    .join(', ');

                return { user, ...stats, totalChanges, avgLeadTime, topDirs };
            });

            statsArray.sort((a, b) => b.totalChanges - a.totalChanges);

            statsArray.forEach(s => {
                const tr = document.createElement('tr');
                tr.innerHTML = `
                    <td>
                        <div class="user-info">
                            <div class="user-avatar" style="background-color: ${stringToColor(s.user)}"></div>
                            <strong>${s.user}</strong>
                        </div>
                    </td>
                    <td>${s.commits.toLocaleString()}</td>
                    <td><span class="badge added">+${s.added.toLocaleString()}</span></td>
                    <td><span class="badge deleted">-${s.deleted.toLocaleString()}</span></td>
                    <td><strong>${s.totalChanges.toLocaleString()}</strong></td>
                    <td>${s.reviewsGiven}</td>
                    <td>${s.avgLeadTime}${s.avgLeadTime !== '-' ? ' ' + t('label_days') : ''}</td>
                    <td>${s.activeDays.size}</td>
                    <td style="font-size: 12px; color: #666;">${s.topDirs || '-'}</td>
                `;
                tbody.appendChild(tr);
            });
        }

        // Initial render
        loadStateFromUrl();
        renderUserCheckboxes();
        updateDashboard();
    </script>
</body>
</html>
"#;



