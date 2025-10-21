import axios from 'axios';

const API_BASE_URL = 'http://localhost:18007/api';

const api = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

// 添加请求拦截器以包含认证令牌
api.interceptors.request.use((config) => {
  const token = localStorage.getItem('sessionToken');
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

export const authAPI = {
  login: (username, password, totpCode = null) =>
    api.post('/auth/login', { username, password, totp_code: totpCode }),
  logout: () => api.post('/auth/logout'),
  changePassword: (oldPassword, newPassword) =>
    api.post('/auth/change-password', { old_password: oldPassword, new_password: newPassword }),
  getStatus: () => api.get('/auth/status'),
};

export const totpAPI = {
  list: () => api.get('/totp/list'),
  add: (name, secret) => api.post('/totp/add', { name, secret }),
  delete: (id, confirmed = false) => api.post('/totp/delete', { id, confirmed }),
  getCode: (id) => api.post('/totp/get-code', { id }),
};

export const settingsAPI = {
  get2FAEnabled: () => api.get('/settings/2fa-enabled'),
  get2FASetup: () => api.get('/settings/2fa-setup'),
  enable2FA: (totpCode) => api.post('/settings/enable-2fa', { totp_code: totpCode }),
  disable2FA: () => api.post('/settings/disable-2fa'),
};

export default api;

