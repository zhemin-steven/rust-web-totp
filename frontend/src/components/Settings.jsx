import React, { useState, useEffect, useRef } from 'react';
import QRCodeStyling from 'qr-code-styling';
import { authAPI, settingsAPI } from '../api';
import '../styles/Settings.css';

export default function Settings({ onLogout }) {
  const [oldPassword, setOldPassword] = useState('');
  const [newPassword, setNewPassword] = useState('');
  const [confirmPassword, setConfirmPassword] = useState('');
  const [twoFAEnabled, setTwoFAEnabled] = useState(false);
  const [error, setError] = useState('');
  const [success, setSuccess] = useState('');
  const [loading, setLoading] = useState(false);
  const [showPasswordForm, setShowPasswordForm] = useState(false);
  const [show2FAModal, setShow2FAModal] = useState(false);
  const [twoFASecret, setTwoFASecret] = useState('');
  const [twoFAQRCode, setTwoFAQRCode] = useState('');
  const [twoFAVerificationCode, setTwoFAVerificationCode] = useState('');
  const [twoFAModalLoading, setTwoFAModalLoading] = useState(false);
  const qrCodeRef = useRef(null);

  useEffect(() => {
    loadSettings();
  }, []);

  // 生成二维码
  useEffect(() => {
    if (show2FAModal && twoFAQRCode && qrCodeRef.current) {
      const qrCode = new QRCodeStyling({
        width: 200,
        height: 200,
        data: twoFAQRCode,
        margin: 10,
        type: 'svg',
        dotsOptions: {
          color: '#000000',
          type: 'square',
        },
        backgroundOptions: {
          color: '#ffffff',
        },
      });

      // 清空之前的内容
      qrCodeRef.current.innerHTML = '';
      qrCode.append(qrCodeRef.current);
    }
  }, [show2FAModal, twoFAQRCode]);

  const loadSettings = async () => {
    try {
      const response = await settingsAPI.get2FAEnabled();
      if (response.data.success) {
        setTwoFAEnabled(response.data.enabled);
      }
    } catch (err) {
      console.error('Failed to load settings:', err);
    }
  };

  const handleChangePassword = async (e) => {
    e.preventDefault();
    setError('');
    setSuccess('');

    if (!oldPassword || !newPassword || !confirmPassword) {
      setError('All fields are required');
      return;
    }

    if (newPassword !== confirmPassword) {
      setError('New passwords do not match');
      return;
    }

    if (newPassword.length < 6) {
      setError('New password must be at least 6 characters');
      return;
    }

    setLoading(true);

    try {
      const response = await authAPI.changePassword(oldPassword, newPassword);
      if (response.data.success) {
        setSuccess('Password changed successfully!');
        setOldPassword('');
        setNewPassword('');
        setConfirmPassword('');
        setTimeout(() => {
          setShowPasswordForm(false);
          setSuccess('');
        }, 1500);
      } else {
        setError(response.data.message || 'Failed to change password');
      }
    } catch (err) {
      setError(err.response?.data?.message || 'Failed to change password');
    } finally {
      setLoading(false);
    }
  };

  const handleToggle2FA = async () => {
    if (twoFAEnabled) {
      // 禁用2FA
      setLoading(true);
      try {
        const response = await settingsAPI.disable2FA();
        if (response.data.success) {
          setTwoFAEnabled(false);
          setSuccess('2FA disabled');
          setTimeout(() => setSuccess(''), 2000);
        } else {
          setError(response.data.message || 'Failed to disable 2FA');
        }
      } catch (err) {
        setError('Failed to disable 2FA');
      } finally {
        setLoading(false);
      }
    } else {
      // 启用2FA - 先获取设置信息
      setLoading(true);
      try {
        const response = await settingsAPI.get2FASetup();
        if (response.data.success) {
          setTwoFASecret(response.data.secret);
          setTwoFAQRCode(response.data.qr_code);
          setTwoFAVerificationCode('');
          setShow2FAModal(true);
        } else {
          setError(response.data.message || 'Failed to get 2FA setup information');
        }
      } catch (err) {
        setError(err.response?.data?.message || 'Failed to get 2FA setup information');
      } finally {
        setLoading(false);
      }
    }
  };

  const handleVerify2FA = async () => {
    if (!twoFAVerificationCode || twoFAVerificationCode.length !== 6) {
      setError('Please enter a valid 6-digit code');
      return;
    }

    setTwoFAModalLoading(true);
    try {
      const response = await settingsAPI.enable2FA(twoFAVerificationCode);
      if (response.data.success) {
        setTwoFAEnabled(true);
        setSuccess('2FA enabled successfully!');
        setShow2FAModal(false);
        setTwoFAVerificationCode('');
        setTimeout(() => setSuccess(''), 2000);
      } else {
        setError(response.data.message || 'Failed to enable 2FA');
      }
    } catch (err) {
      setError(err.response?.data?.message || 'Failed to enable 2FA');
    } finally {
      setTwoFAModalLoading(false);
    }
  };

  const handleLogout = async () => {
    try {
      await authAPI.logout();
    } catch (err) {
      console.error('Logout error:', err);
    }
    localStorage.removeItem('sessionToken');
    localStorage.removeItem('username');
    onLogout();
  };

  return (
    <div className="settings">
      <h2>Settings</h2>

      <div className="settings-section">
        <h3>Security</h3>

        <div className="setting-item">
          <div className="setting-info">
            <label>2FA Login Protection</label>
            <p>Require 2FA code when logging in</p>
          </div>
          <label className="toggle-switch">
            <input
              type="checkbox"
              checked={twoFAEnabled}
              onChange={handleToggle2FA}
              disabled={loading}
            />
            <span className="slider"></span>
          </label>
        </div>

        {!showPasswordForm ? (
          <button
            className="change-password-btn"
            onClick={() => setShowPasswordForm(true)}
          >
            Change Password
          </button>
        ) : (
          <form onSubmit={handleChangePassword} className="password-form">
            <div className="form-group">
              <label htmlFor="oldPassword">Current Password</label>
              <input
                id="oldPassword"
                type="password"
                value={oldPassword}
                onChange={(e) => setOldPassword(e.target.value)}
                placeholder="Enter current password"
                disabled={loading}
              />
            </div>

            <div className="form-group">
              <label htmlFor="newPassword">New Password</label>
              <input
                id="newPassword"
                type="password"
                value={newPassword}
                onChange={(e) => setNewPassword(e.target.value)}
                placeholder="Enter new password"
                disabled={loading}
              />
            </div>

            <div className="form-group">
              <label htmlFor="confirmPassword">Confirm New Password</label>
              <input
                id="confirmPassword"
                type="password"
                value={confirmPassword}
                onChange={(e) => setConfirmPassword(e.target.value)}
                placeholder="Confirm new password"
                disabled={loading}
              />
            </div>

            {error && <div className="error-message">{error}</div>}
            {success && <div className="success-message">{success}</div>}

            <div className="form-actions">
              <button
                type="button"
                className="cancel-btn"
                onClick={() => setShowPasswordForm(false)}
                disabled={loading}
              >
                Cancel
              </button>
              <button type="submit" disabled={loading} className="submit-btn">
                {loading ? 'Updating...' : 'Update Password'}
              </button>
            </div>
          </form>
        )}
      </div>

      <div className="settings-section">
        <h3>Account</h3>
        <button className="logout-btn" onClick={handleLogout}>
          Logout
        </button>
      </div>

      {/* 2FA Setup Modal */}
      {show2FAModal && (
        <div className="modal-overlay" onClick={() => setShow2FAModal(false)}>
          <div className="modal-content" onClick={(e) => e.stopPropagation()}>
            <div className="modal-header">
              <h2>2FA Login Protection Enabled</h2>
              <button
                className="modal-close"
                onClick={() => setShow2FAModal(false)}
              >
                ×
              </button>
            </div>

            <div className="modal-body">
              <p className="modal-warning">
                ⚠️ Please save your secret key and scan the QR code with your authenticator app (Google Authenticator, Authy, etc.)
              </p>

              <div className="qr-section">
                <div className="qr-code-container" ref={qrCodeRef}></div>
              </div>

              <div className="secret-section">
                <label>Secret Key (Manual Entry):</label>
                <div className="secret-display">
                  <code>{twoFASecret}</code>
                  <button
                    className="copy-btn"
                    onClick={() => {
                      navigator.clipboard.writeText(twoFASecret);
                      alert('Secret key copied to clipboard!');
                    }}
                  >
                    Copy
                  </button>
                </div>
              </div>

              <div className="modal-instructions">
                <h3>Instructions:</h3>
                <ol>
                  <li>Open your authenticator app (Google Authenticator, Authy, Microsoft Authenticator, etc.)</li>
                  <li>Scan the QR code above, or manually enter the secret key</li>
                  <li>Your authenticator app will generate a 6-digit code</li>
                  <li>Enter the code below to verify and enable 2FA</li>
                </ol>
              </div>

              <div className="verification-section">
                <label htmlFor="verificationCode">Verification Code:</label>
                <input
                  id="verificationCode"
                  type="text"
                  value={twoFAVerificationCode}
                  onChange={(e) => setTwoFAVerificationCode(e.target.value.replace(/\D/g, '').slice(0, 6))}
                  placeholder="Enter 6-digit code"
                  maxLength="6"
                  disabled={twoFAModalLoading}
                  autoFocus
                />
                <p className="hint">Enter the 6-digit code from your authenticator app</p>
              </div>

              {error && <div className="error-message">{error}</div>}
            </div>

            <div className="modal-footer">
              <button
                className="modal-btn-secondary"
                onClick={() => {
                  setShow2FAModal(false);
                  setTwoFAVerificationCode('');
                  setError('');
                }}
                disabled={twoFAModalLoading}
              >
                Cancel
              </button>
              <button
                className="modal-btn-primary"
                onClick={handleVerify2FA}
                disabled={twoFAModalLoading || twoFAVerificationCode.length !== 6}
              >
                {twoFAModalLoading ? 'Verifying...' : 'Verify & Enable 2FA'}
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

