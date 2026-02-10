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

        <div class="insights-section" id="insightsContainer">
            <h2>💡 <span data-i18n="insights_title">Insights</span></h2>
            <div class="insights-grid" id="insightsGrid"></div>
        </div>

        <div class="charts-grid">
            <div class="chart-box full-width">
                <div class="chart-title">
                    <span data-i18n="chart_timeline">Timeline</span> 
                    <span class="info-icon" data-i18n-tooltip="tooltip_timeline" data-tooltip="Shows activity trends over time. Look for spikes (sprints/releases) or gaps (blockers/downtime). Ideally, activity should be consistent.">i</span>
                </div>
                <canvas id="productivityChart"></canvas>
            </div>
            <div class="chart-box">
                <div class="chart-title">
                    <span data-i18n="chart_share">User Share</span>
                    <span class="info-icon" data-i18n-tooltip="tooltip_share" data-tooltip="Distribution of contributions. Helps identify 'Bus Factor' (reliance on single dev) or uneven workload distribution.">i</span>
                </div>
                <canvas id="shareChart"></canvas>
            </div>
            <div class="chart-box">
                <div class="chart-title">
                    <span data-i18n="chart_dow">Day of Week Activity</span>
                    <span class="info-icon" data-i18n-tooltip="tooltip_dow" data-tooltip="Weekly rhythm. Most teams peak Tue-Thu. High weekend activity might indicate crunch time or unhealthy work habits.">i</span>
                </div>
                <canvas id="dayOfWeekChart"></canvas>
            </div>
            <div class="chart-box full-width">
                <div class="chart-title">
                    <span data-i18n="chart_heatmap">Activity Heatmap (Hour vs Day)</span>
                    <span class="info-icon" data-i18n-tooltip="tooltip_heatmap" data-tooltip="Identifies core working hours. Look for clusters outside normal hours (e.g. late nights), which suggests overtime or burnout risk.">i</span>
                </div>
                <canvas id="heatmapChart"></canvas>
            </div>
             <div class="chart-box full-width">
                <div class="chart-title">
                    <span data-i18n="chart_size">Commit Size Distribution</span>
                    <span class="info-icon" data-i18n-tooltip="tooltip_size" data-tooltip="Breakdown of commit sizes. 'XS'/'S' are ideal (atomic commits). Too many 'XL' suggests large, risky changes that are hard to review.">i</span>
                </div>
                <canvas id="sizeDistChart"></canvas>
            </div>
            <div class="chart-box full-width">
                <div class="chart-title">
                    <span data-i18n="chart_hotspots">File Hotspots (Top 20 Modified)</span>
                    <span class="info-icon" data-i18n-tooltip="tooltip_hotspots" data-tooltip="Most frequently changed files. These are potential architectural bottlenecks, 'God Classes', or unstable modules needing refactoring.">i</span>
                </div>
                <canvas id="hotspotsChart"></canvas>
            </div>
            <div class="chart-box full-width">
                <div class="chart-title">
                    <span data-i18n="chart_duration">Est. Daily Work Duration</span>
                    <span class="info-icon" data-i18n-tooltip="tooltip_duration" data-tooltip="Time between first and last commit of the day. NOTE: Not actual work hours, but indicates span of activity. Long spans may suggest burnout.">i</span>
                </div>
                <canvas id="workDurationChart"></canvas>
            </div>
             <div class="chart-box full-width">
                <div class="chart-title">
                    <span data-i18n="chart_health">Team Health Trends</span>
                    <span class="info-icon" data-i18n-tooltip="tooltip_health" data-tooltip="Red: Churn Rate (Rework/Volatility). High = Unstable/Refactoring.
Purple: Avg Duration. Rising trend = Potential Overwork.">i</span>
                </div>
                <canvas id="healthTrendChart"></canvas>
            </div>
            <div class="chart-box full-width">
                <div class="chart-title">
                    <span data-i18n="chart_ownership">Code Ownership (Top 15 Files)</span>
                    <span class="info-icon" data-i18n-tooltip="tooltip_ownership" data-tooltip="Shows who contributes to which files. Files with only one contributor are a 'Bus Factor' risk. Balanced ownership improves team resilience.">i</span>
                </div>
                <canvas id="ownershipChart"></canvas>
            </div>
            <div class="chart-box full-width">
                <div class="chart-title">
                    <span data-i18n="chart_leadtime">Branch Lead Time</span>
                    <span class="info-icon" data-i18n-tooltip="tooltip_leadtime" data-tooltip="Time span of merged branches (from first commit to merge). Shorter lead times indicate faster delivery. Long-lived branches increase merge complexity.">i</span>
                </div>
                <canvas id="leadTimeChart"></canvas>
            </div>
            <div class="chart-box full-width">
                <div class="chart-title">
                    <span data-i18n="chart_ctxswitch">Context Switching (Daily Directory Diversity)</span>
                    <span class="info-icon" data-i18n-tooltip="tooltip_ctxswitch" data-tooltip="Number of distinct directories touched per day. High values indicate frequent context switching, which reduces focus and deep work. Lower is generally better.">i</span>
                </div>
                <canvas id="ctxSwitchChart"></canvas>
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
                insight_longlived_desc: "{value} branch(es) lived longer than 7 days. Long-lived branches increase merge complexity and risk."
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
                insight_longlived_desc: "{value}個のブランチが7日以上存続しています。長命ブランチはマージの複雑さとリスクを増大させます。"
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
        const ownerCtx = document.getElementById('ownershipChart').getContext('2d');
        const leadCtx = document.getElementById('leadTimeChart').getContext('2d');
        const ctxSwitchCtx = document.getElementById('ctxSwitchChart').getContext('2d');

        let mainChart, pieChart, dowChart, heatmapChart, sizeChart, hotChart, durChart, healthChart, ownerChart, leadChart, ctxChart;

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
            updateOwnershipChart(filteredData);
            updateLeadTimeChart(filteredData);
            updateContextSwitchChart(filteredData);
            generateInsights(filteredData, startDate, endDate);
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
                Object.entries(values).forEach(([k, v]) => {
                    desc = desc.replace(`{${k}}`, v);
                });
                insights.push({ severity, icon, title: t(titleKey), desc });
            }

            // --- Rule 1: Burnout Risk (avg work span > 8h in last 7 days) ---
            const recentDates = [...new Set(filteredData.map(d => d.dateStr))].sort().slice(-7);
            const userDailySpans = new Map();
            filteredData.filter(d => recentDates.includes(d.dateStr)).forEach(d => {
                const key = `${d.dateStr}-${d.author}`;
                if (!userDailySpans.has(key)) userDailySpans.set(key, { min: 24, max: 0, count: 0 });
                const s = userDailySpans.get(key);
                if (d.hour < s.min) s.min = d.hour;
                if (d.hour > s.max) s.max = d.hour;
                s.count++;
            });
            let totalSpan = 0, spanCount = 0;
            userDailySpans.forEach(s => {
                if (s.count > 1) { totalSpan += (s.max - s.min); spanCount++; }
            });
            const avgSpan = spanCount > 0 ? totalSpan / spanCount : 0;
            if (avgSpan > 8) {
                addInsight('warning', '🔥', 'insight_burnout_title', 'insight_burnout_desc', { value: avgSpan.toFixed(1) });
            }

            // --- Rule 2: Code Instability (Churn Rate > 50%) ---
            const totalChanges = filteredData.reduce((a, d) => a + d.total_changes, 0);
            const totalChurn = filteredData.reduce((a, d) => a + d.churn, 0);
            const churnRate = totalChanges > 0 ? (totalChurn / totalChanges) * 100 : 0;
            if (churnRate > 50) {
                addInsight('warning', '📉', 'insight_unstable_title', 'insight_unstable_desc', { value: churnRate.toFixed(1) });
            }

            // --- Rule 3: Bus Factor (top contributor > 70%) ---
            const userCommits = {};
            filteredData.forEach(d => { userCommits[d.author] = (userCommits[d.author] || 0) + 1; });
            const totalCommits = filteredData.length;
            const sortedUsers = Object.entries(userCommits).sort((a, b) => b[1] - a[1]);
            if (sortedUsers.length > 0) {
                const topShare = (sortedUsers[0][1] / totalCommits) * 100;
                if (topShare > 70) {
                    addInsight('warning', '🚌', 'insight_busfactor_title', 'insight_busfactor_desc', { name: sortedUsers[0][0], value: topShare.toFixed(0) });
                }
            }

            // --- Rule 4: Large Commits (XL > 20%) ---
            const xlCount = filteredData.filter(d => d.total_changes > 500).length;
            const xlPct = totalCommits > 0 ? (xlCount / totalCommits) * 100 : 0;
            if (xlPct > 20) {
                addInsight('info', '📦', 'insight_largecommit_title', 'insight_largecommit_desc', { value: xlPct.toFixed(0) });
            }

            // --- Rule 5: Hotspot Concentration (Top 3 files > 30%) ---
            const fileCounts = {};
            filteredData.forEach(d => {
                (d.files || []).forEach(fIdx => {
                    const fName = filePaths[fIdx] || fIdx;
                    fileCounts[fName] = (fileCounts[fName] || 0) + 1;
                });
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

            // --- Rule 6: Weekend Work (> 15%) ---
            const weekendCount = filteredData.filter(d => d.dayOfWeek === 0 || d.dayOfWeek === 6).length;
            const weekendPct = totalCommits > 0 ? (weekendCount / totalCommits) * 100 : 0;
            if (weekendPct > 15) {
                addInsight('warning', '📅', 'insight_weekend_title', 'insight_weekend_desc', { value: weekendPct.toFixed(0) });
            }

            // --- Rule 7: Late Night Activity (22-5h > 20%) ---
            const lateCount = filteredData.filter(d => d.hour >= 22 || d.hour < 5).length;
            const latePct = totalCommits > 0 ? (lateCount / totalCommits) * 100 : 0;
            if (latePct > 20) {
                addInsight('warning', '🌙', 'insight_latenight_title', 'insight_latenight_desc', { value: latePct.toFixed(0) });
            }

            // --- Rule 8: Stable Pace (active > 60% of days & churn < 30%) ---
            const start = new Date(startDate);
            const end = new Date(endDate);
            const totalDays = Math.max(1, Math.round((end - start) / 86400000) + 1);
            const activeDays = new Set(filteredData.map(d => d.dateStr)).size;
            const activePct = (activeDays / totalDays) * 100;
            if (activePct > 60 && churnRate < 30) {
                addInsight('positive', '✅', 'insight_stable_title', 'insight_stable_desc', { value: activePct.toFixed(0) });
            }

            // --- Rule 9: Small Commit Habits (XS+S > 70%) ---
            const smallCount = filteredData.filter(d => d.total_changes <= 50).length;
            const smallPct = totalCommits > 0 ? (smallCount / totalCommits) * 100 : 0;
            if (smallPct > 70) {
                addInsight('positive', '✅', 'insight_smallcommit_title', 'insight_smallcommit_desc', { value: smallPct.toFixed(0) });
            }

            // --- Rule 10: Isolated Files (files with only 1 contributor) ---
            const fileOwners = {};
            filteredData.forEach(d => {
                (d.files || []).forEach(fIdx => {
                    const fName = filePaths[fIdx] || fIdx;
                    if (!fileOwners[fName]) fileOwners[fName] = new Set();
                    fileOwners[fName].add(d.author);
                });
            });
            const isolatedCount = Object.values(fileOwners).filter(s => s.size === 1).length;
            const totalFilesCount = Object.keys(fileOwners).length;
            if (isolatedCount > 0 && totalFilesCount > 3 && (isolatedCount / totalFilesCount) > 0.5) {
                addInsight('info', '📋', 'insight_isolated_title', 'insight_isolated_desc', { value: isolatedCount });
            }

            // --- Rule 11: Frequent Context Switching (avg dirs > 3) ---
            const dayDirMap = {}; 
            filteredData.forEach(d => {
                if (!dayDirMap[d.dateStr]) dayDirMap[d.dateStr] = new Set();
                (d.files || []).forEach(fIdx => {
                    const fName = filePaths[fIdx] || '';
                    const dir = fName.includes('/') ? fName.split('/')[0] : '(root)';
                    dayDirMap[d.dateStr].add(dir);
                });
            });
            const datesArr = Object.keys(dayDirMap);
            const avgDirs = datesArr.length > 0 ? Object.values(dayDirMap).reduce((a, b) => a + b.size, 0) / datesArr.length : 0;
            if (avgDirs > 3) {
                addInsight('warning', '🔀', 'insight_ctxswitch_title', 'insight_ctxswitch_desc', { value: avgDirs.toFixed(1) });
            }

            // --- Rule 12: Long-lived Branches (> 7 days) ---
            const mergeCommits = filteredData.filter(d => d.is_merge && d.message);
            let longLivedCount = 0;
            mergeCommits.forEach(mc => {
                const match = mc.message.match(/Merge\s+(?:branch|pull request)\s+'?([^'"\s]+)'?/i);
                if (!match) return;
                const mergeDate = new Date(mc.date);
                const parentCommits = filteredData.filter(d => 
                    !d.is_merge && 
                    new Date(d.date) <= mergeDate && 
                    new Date(d.date) >= new Date(mergeDate.getTime() - 90 * 86400000)
                );
                if (parentCommits.length > 0) {
                    const daysDiff = (mergeDate - new Date(parentCommits[parentCommits.length - 1].date)) / 86400000;
                    if (daysDiff > 7) longLivedCount++;
                }
            });
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

        function updateOwnershipChart(filteredData) {
            if (ownerChart) ownerChart.destroy();

            // Build file -> { user -> count } map
            const fileUserMap = {};
            filteredData.forEach(d => {
                (d.files || []).forEach(fIdx => {
                    const fName = filePaths[fIdx] || `file_${fIdx}`;
                    if (!fileUserMap[fName]) fileUserMap[fName] = {};
                    fileUserMap[fName][d.author] = (fileUserMap[fName][d.author] || 0) + 1;
                });
            });

            // Sort files by total modifications, take top 15
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

        function updateLeadTimeChart(filteredData) {
            if (leadChart) leadChart.destroy();

            // Find merge commits and extract branch names
            const mergeCommits = filteredData.filter(d => d.is_merge && d.message);
            const branches = [];

            mergeCommits.forEach(mc => {
                // Match "Merge branch 'xxx'" or "Merge branch 'xxx' into yyy"
                const match = mc.message.match(/Merge\s+(?:branch|pull request)\s+'?([^'"\s]+)'?/i);
                if (!match) return;
                const branchName = match[1];
                const mergeDate = new Date(mc.date);

                // Find the earliest non-merge commit closest before this merge
                // Use the parent commits' time range as approximation
                const parentCommits = filteredData.filter(d =>
                    !d.is_merge &&
                    new Date(d.date) <= mergeDate &&
                    new Date(d.date) >= new Date(mergeDate.getTime() - 90 * 86400000) // within 90 days
                );

                if (parentCommits.length === 0) return;

                // Estimate: look for commits by same author within a reasonable timeframe
                const daysDiff = Math.max(1, Math.round(
                    (mergeDate - new Date(parentCommits[parentCommits.length - 1].date)) / 86400000
                ));

                branches.push({
                    name: branchName.length > 25 ? branchName.substring(0, 22) + '...' : branchName,
                    days: Math.min(daysDiff, 90),
                    mergeDate: mc.dateStr
                });
            });

            // Take last 15 branches
            const recentBranches = branches.slice(0, 15).reverse();

            if (recentBranches.length === 0) {
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

            const colors = recentBranches.map(b => b.days > 7 ? '#e74c3c99' : b.days > 3 ? '#f39c1299' : '#27ae6099');

            leadChart = new Chart(leadCtx, {
                type: 'bar',
                data: {
                    labels: recentBranches.map(b => b.name),
                    datasets: [{
                        label: t('label_leadtime_days'),
                        data: recentBranches.map(b => b.days),
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

        function updateContextSwitchChart(filteredData) {
            if (ctxChart) ctxChart.destroy();

            // For each day, count unique top-level directories per user
            const dayDirMap = {}; // dateStr -> Set of dirs
            filteredData.forEach(d => {
                if (!dayDirMap[d.dateStr]) dayDirMap[d.dateStr] = new Set();
                (d.files || []).forEach(fIdx => {
                    const fName = filePaths[fIdx] || '';
                    const dir = fName.includes('/') ? fName.split('/')[0] : '(root)';
                    dayDirMap[d.dateStr].add(dir);
                });
            });

            const dates = Object.keys(dayDirMap).sort();
            const dirCounts = dates.map(d => dayDirMap[d].size);

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



