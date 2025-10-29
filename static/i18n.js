// 国际化支持
const translations = {
    'zh-CN': {
        // 通用
        'app_title': 'Web TOTP',
        'app_subtitle': '安全的双因素认证管理工具',
        
        // 解锁页面
        'unlock_title': '数据库已加密',
        'unlock_hint': '请输入主密码解锁',
        'unlock_button': '解锁',
        'unlock_first_time': '💡 首次使用？输入一个新密码来创建数据库',
        'master_password': '主密码',
        'unlocking': '解锁中...',
        
        // 登录页面
        'username': '用户名',
        'password': '密码',
        'login_button': '登录',
        'totp_code': '2FA 验证码',
        
        // 主界面
        'totp_list': '2FA 列表',
        'settings': '设置',
        'logout': '退出',
        'add_totp': '+ 添加 2FA',
        'no_entries': '暂无 2FA 条目',
        'click_add': '点击上方"添加 2FA"按钮开始',
        
        // 添加 TOTP
        'add_totp_title': '添加 2FA',
        'entry_name': '名称',
        'entry_issuer': '发行者',
        'entry_secret': '密钥',
        'name_placeholder': '例如: Google',
        'issuer_placeholder': '例如: Google',
        'secret_placeholder': '输入密钥',
        'cancel': '取消',
        'add': '添加',
        
        // 设置
        'change_password_title': '修改密码',
        'old_password': '当前密码',
        'new_password': '新密码',
        'confirm_password': '确认新密码',
        'change_password_button': '更改密码',
        'two_factor_auth': '两步验证',
        'enable_2fa_desc': '为此工具启用两步验证以增强安全性',
        'status': '状态',
        'enabled': '已启用',
        'disabled': '未启用',
        'enable_2fa': '启用 2FA',
        'disable_2fa': '禁用 2FA',
        
        // 2FA 设置
        'enable_2fa_title': '启用两步验证',
        'scan_qr': '使用 Google Authenticator 或其他 2FA 应用扫描此二维码：',
        'manual_entry': '或手动输入密钥:',
        'verify_code': '输入验证码以确认:',
        'verify_and_enable': '验证并启用',
        
        // 禁用 2FA
        'disable_2fa_title': '禁用两步验证',
        'disable_2fa_desc': '禁用两步验证需要验证您的身份',
        'current_password': '当前密码',
        'current_2fa_code': '当前 2FA 验证码',
        'confirm_disable': '确认禁用',
        
        // 删除确认
        'delete_confirm_title': '确认删除',
        'delete_confirm_message': '确定要删除这个 2FA 条目吗？此操作无法撤销。',
        'delete': '删除',
        
        // 提示消息
        'copied': '已复制!',
        'click_to_copy': '点击复制',
        'seconds': '秒',
        
        // 错误消息
        'invalid_password': '密码错误',
        'invalid_2fa': '2FA 验证码错误',
        'login_failed': '登录失败，请重试',
        'unlock_failed': '解锁失败，请重试',
    },
    'en-US': {
        // Common
        'app_title': 'Web TOTP',
        'app_subtitle': 'Secure Two-Factor Authentication Manager',
        
        // Unlock page
        'unlock_title': 'Database Encrypted',
        'unlock_hint': 'Enter master password to unlock',
        'unlock_button': 'Unlock',
        'unlock_first_time': '💡 First time? Enter a new password to create database',
        'master_password': 'Master Password',
        'unlocking': 'Unlocking...',
        
        // Login page
        'username': 'Username',
        'password': 'Password',
        'login_button': 'Login',
        'totp_code': '2FA Code',
        
        // Main interface
        'totp_list': '2FA List',
        'settings': 'Settings',
        'logout': 'Logout',
        'add_totp': '+ Add 2FA',
        'no_entries': 'No 2FA entries',
        'click_add': 'Click "Add 2FA" button above to start',
        
        // Add TOTP
        'add_totp_title': 'Add 2FA',
        'entry_name': 'Name',
        'entry_issuer': 'Issuer',
        'entry_secret': 'Secret',
        'name_placeholder': 'e.g. Google',
        'issuer_placeholder': 'e.g. Google',
        'secret_placeholder': 'Enter secret key',
        'cancel': 'Cancel',
        'add': 'Add',
        
        // Settings
        'change_password_title': 'Change Password',
        'old_password': 'Current Password',
        'new_password': 'New Password',
        'confirm_password': 'Confirm Password',
        'change_password_button': 'Change Password',
        'two_factor_auth': 'Two-Factor Authentication',
        'enable_2fa_desc': 'Enable 2FA for this tool to enhance security',
        'status': 'Status',
        'enabled': 'Enabled',
        'disabled': 'Disabled',
        'enable_2fa': 'Enable 2FA',
        'disable_2fa': 'Disable 2FA',
        
        // 2FA Setup
        'enable_2fa_title': 'Enable Two-Factor Authentication',
        'scan_qr': 'Scan this QR code with Google Authenticator or other 2FA app:',
        'manual_entry': 'Or enter manually:',
        'verify_code': 'Enter code to confirm:',
        'verify_and_enable': 'Verify and Enable',
        
        // Disable 2FA
        'disable_2fa_title': 'Disable Two-Factor Authentication',
        'disable_2fa_desc': 'Disabling 2FA requires identity verification',
        'current_password': 'Current Password',
        'current_2fa_code': 'Current 2FA Code',
        'confirm_disable': 'Confirm Disable',
        
        // Delete confirmation
        'delete_confirm_title': 'Confirm Deletion',
        'delete_confirm_message': 'Are you sure you want to delete this 2FA entry? This cannot be undone.',
        'delete': 'Delete',
        
        // Messages
        'copied': 'Copied!',
        'click_to_copy': 'Click to copy',
        'seconds': 's',
        
        // Errors
        'invalid_password': 'Invalid password',
        'invalid_2fa': 'Invalid 2FA code',
        'login_failed': 'Login failed, please try again',
        'unlock_failed': 'Unlock failed, please try again',
    }
};

// 获取当前语言
function getCurrentLanguage() {
    const saved = localStorage.getItem('language');
    if (saved) return saved;
    
    const browserLang = navigator.language || navigator.userLanguage;
    if (browserLang.startsWith('zh')) return 'zh-CN';
    return 'en-US';
}

// 设置语言
function setLanguage(lang) {
    localStorage.setItem('language', lang);
    currentLanguage = lang;
    updatePageText();
}

// 当前语言（全局变量）
window.currentLanguage = getCurrentLanguage();

// 获取翻译文本（全局函数）
window.t = function(key) {
    return translations[window.currentLanguage][key] || key;
};

// 更新页面文本（全局函数）
window.updatePageText = function() {
    // 更新普通文本
    document.querySelectorAll('[data-i18n]').forEach(element => {
        const key = element.getAttribute('data-i18n');
        const text = window.t(key);
        
        if (element.tagName === 'INPUT' && element.hasAttribute('placeholder')) {
            element.placeholder = text;
        } else {
            element.textContent = text;
        }
    });
    
    // 更新 placeholder（使用 data-i18n-placeholder 属性）
    document.querySelectorAll('[data-i18n-placeholder]').forEach(element => {
        const key = element.getAttribute('data-i18n-placeholder');
        const text = window.t(key);
        element.placeholder = text;
    });
};

// 更新语言按钮状态（全局函数）
window.updateLanguageButtons = function() {
    const langOptions = document.querySelectorAll('.lang-option');
    if (langOptions.length > 0) {
        langOptions.forEach(opt => opt.classList.remove('active'));
        
        // Set active for all option sets
        const suffix = window.currentLanguage === 'zh-CN' ? 'zh' : 'en';
        const options = [
            document.getElementById('lang-opt-' + suffix),
            document.getElementById('lang-opt-' + suffix + '-login'),
            document.getElementById('lang-opt-' + suffix + '-main')
        ];
        
        options.forEach(opt => {
            if (opt) {
                opt.classList.add('active');
            }
        });
    }
};

// 立即执行
setTimeout(() => {
    if (typeof window.updatePageText === 'function') {
        window.updatePageText();
        window.updateLanguageButtons();
    }
}, 100);

