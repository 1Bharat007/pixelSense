import { useStore } from './store/useStore';
import { useDashboard } from './hooks/useDashboard';
import { Overview } from './pages/Overview';
import { Developer } from './pages/Developer';
import { Settings } from './pages/Settings';
import { About } from './pages/About';
import { Diagnostics } from './pages/Diagnostics';
import { cn } from './lib/utils';
import { Monitor, LayoutDashboard, Settings2, Code2, Info as InfoIcon, Activity } from 'lucide-react';
import { getVersion } from '@tauri-apps/api/app';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { useState, useEffect } from 'react';

const NAV_ITEMS = [
  { id: 'Overview', label: 'Overview', icon: LayoutDashboard },
  { id: 'Settings', label: 'Settings', icon: Settings2 },
  { id: 'Developer', label: 'Developer', icon: Code2 },
  { id: 'Diagnostics', label: 'Diagnostics', icon: Activity },
  { id: 'About', label: 'About', icon: InfoIcon },
];

function App() {
  useDashboard(500);

  const { activeTab, setActiveTab, developerMode } = useStore();
  const [appVersion, setAppVersion] = useState("v1.0.0");

  useEffect(() => {
    getVersion().then(version => setAppVersion(`v${version}`)).catch(console.error);
    getCurrentWindow().show().catch(console.error);
  }, []);

  return (
    <div className="flex min-h-screen w-full bg-background text-foreground overflow-x-hidden">
      
      {/* Sidebar Navigation */}
      <aside className="w-[240px] border-r border-border bg-sidebar flex flex-col pt-6 pb-4 shadow-sm z-10">
        <div className="flex items-center gap-3 px-6 mb-8 text-sidebar-primary">
          <Monitor className="w-6 h-6 text-accent" />
          <span className="font-semibold text-lg tracking-tight">PixelSense</span>
        </div>
        
        <nav aria-label="Main Navigation" className="flex flex-col gap-1 px-3" role="tablist">
          {NAV_ITEMS.map(item => {
            if ((item.id === 'Developer' || item.id === 'Diagnostics') && !developerMode) return null;
            
            const Icon = item.icon;
            const isActive = activeTab === item.id;
            
            return (
              <button
                key={item.id}
                role="tab"
                aria-selected={isActive}
                aria-controls={`panel-${item.id}`}
                id={`tab-${item.id}`}
                tabIndex={0}
                onClick={() => setActiveTab(item.id)}
                onKeyDown={(e) => {
                  if (e.key === 'Enter' || e.key === ' ') {
                    e.preventDefault();
                    setActiveTab(item.id);
                  }
                }}
                className={cn(
                  "flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm font-medium transition-all duration-200 focus-visible:outline-ring",
                  isActive 
                    ? "bg-accent text-accent-foreground shadow-sm" 
                    : "text-muted-foreground hover:bg-secondary/50 hover:text-foreground"
                )}
              >
                <Icon className={cn("w-4 h-4", isActive ? "text-accent-foreground" : "text-muted-foreground opacity-70")} aria-hidden="true" />
                {item.label}
              </button>
            )
          })}
        </nav>

        <div className="mt-auto px-6">
          <div className="text-xs text-muted-foreground/60 font-medium" aria-label={`App version ${appVersion}`}>{appVersion}</div>
        </div>
      </aside>

      {/* Main Content Area */}
      <main className="flex-1 bg-background relative overflow-hidden flex flex-col">
        {activeTab === 'Overview' && <Overview />}
        {activeTab === 'Settings' && <Settings />}
        {activeTab === 'Developer' && developerMode && <Developer />}
        {activeTab === 'Diagnostics' && developerMode && <Diagnostics />}
        {activeTab === 'About' && <About />}
      </main>
    </div>
  );
}

export default App;
