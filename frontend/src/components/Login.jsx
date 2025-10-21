import React, { useState } from 'react';
import { authAPI } from '../api';
import '../styles/Login.css';

export default function Login({ onLoginSuccess }) {
  const [username, setUsername] = useState('admin');
  const [password, setPassword] = useState('admin');
  const [totpCode, setTotpCode] = useState('');
  const [requires2FA, setRequires2FA] = useState(false);
  const [error, setError] = useState('');
  const [loading, setLoading] = useState(false);

  const handleLogin = async (e) => {
    e.preventDefault();
    setError('');
    setLoading(true);

    try {
      const response = await authAPI.login(username, password, totpCode || undefined);

      if (response.data.success) {
        localStorage.setItem('sessionToken', response.data.session_token);
        localStorage.setItem('username', username);
        onLoginSuccess();
      } else if (response.data.requires_2fa) {
        setRequires2FA(true);
        setTotpCode('');
        setError('');
      } else {
        setError(response.data.message || 'Login failed');
      }
    } catch (err) {
      // 处理错误响应
      const errorData = err.response?.data;

      if (errorData?.requires_2fa) {
        // 2FA 需要
        setRequires2FA(true);
        setTotpCode('');
        setError('');
      } else {
        setError(errorData?.message || 'Login failed');
      }
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="login-container">
      <div className="login-box">
        <h1>WebTOTP</h1>
        <p className="subtitle">2FA Code Generator</p>

        <form onSubmit={handleLogin}>
          <div className="form-group">
            <label htmlFor="username">Username</label>
            <input
              id="username"
              type="text"
              value={username}
              onChange={(e) => setUsername(e.target.value)}
              placeholder="Enter username"
              disabled={loading || requires2FA}
            />
          </div>

          <div className="form-group">
            <label htmlFor="password">Password</label>
            <input
              id="password"
              type="password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              placeholder="Enter password"
              disabled={loading || requires2FA}
            />
          </div>

          {requires2FA && (
            <div className="form-group">
              <label htmlFor="totp">2FA Code</label>
              <input
                id="totp"
                type="text"
                value={totpCode}
                onChange={(e) => setTotpCode(e.target.value.replace(/\D/g, '').slice(0, 6))}
                placeholder="Enter 6-digit code"
                maxLength="6"
                disabled={loading}
                autoFocus
              />
              <p className="hint">Enter the 6-digit code from your authenticator app</p>
            </div>
          )}

          {error && <div className="error-message">{error}</div>}

          <button type="submit" disabled={loading} className="login-btn">
            {loading ? 'Loading...' : requires2FA ? 'Verify' : 'Login'}
          </button>

          {requires2FA && (
            <button
              type="button"
              onClick={() => {
                setRequires2FA(false);
                setTotpCode('');
                setError('');
              }}
              disabled={loading}
              className="back-btn"
            >
              Back
            </button>
          )}
        </form>

        <div className="default-credentials">
          <p>Default credentials:</p>
          <p>Username: <strong>admin</strong></p>
          <p>Password: <strong>admin</strong></p>
        </div>
      </div>
    </div>
  );
}

