import React, { useState, useEffect } from 'react';
import { totpAPI } from '../api';
import '../styles/TOTPList.css';

export default function TOTPList() {
  const [entries, setEntries] = useState([]);
  const [codes, setCodes] = useState({});
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState('');
  const [deleteConfirm, setDeleteConfirm] = useState(null);

  useEffect(() => {
    loadEntries();
    const interval = setInterval(loadEntries, 1000);
    return () => clearInterval(interval);
  }, []);

  const loadEntries = async () => {
    try {
      const response = await totpAPI.list();
      if (response.data.success) {
        setEntries(response.data.entries);
        // 获取所有条目的代码
        response.data.entries.forEach((entry) => {
          getCode(entry.id);
        });
      }
    } catch (err) {
      setError('Failed to load TOTP entries');
    } finally {
      setLoading(false);
    }
  };

  const getCode = async (id) => {
    try {
      const response = await totpAPI.getCode(id);
      if (response.data.success) {
        setCodes((prev) => ({
          ...prev,
          [id]: {
            code: response.data.code,
            remaining: response.data.remaining_seconds,
          },
        }));
      }
    } catch (err) {
      console.error('Failed to get code:', err);
    }
  };

  const handleDelete = async (id) => {
    if (deleteConfirm !== id) {
      setDeleteConfirm(id);
      return;
    }

    try {
      const response = await totpAPI.delete(id, true);
      if (response.data.success) {
        setEntries(entries.filter((e) => e.id !== id));
        setDeleteConfirm(null);
      } else {
        setError(response.data.message || 'Failed to delete entry');
      }
    } catch (err) {
      setError('Failed to delete entry');
    }
  };

  const copyToClipboard = (code) => {
    navigator.clipboard.writeText(code);
  };

  if (loading) {
    return <div className="totp-list"><p>Loading...</p></div>;
  }

  return (
    <div className="totp-list">
      <h2>2FA Codes</h2>
      
      {error && <div className="error-message">{error}</div>}

      {entries.length === 0 ? (
        <p className="empty-message">No 2FA entries yet. Add one to get started!</p>
      ) : (
        <div className="entries-grid">
          {entries.map((entry) => {
            const codeData = codes[entry.id];
            const code = codeData?.code || '------';
            const remaining = codeData?.remaining || 0;
            const progress = (remaining / 30) * 100;

            return (
              <div key={entry.id} className="entry-card">
                <div className="entry-header">
                  <h3>{entry.name}</h3>
                  <button
                    className={`delete-btn ${deleteConfirm === entry.id ? 'confirm' : ''}`}
                    onClick={() => handleDelete(entry.id)}
                  >
                    {deleteConfirm === entry.id ? 'Confirm Delete?' : '×'}
                  </button>
                </div>

                <div className="code-display">
                  <div className="code" onClick={() => copyToClipboard(code)}>
                    {code}
                  </div>
                  <p className="copy-hint">Click to copy</p>
                </div>

                <div className="progress-bar">
                  <div className="progress" style={{ width: `${progress}%` }}></div>
                </div>
                <p className="remaining">{remaining}s</p>

                <p className="created-at">
                  Added: {new Date(entry.created_at).toLocaleDateString()}
                </p>
              </div>
            );
          })}
        </div>
      )}
    </div>
  );
}

