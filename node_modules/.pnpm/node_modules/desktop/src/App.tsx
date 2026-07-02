import { useEffect, useState } from 'react';
import { useConfigStore } from './store/configStore';
import { Layout } from './components/Layout';
import { General } from './pages/General';
import { Brightness } from './pages/Brightness';
import { Adaptive } from './pages/Adaptive';
import { Transition } from './pages/Transition';
import { Performance } from './pages/Performance';
import { Developer } from './pages/Developer';
import { OverviewView } from './overview/OverviewView';
import { About } from './pages/About';
import { WizardFlow } from './wizard/WizardFlow';

function App() {
  const { loadConfig, isLoading, error, config } = useConfigStore();
  const [activeTab, setActiveTab] = useState('overview');

  useEffect(() => {
    loadConfig();
  }, [loadConfig]);

  useEffect(() => {
    if (!isLoading && config) {
      // Apply theme
      const root = document.documentElement;
      if (config.appearance.theme === 'Dark') {
        root.setAttribute('data-theme', 'Dark');
      } else if (config.appearance.theme === 'Light') {
        root.removeAttribute('data-theme');
      } else {
        // System preference
        if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
          root.setAttribute('data-theme', 'Dark');
        } else {
          root.removeAttribute('data-theme');
        }
      }
    }
  }, [config, isLoading]);

  if (isLoading) {
    return <div className="loading">Loading PixelSense Configuration...</div>;
  }

  if (error) {
    return <div className="loading" style={{ color: 'var(--danger-color)' }}>Error: {error}</div>;
  }

  const renderContent = () => {
    switch (activeTab) {
      case 'general': return <General />;
      case 'brightness': return <Brightness />;
      case 'adaptive': return <Adaptive />;
      case 'transition': return <Transition />;
      case 'performance': return <Performance />;
      case 'overview': return <OverviewView />;
      case 'developer': return <Developer />;
      case 'about': return <About />;
      default: return <General />;
    }
  };

  return (
    <Layout activeTab={activeTab} onTabChange={setActiveTab}>
      {renderContent()}
    </Layout>
  );
}

export default App;


