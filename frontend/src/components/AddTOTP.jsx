import React, { useState } from 'react';
import { totpAPI } from '../api';
import '../styles/AddTOTP.css';

export default function AddTOTP({ onSuccess }) {
  const [name, setName] = useState('');
  const [secret, setSecret] = useState('');
  const [error, setError] = useState('');
  const [success, setSuccess] = useState('');
  const [loading, setLoading] = useState(false);
  const [showForm, setShowForm] = useState(false);

  const handleSubmit = async (e) => {
    e.preventDefault();
    setError('');
    setSuccess('');
    setLoading(true);

    if (!name.trim()) {
      setError('Please enter a name');
      setLoading(false);
      return;
    }

    if (!secret.trim()) {
      setError('Please enter a secret key');
      setLoading(false);
      return;
    }

    try {
      const response = await totpAPI.add(name, secret);
      if (response.data.success) {
        setSuccess('2FA entry added successfully!');
        setName('');
        setSecret('');
        setTimeout(() => {
          setShowForm(false);
          setSuccess('');
          onSuccess();
        }, 1500);
      } else {
        setError(response.data.message || 'Failed to add entry');
      }
    } catch (err) {
      setError(err.response?.data?.message || 'Failed to add entry');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="add-totp">
      {!showForm ? (
        <button className="add-btn" onClick={() => setShowForm(true)}>
          + Add 2FA
        </button>
      ) : (
        <div className="form-container">
          <div className="form-header">
            <h3>Add New 2FA Entry</h3>
            <button className="close-btn" onClick={() => setShowForm(false)}>×</button>
          </div>

          <form onSubmit={handleSubmit}>
            <div className="form-group">
              <label htmlFor="name">Service Name</label>
              <input
                id="name"
                type="text"
                value={name}
                onChange={(e) => setName(e.target.value)}
                placeholder="e.g., Gmail, GitHub, AWS"
                disabled={loading}
              />
            </div>

            <div className="form-group">
              <label htmlFor="secret">Secret Key (Base32)</label>
              <textarea
                id="secret"
                value={secret}
                onChange={(e) => setSecret(e.target.value.toUpperCase())}
                placeholder="Paste your secret key here (usually from QR code)"
                disabled={loading}
                rows="4"
              />
              <p className="hint">
                You can usually find this in your account settings under "2FA" or "Security"
              </p>
            </div>

            {error && <div className="error-message">{error}</div>}
            {success && <div className="success-message">{success}</div>}

            <div className="form-actions">
              <button
                type="button"
                className="cancel-btn"
                onClick={() => setShowForm(false)}
                disabled={loading}
              >
                Cancel
              </button>
              <button type="submit" disabled={loading} className="submit-btn">
                {loading ? 'Adding...' : 'Add Entry'}
              </button>
            </div>
          </form>
        </div>
      )}
    </div>
  );
}

