// Global state
let currentEntryIdToDelete = null;
let totpIntervals = {};
let isUnlocked = false;

// Page navigation
const unlockPage = document.getElementById('unlock-page');
const loginPage = document.getElementById('login-page');
const mainPage = document.getElementById('main-page');

// Check lock status and session on load
async function checkLockStatusAndSession() {
    try {
        const lockResponse = await fetch('/api/lock-status');
        const lockData = await lockResponse.json();
        
        if (lockData.locked) {
            showUnlockPage();
            return;
        }
        
        isUnlocked = true;
        const sessionResponse = await fetch('/api/check-session');
        const sessionData = await sessionResponse.json();
        
        if (sessionData.success) {
            showMainPage();
        } else {
            showLoginPage();
        }
    } catch (error) {
        console.error('Status check failed:', error);
        showUnlockPage();
    }
}

function showUnlockPage() {
    if (unlockPage) {
        unlockPage.style.display = 'flex';
    }
    if (loginPage) loginPage.style.display = 'none';
    if (mainPage) mainPage.style.display = 'none';
}

function showLoginPage() {
    if (unlockPage) unlockPage.style.display = 'none';
    loginPage.style.display = 'block';
    mainPage.style.display = 'none';
}

function showMainPage() {
    if (unlockPage) unlockPage.style.display = 'none';
    loginPage.style.display = 'none';
    mainPage.style.display = 'block';
    loadTotpEntries();
    load2FAStatus();
}

// Login functionality
const loginForm = document.getElementById('login-form');
const totpInputGroup = document.getElementById('totp-input-group');
const loginError = document.getElementById('login-error');
const usernameInput = document.getElementById('username');
const passwordInput = document.getElementById('password');

// Check if user has 2FA enabled
let lastCheckedUsername = '';
let checkTimeout = null;

async function checkUser2FA() {
    const username = usernameInput.value.trim();
    if (!username || username === lastCheckedUsername) return;
    
    lastCheckedUsername = username;
    
    try {
        const response = await fetch('/api/check-user-2fa', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ username })
        });
        
        const data = await response.json();
        
        if (data.requires_2fa) {
            totpInputGroup.style.display = 'block';
        } else {
            totpInputGroup.style.display = 'none';
            document.getElementById('totp-code').value = '';
        }
    } catch (error) {
        console.error('Failed to check 2FA status:', error);
    }
}

// Check on page load (for autofill)
window.addEventListener('load', () => {
    setTimeout(() => {
        if (usernameInput.value.trim()) {
            checkUser2FA();
        }
    }, 100);
});

// Check when user types in username (with debounce)
usernameInput.addEventListener('input', () => {
    clearTimeout(checkTimeout);
    checkTimeout = setTimeout(() => {
        checkUser2FA();
    }, 300); // 300ms debounce
});

// Check when username loses focus
usernameInput.addEventListener('blur', () => {
    checkUser2FA();
});

// Check when username changes (for autofill)
usernameInput.addEventListener('change', () => {
    checkUser2FA();
});

// Check when password field gets focus
passwordInput.addEventListener('focus', () => {
    if (usernameInput.value.trim()) {
        checkUser2FA();
    }
});

loginForm.addEventListener('submit', async (e) => {
    e.preventDefault();
    loginError.textContent = '';
    
    const username = usernameInput.value;
    const password = passwordInput.value;
    const totpCode = document.getElementById('totp-code').value || null;
    
    try {
        const response = await fetch('/api/login', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ username, password, totp_code: totpCode })
        });
        
        const data = await response.json();
        
        if (data.success) {
            showMainPage();
            totpInputGroup.style.display = 'none';
            document.getElementById('totp-code').value = '';
            lastCheckedUsername = '';
        } else if (data.requires_2fa) {
            totpInputGroup.style.display = 'block';
            loginError.textContent = data.message;
            document.getElementById('totp-code').focus();
        } else {
            loginError.textContent = data.message;
        }
    } catch (error) {
        loginError.textContent = '登录失败，请重试';
        console.error('Login error:', error);
    }
});

// Logout functionality
document.getElementById('logout-btn').addEventListener('click', async () => {
    try {
        await fetch('/api/logout', { method: 'POST' });
        // Clear all intervals
        Object.values(totpIntervals).forEach(interval => clearInterval(interval));
        totpIntervals = {};
        showLoginPage();
        loginForm.reset();
        totpInputGroup.style.display = 'none';
        loginError.textContent = '';
    } catch (error) {
        console.error('Logout error:', error);
    }
});

// Tab navigation
const navLinks = document.querySelectorAll('.nav-link');
const totpListTab = document.getElementById('totp-list-tab');
const settingsTab = document.getElementById('settings-tab');

navLinks.forEach(link => {
    link.addEventListener('click', (e) => {
        e.preventDefault();
        const tabName = link.dataset.tab;
        
        navLinks.forEach(l => l.classList.remove('active'));
        link.classList.add('active');
        
        if (tabName === 'totp-list') {
            totpListTab.classList.add('active');
            settingsTab.classList.remove('active');
        } else if (tabName === 'settings') {
            totpListTab.classList.remove('active');
            settingsTab.classList.add('active');
        }
    });
});

// TOTP Entry Management
const totpEntriesContainer = document.getElementById('totp-entries');
const emptyState = document.getElementById('empty-state');

async function loadTotpEntries() {
    try {
        const response = await fetch('/api/totp/list');
        const entries = await response.json();
        
        if (entries.length === 0) {
            emptyState.style.display = 'block';
            totpEntriesContainer.innerHTML = '';
        } else {
            emptyState.style.display = 'none';
            renderTotpEntries(entries);
        }
    } catch (error) {
        console.error('Failed to load TOTP entries:', error);
    }
}

function renderTotpEntries(entries) {
    totpEntriesContainer.innerHTML = entries.map(entry => `
        <div class="totp-card" data-id="${entry.id}">
            <div class="totp-header">
                <div class="totp-info">
                    <h3>${escapeHtml(entry.name)}</h3>
                    <span class="issuer">${escapeHtml(entry.issuer)}</span>
                </div>
                <div class="totp-actions">
                    <button class="icon-btn delete" onclick="showDeleteConfirm('${entry.id}')">🗑️</button>
                </div>
            </div>
            <div class="totp-code-display" onclick="copyCode('${entry.id}')" style="cursor: pointer;" title="点击复制">
                <div class="totp-code" id="code-${entry.id}">------</div>
                <div class="copy-hint" id="copy-hint-${entry.id}" style="display: none;">已复制!</div>
            </div>
            <div class="totp-timer">
                <div class="timer-bar">
                    <div class="timer-fill" id="timer-${entry.id}"></div>
                </div>
                <span id="seconds-${entry.id}">30s</span>
            </div>
        </div>
    `).join('');
    
    // Start generating codes for all entries
    entries.forEach(entry => {
        generateCode(entry.id);
        // Clear existing interval if any
        if (totpIntervals[entry.id]) {
            clearInterval(totpIntervals[entry.id]);
        }
        // Update every second
        totpIntervals[entry.id] = setInterval(() => generateCode(entry.id), 1000);
    });
}

// Copy TOTP code to clipboard
function copyCode(entryId) {
    const codeElement = document.getElementById(`code-${entryId}`);
    const copyHint = document.getElementById(`copy-hint-${entryId}`);
    const code = codeElement.textContent;
    
    if (code === '------') return;
    
    navigator.clipboard.writeText(code).then(() => {
        copyHint.style.display = 'block';
        setTimeout(() => {
            copyHint.style.display = 'none';
        }, 1500);
    }).catch(err => {
        console.error('Failed to copy:', err);
        alert('复制失败，请手动复制');
    });
}

async function generateCode(entryId) {
    try {
        const response = await fetch(`/api/totp/generate/${entryId}`);
        const data = await response.json();
        
        const codeElement = document.getElementById(`code-${entryId}`);
        const timerElement = document.getElementById(`timer-${entryId}`);
        const secondsElement = document.getElementById(`seconds-${entryId}`);
        
        if (codeElement && data.code) {
            codeElement.textContent = data.code;
            const percentage = (data.remaining_seconds / 30) * 100;
            timerElement.style.width = `${percentage}%`;
            secondsElement.textContent = `${data.remaining_seconds}s`;
            
            // Change color when time is running out
            if (data.remaining_seconds <= 5) {
                timerElement.style.background = 'var(--danger-color)';
            } else {
                timerElement.style.background = 'var(--success-color)';
            }
        }
    } catch (error) {
        console.error(`Failed to generate code for ${entryId}:`, error);
    }
}

// Add TOTP Modal
const addTotpModal = document.getElementById('add-totp-modal');
const addTotpBtn = document.getElementById('add-totp-btn');
const addTotpForm = document.getElementById('add-totp-form');

addTotpBtn.addEventListener('click', () => {
    addTotpModal.classList.add('show');
});

addTotpForm.addEventListener('submit', async (e) => {
    e.preventDefault();
    
    const name = document.getElementById('entry-name').value;
    const issuer = document.getElementById('entry-issuer').value;
    const secret = document.getElementById('entry-secret').value;
    
    try {
        const response = await fetch('/api/totp/add', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ name, issuer, secret })
        });
        
        if (response.ok) {
            addTotpModal.classList.remove('show');
            addTotpForm.reset();
            loadTotpEntries();
        } else {
            alert('添加失败，请检查密钥是否正确');
        }
    } catch (error) {
        console.error('Failed to add TOTP entry:', error);
        alert('添加失败，请重试');
    }
});

// Delete TOTP Entry
const deleteConfirmModal = document.getElementById('delete-confirm-modal');
const confirmDeleteBtn = document.getElementById('confirm-delete-btn');

function showDeleteConfirm(entryId) {
    currentEntryIdToDelete = entryId;
    deleteConfirmModal.classList.add('show');
}

confirmDeleteBtn.addEventListener('click', async () => {
    if (!currentEntryIdToDelete) return;
    
    try {
        const response = await fetch('/api/totp/delete', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ id: currentEntryIdToDelete })
        });
        
        const data = await response.json();
        
        if (data.success) {
            // Clear interval for this entry
            if (totpIntervals[currentEntryIdToDelete]) {
                clearInterval(totpIntervals[currentEntryIdToDelete]);
                delete totpIntervals[currentEntryIdToDelete];
            }
            
            deleteConfirmModal.classList.remove('show');
            currentEntryIdToDelete = null;
            loadTotpEntries();
        } else {
            alert('删除失败，请重试');
        }
    } catch (error) {
        console.error('Failed to delete TOTP entry:', error);
        alert('删除失败，请重试');
    }
});

// Change Password
const changePasswordForm = document.getElementById('change-password-form');

changePasswordForm.addEventListener('submit', async (e) => {
    e.preventDefault();
    
    const oldPassword = document.getElementById('old-password').value;
    const newPassword = document.getElementById('new-password').value;
    const confirmPassword = document.getElementById('confirm-password').value;
    
    if (newPassword !== confirmPassword) {
        alert('新密码和确认密码不匹配');
        return;
    }
    
    try {
        const response = await fetch('/api/change-password', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ old_password: oldPassword, new_password: newPassword })
        });
        
        const data = await response.json();
        
        if (data.success) {
            alert('密码修改成功');
            changePasswordForm.reset();
        } else {
            alert(data.message);
        }
    } catch (error) {
        console.error('Failed to change password:', error);
        alert('密码修改失败，请重试');
    }
});

// 2FA Settings
const toggle2FABtn = document.getElementById('toggle-2fa-btn');
const twoFAStatusText = document.getElementById('2fa-status-text');
const twoFASetupModal = document.getElementById('2fa-setup-modal');
const verify2FAForm = document.getElementById('verify-2fa-form');

let is2FAEnabled = false;

async function load2FAStatus() {
    try {
        const response = await fetch('/api/2fa-status');
        const data = await response.json();
        is2FAEnabled = data.enabled;
        update2FAStatus();
    } catch (error) {
        console.error('Failed to load 2FA status:', error);
    }
}

function update2FAStatus() {
    const statusValue = document.getElementById('2fa-status-value');
    
    if (is2FAEnabled) {
        // 2FA已启用
        if (statusValue) {
            statusValue.textContent = window.t ? window.t('enabled') : '已启用';
            statusValue.setAttribute('data-i18n', 'enabled');
        }
        toggle2FABtn.textContent = window.t ? window.t('disable_2fa') : '禁用 2FA';
        toggle2FABtn.setAttribute('data-i18n', 'disable_2fa');
        toggle2FABtn.classList.remove('btn-primary');
        toggle2FABtn.classList.add('btn-danger');
    } else {
        // 2FA未启用
        if (statusValue) {
            statusValue.textContent = window.t ? window.t('disabled') : '未启用';
            statusValue.setAttribute('data-i18n', 'disabled');
        }
        toggle2FABtn.textContent = window.t ? window.t('enable_2fa') : '启用 2FA';
        toggle2FABtn.setAttribute('data-i18n', 'enable_2fa');
        toggle2FABtn.classList.remove('btn-danger');
        toggle2FABtn.classList.add('btn-primary');
    }
}

toggle2FABtn.addEventListener('click', async () => {
    if (is2FAEnabled) {
        // Disable 2FA - show confirmation modal
        showDisable2FAModal();
    } else {
        // Enable 2FA - show setup modal
        try {
            const response = await fetch('/api/enable-2fa', { method: 'POST' });
            const data = await response.json();
            
            document.getElementById('2fa-qr-code').src = data.qr_code;
            document.getElementById('2fa-secret-display').value = data.secret;
            twoFASetupModal.classList.add('show');
        } catch (error) {
            console.error('Failed to enable 2FA:', error);
            alert('操作失败，请重试');
        }
    }
});

// Show disable 2FA confirmation modal
function showDisable2FAModal() {
    const modal = document.getElementById('disable-2fa-modal');
    modal.classList.add('show');
}

// Handle disable 2FA form submission
document.getElementById('disable-2fa-form').addEventListener('submit', async (e) => {
    e.preventDefault();
    
    const password = document.getElementById('disable-2fa-password').value;
    const code = document.getElementById('disable-2fa-code').value;
    
    try {
        const response = await fetch('/api/disable-2fa', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ password, code })
        });
        
        const data = await response.json();
        
        if (data.success) {
            is2FAEnabled = false;
            update2FAStatus();
            document.getElementById('disable-2fa-modal').classList.remove('show');
            document.getElementById('disable-2fa-form').reset();
            alert('两步验证已禁用');
        } else {
            alert(data.message);
        }
    } catch (error) {
        console.error('Failed to disable 2FA:', error);
        alert('操作失败，请重试');
    }
});

verify2FAForm.addEventListener('submit', async (e) => {
    e.preventDefault();
    
    const code = document.getElementById('verify-code').value;
    
    try {
        const response = await fetch('/api/verify-2fa', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ code })
        });
        
        const data = await response.json();
        
        if (data.success) {
            is2FAEnabled = true;
            update2FAStatus();
            twoFASetupModal.classList.remove('show');
            verify2FAForm.reset();
            alert('两步验证已启用');
        } else {
            alert(data.message);
        }
    } catch (error) {
        console.error('Failed to verify 2FA:', error);
        alert('验证失败，请重试');
    }
});

// Modal close handlers
document.querySelectorAll('.close-btn, .cancel-btn').forEach(btn => {
    btn.addEventListener('click', (e) => {
        e.preventDefault();
        const modal = btn.closest('.modal');
        if (modal) {
            modal.classList.remove('show');
        }
    });
});

// Close modal on outside click
document.querySelectorAll('.modal').forEach(modal => {
    modal.addEventListener('click', (e) => {
        if (e.target === modal) {
            modal.classList.remove('show');
        }
    });
});

// Utility function
function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

// Toggle language menu
window.toggleLangMenu = function(event) {
    event.stopPropagation();
    const menu = event.target.nextElementSibling;
    const allMenus = document.querySelectorAll('.lang-menu');
    
    // Close all other menus
    allMenus.forEach(m => {
        if (m !== menu) {
            m.classList.remove('show');
        }
    });
    
    // Toggle current menu
    menu.classList.toggle('show');
};

// Hide language menu
window.hideLangMenu = function() {
    document.querySelectorAll('.lang-menu').forEach(menu => {
        menu.classList.remove('show');
    });
};

// Close menu when clicking outside
document.addEventListener('click', function(event) {
    if (!event.target.closest('.language-selector')) {
        window.hideLangMenu();
    }
});

// Language switcher (defined globally for onclick)
window.setLanguage = function(lang) {
    console.log('Switching language to:', lang);
    
    // Update global language variable
    window.currentLanguage = lang;
    localStorage.setItem('language', lang);
    
    // Update page text
    if (typeof window.updatePageText === 'function') {
        window.updatePageText();
    }
    
    // Update all language option buttons
    document.querySelectorAll('.lang-option').forEach(btn => btn.classList.remove('active'));
    
    // Set active for all option sets
    const suffix = lang === 'zh-CN' ? 'zh' : 'en';
    const options = [
        document.getElementById('lang-opt-' + suffix),           // unlock page
        document.getElementById('lang-opt-' + suffix + '-login'), // login page
        document.getElementById('lang-opt-' + suffix + '-main')   // main page
    ];
    
    options.forEach(opt => {
        if (opt) {
            opt.classList.add('active');
        }
    });
    
    console.log('Language switched successfully');
};

// Unlock functionality
const unlockForm = document.getElementById('unlock-form');
if (unlockForm) {
    unlockForm.addEventListener('submit', async (e) => {
        e.preventDefault();
        const masterPassword = document.getElementById('master-password').value;
        const unlockError = document.getElementById('unlock-error');
        const unlockBtn = document.querySelector('#unlock-form button[type="submit"]');
        
        unlockError.textContent = '';
        unlockBtn.disabled = true;
        unlockBtn.textContent = window.t ? window.t('unlocking') : 'Unlocking...';
        
        try {
            const response = await fetch('/api/unlock', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ master_password: masterPassword })
            });
            
            const data = await response.json();
            
            if (data.success) {
                isUnlocked = true;
                showLoginPage();
            } else {
                unlockError.textContent = data.message || 'Invalid master password';
            }
        } catch (error) {
            unlockError.textContent = 'Unlock failed. Please try again.';
            console.error('Unlock error:', error);
        } finally {
            unlockBtn.disabled = false;
            unlockBtn.textContent = window.t('unlock_button');
        }
    });
}

// Initialize
checkLockStatusAndSession();

