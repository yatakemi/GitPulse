
function selectAllUsers(shouldSelect) {
    document.querySelectorAll('.user-checkbox').forEach(cb => cb.checked = shouldSelect);
    if (shouldSelect) {
        selectedUsers = new Set(allUsers);
    } else {
        selectedUsers.clear();
    }
    updateDashboard();
}

function toggleSort(column) {
    if (currentSort.column === column) {
        currentSort.direction = currentSort.direction === 'desc' ? 'asc' : 'desc';
    } else {
        currentSort.column = column;
        currentSort.direction = 'desc'; // Default to desc for new column
    }
    updateUserList();
}

function syncStateToUrl() {
    const params = new URLSearchParams();
    params.set('start', document.getElementById('startDate').value);
    params.set('end', document.getElementById('endDate').value);
    params.set('lang', document.getElementById('langSelect').value);
    params.set('metric', document.getElementById('metricSelect').value);
    params.set('chart', document.getElementById('chartTypeSelect').value);
    params.set('trend', document.getElementById('showTrend').checked);

    if (selectedUsers.size !== allUsers.length) {
        params.set('users', Array.from(selectedUsers).join(','));
    }

    const newUrl = window.location.pathname + '#' + params.toString();
    history.replaceState(null, '', newUrl);
}

function loadStateFromUrl() {
    if (!window.location.hash) return;
    const params = new URLSearchParams(window.location.hash.substring(1));

    if (params.has('start')) document.getElementById('startDate').value = params.get('start');
    if (params.has('end')) document.getElementById('endDate').value = params.get('end');
    if (params.has('lang')) {
        const lang = params.get('lang');
        document.getElementById('langSelect').value = lang;
        currentLang = lang;
        applyTranslations();
    }
    if (params.has('metric')) document.getElementById('metricSelect').value = params.get('metric');
    if (params.has('chart')) document.getElementById('chartTypeSelect').value = params.get('chart');
    if (params.has('trend')) document.getElementById('showTrend').checked = params.get('trend') === 'true';

    if (params.has('users')) {
        const users = params.get('users').split(',');
        selectedUsers = new Set(users.filter(u => allUsers.includes(u)));
    }
}

const _updateDashboardDebounced = debounce(_updateDashboardInternal, 250);

function updateDashboard(immediate = false) {
    if (immediate === true) {
        _updateDashboardInternal();
    } else {
        _updateDashboardDebounced();
    }
}

async function _updateDashboardInternal() {
    const startDate = document.getElementById('startDate').value;
    const endDate = document.getElementById('endDate').value;
    const metric = document.getElementById('metricSelect').value;
    const chartType = document.getElementById('chartTypeSelect').value;
    const showTrend = document.getElementById('showTrend').checked;

    if (!startDate || !endDate) return;

    syncStateToUrl();

    // Filter aggregated daily stats
    const filteredData = data.filter(d =>
        d.dateStr >= startDate && d.dateStr <= endDate && selectedUsers.has(d.author)
    );

    // 1. Calculate Previous Period for comparison
    const start = new Date(startDate);
    const end = new Date(endDate);
    const diffTime = Math.abs(end - start);
    const diffDays = Math.ceil(diffTime / (1000 * 60 * 60 * 24)) + 1; // inclusive

    const prevEnd = new Date(start);
    prevEnd.setDate(start.getDate() - 1);
    const prevStart = new Date(prevEnd);
    prevStart.setDate(prevEnd.getDate() - (diffDays - 1));

    const prevStartStr = prevStart.toISOString().split('T')[0];
    const prevEndStr = prevEnd.toISOString().split('T')[0];

    const previousData = data.filter(d =>
        d.dateStr >= prevStartStr && d.dateStr <= prevEndStr && selectedUsers.has(d.author)
    );

    // 2. High Priority: Summary and Primary Chart
    updateSummary(filteredData, previousData, metric, startDate, endDate);
    updateTimelineChart(filteredData, metric, chartType, showTrend, startDate, endDate);

    await yieldToMain();

    // 2. Secondary Priority: Core Charts
    updatePieChart(filteredData, metric);
    updateDayOfWeekChart(filteredData, metric);
    updateHeatmapChart(filteredData, metric);
    updateSizeDistChart(filteredData);
    updateWorkDurationChart(filteredData);
    updateHealthTrendChart(filteredData, startDate, endDate);

    await yieldToMain();

    // 3. Background: Ownership, Multi-author analysis, and Lead Time
    updateOwnershipChart(filteredData, startDate, endDate);
    updateIsolatedFilesTable(filteredData);
    updateLeadTimeChart(filteredData, startDate, endDate);
    updateFileTypeChart(startDate, endDate);
    updateFileTypeTrendChart(startDate, endDate);

    await yieldToMain();

    // 4. GitHub Specifics and Context Switching
    updateReviewActivityChart(startDate, endDate);
    updateGitHubAdvancedMetrics(startDate, endDate);
    updateContextSwitchChart(filteredData, startDate, endDate);
    updateContextSwitchTrendChart(filteredData, startDate, endDate);
    updateFragmentationChart(filteredData, startDate, endDate);

    await yieldToMain();

    // 5. Impact, Insights and Predictions
    updateImpactAssessment(undefined);
    updateUserList(filteredData);
    generateInsights(filteredData, startDate, endDate);
    updatePredictiveDashboard(filteredData);
    updateVelocitySizeChart(startDate, endDate);
}

const _debouncedUpdatePredictionOnly = debounce(() => {
    syncStateToUrl();
    updatePredictiveDashboard();
}, 100);

function updatePredictionOnly() {
    _debouncedUpdatePredictionOnly();
}

// Initialization and Event Listeners
document.addEventListener('DOMContentLoaded', () => {
    // Set default dates if available
    if (typeof allDates !== 'undefined' && allDates.length > 0) {
        if (!document.getElementById('startDate').value) {
            document.getElementById('startDate').value = allDates[0];
        }
        if (!document.getElementById('endDate').value) {
            document.getElementById('endDate').value = allDates[allDates.length - 1];
        }
    }

    loadStateFromUrl();
    renderUserCheckboxes();
    updateDashboard(true);

    // Setup other event listeners if they are not inline
    // (Most are inline in HTML: onchange="updateDashboard()")
});
