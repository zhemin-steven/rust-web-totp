import { useState, useEffect } from 'react';
import Login from './components/Login';
import TOTPList from './components/TOTPList';
import AddTOTP from './components/AddTOTP';
import Settings from './components/Settings';
import { authAPI } from './api';
import './App.css';

function App() {
  const [authenticated, setAuthenticated] = useState(false);
  const [activeTab, setActiveTab] = useState('list');
  const [loading, setLoading] = useState(true);
  const [refreshKey, setRefreshKey] = useState(0);

  useEffect(() => {
    checkAuth();
  }, []);

  const checkAuth = async () => {
    try {
      const response = await authAPI.getStatus();
      if (response.data.authenticated) {
        setAuthenticated(true);
      }
    } catch (err) {
      console.error('Auth check failed:', err);
    } finally {
      setLoading(false);
    }
  };

  const handleLoginSuccess = () => {
    setAuthenticated(true);
    setActiveTab('list');
  };

  const handleLogout = () => {
    setAuthenticated(false);
    setActiveTab('list');
  };

  const handleAddSuccess = () => {
    setRefreshKey((prev) => prev + 1);
  };

  if (loading) {
    return <div className="app loading">Loading...</div>;
  }

  if (!authenticated) {
    return <Login onLoginSuccess={handleLoginSuccess} />;
  }

  return (
    <div className="app">
      <header className="app-header">
        <h1>WebTOTP</h1>
        <nav className="tabs">
          <button
            className={`tab ${activeTab === 'list' ? 'active' : ''}`}
            onClick={() => setActiveTab('list')}
          >
            2FA Codes
          </button>
          <button
            className={`tab ${activeTab === 'settings' ? 'active' : ''}`}
            onClick={() => setActiveTab('settings')}
          >
            Settings
          </button>
        </nav>
      </header>

      <main className="app-main">
        {activeTab === 'list' && (
          <div className="tab-content">
            <AddTOTP onSuccess={handleAddSuccess} />
            <TOTPList key={refreshKey} />
          </div>
        )}

        {activeTab === 'settings' && (
          <div className="tab-content">
            <Settings onLogout={handleLogout} />
          </div>
        )}
      </main>
    </div>
  );
}

export default App;
