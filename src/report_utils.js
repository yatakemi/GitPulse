
// Normal Distribution CDF approximation
function normalCDF(x, mean, std) {
    if (std === 0) return x >= mean ? 1.0 : 0.0;
    const z = (x - mean) / std;
    const t = 1.0 / (1.0 + 0.2316419 * Math.abs(z));
    const d = 0.398942280401432678 * Math.exp(-z * z / 2);
    let p = d * t * (0.319381530 + t * (-0.356563782 + t * (1.781477937 + t * (-1.821255978 + t * 1.330274429))));
    if (z > 0) p = 1.0 - p;
    return p;
}

function t(key) {
    const keys = key.split('.');
    let res = translations[currentLang];
    for (const k of keys) {
        res = res[k];
        if (res === undefined) return key;
    }
    return res;
}

function updateLanguage(lang) {
    currentLang = lang;
    document.getElementById('langSelect').value = lang;
    applyTranslations();
    updateDashboard();
}

function applyTranslations() {
    document.querySelectorAll('[data-i18n]').forEach(el => {
        const key = el.getAttribute('data-i18n');
        el.textContent = t(key);
    });
    document.querySelectorAll('[data-i18n-tooltip]').forEach(el => {
        const key = el.getAttribute('data-i18n-tooltip');
        el.setAttribute('data-tooltip', t(key));
    });
    // Re-render components that have text generated in JS
    updateDashboard();
}

function normalizeAuthor(name) {
    if (typeof aliases !== 'undefined' && aliases[name]) return aliases[name];

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

function getCanvasCtx(id) {
    const el = document.getElementById(id);
    if (!el) {
        // Suppress warning if chart is legitimately hidden or missing
        // console.warn('Canvas element not found:', id);
        return null;
    }
    return el.getContext('2d');
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

function renderUserCheckboxes() {
    const container = document.getElementById('userCheckboxes');
    if (!container) return;

    container.innerHTML = '';

    allUsers.forEach(user => {
        const label = document.createElement('label');
        label.style.display = 'block';

        const cb = document.createElement('input');
        cb.type = 'checkbox';
        cb.className = 'user-checkbox';
        cb.value = user;
        cb.checked = selectedUsers.has(user);
        cb.onchange = (e) => {
            if (e.target.checked) selectedUsers.add(user);
            else selectedUsers.delete(user);
            updateDashboard();
        };

        label.appendChild(document.createTextNode(' ' + user));
        container.appendChild(label);
    });
}

/**
 * Standard debounce function to limit the rate at which a function can fire.
 */
function debounce(func, wait) {
    let timeout;
    return function executedFunction(...args) {
        const later = () => {
            clearTimeout(timeout);
            func(...args);
        };
        clearTimeout(timeout);
        timeout = setTimeout(later, wait);
    };
}

/**
 * Helper to yield execution back to the main thread, allowing the UI to remain responsive.
 */
function yieldToMain() {
    return new Promise(resolve => setTimeout(resolve, 0));
}
