
let currentLang = 'en';
let _currentWeeklyMean, _currentWeeklyStdev, _currentVelocity;


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

// Canvas Contexts
// Note: getCanvasCtx is defined in report_utils.js which must be loaded before this file
// if we call it immediately. But wait, this file (globals) defines constants.
// If getCanvasCtx is in utils, and utils is loaded first, this works.
const ctx = getCanvasCtx('productivityChart');
const pieCtx = getCanvasCtx('shareChart');
const fileTypeCtx = getCanvasCtx('fileTypeChart');
const dowCtx = getCanvasCtx('dayOfWeekChart');
const heatmapCtx = getCanvasCtx('heatmapChart');
const sizeCtx = getCanvasCtx('sizeDistChart');
const durCtx = getCanvasCtx('workDurationChart');
const healthCtx = getCanvasCtx('healthTrendChart');
const ownerCtx = getCanvasCtx('ownershipChart');
const leadCtx = getCanvasCtx('leadTimeChart');
const leadTimeTrendCtx = getCanvasCtx('leadTimeTrendChart');
const fileTypeTrendCtx = getCanvasCtx('fileTypeTrendChart');
const velocitySizeCtx = getCanvasCtx('velocitySizeChart');
const reviewActivityCtx = getCanvasCtx('reviewActivityChart');
const reciprocityCtx = getCanvasCtx('reciprocityChart');
const scatterCtx = getCanvasCtx('scatterChart');
const resDistCtx = getCanvasCtx('resDistChart');
const leadDistCtx = getCanvasCtx('leadDistChart');
const ctxSwitchCtx = getCanvasCtx('ctxSwitchChart');
const fragmentationCtx = getCanvasCtx('fragmentationChart');
const ctxSwitchTrendCtx = getCanvasCtx('ctxSwitchTrendChart');
const forecastCtx = getCanvasCtx('forecastChart');

// Chart Instances
let mainChart, pieChart, fileTypeChart, dowChart, heatmapChart, sizeChart, durChart, healthChart, ownerChart, leadChart, leadTimeTrendChart, fileTypeTrendChart, velocitySizeChart, reviewActivityChart, reciprocityChart, scatterChart, resDistChart, leadDistChart, ctxChart, fragmentationChart, ctxSwitchTrendChart, forecastChart;

// User Data
const allUsers = [...new Set(data.map(d => d.author))].sort();
let selectedUsers = new Set(allUsers);
const allDates = [...new Set(data.map(d => d.dateStr))].sort();

let currentSort = { column: 'commits', direction: 'desc' };
