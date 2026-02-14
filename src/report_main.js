
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

function updateDashboard() {
    const startDate = document.getElementById('startDate').value;
    const endDate = document.getElementById('endDate').value;
    const metric = document.getElementById('metricSelect').value;
    const chartType = document.getElementById('chartTypeSelect').value;
    const showTrend = document.getElementById('showTrend').checked;

    syncStateToUrl();

    // Filter aggregated daily stats
    const filteredData = data.filter(d =>
        d.dateStr >= startDate && d.dateStr <= endDate && selectedUsers.has(d.author)
    );

    // 1. Update Charts
    updateSummary(filteredData, metric, startDate, endDate);
    updateTimelineChart(filteredData, metric, chartType, showTrend, startDate, endDate);
    updatePieChart(filteredData, metric);
    updateDayOfWeekChart(filteredData, metric);
    updateHeatmapChart(filteredData, metric);
    updateSizeDistChart(filteredData);
    updateWorkDurationChart(filteredData);
    updateHealthTrendChart(filteredData, startDate, endDate);
    updateOwnershipChart(filteredData, startDate, endDate);
    updateIsolatedFilesTable(filteredData);
    updateLeadTimeChart(filteredData, startDate, endDate);
    // updateLeadTimeTrendChart(startDate, endDate); // Removed/Merged into Timeline
    updateFileTypeChart(startDate, endDate); // Uses raw commits internally
    updateFileTypeTrendChart(startDate, endDate);
    updateVelocitySizeChart(startDate, endDate);
    updateReviewActivityChart(startDate, endDate);
    updateGitHubAdvancedMetrics(startDate, endDate);
    updateContextSwitchChart(filteredData, startDate, endDate);
    updateContextSwitchTrendChart(filteredData, startDate, endDate);
    updateFragmentationChart(filteredData, startDate, endDate);

    // 2. Update Impact Assessment (pass undefined to use current selection)
    updateImpactAssessment(undefined);

    // 3. Update Text-based Sections
    updateUserList(filteredData);
    generateInsights(filteredData, startDate, endDate);
    updatePredictiveDashboard(filteredData);

    renderUserCheckboxes(); // Re-render checkboxes to show selection state if needed
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
    updateDashboard();

    // Setup other event listeners if they are not inline
    // (Most are inline in HTML: onchange="updateDashboard()")
});
