import { Monitor, LayoutDashboard, Settings2, Code2, Users, Info as InfoIcon } from 'lucide-react';
import { useStore } from './store/useStore';
import { useDashboard } from './hooks/useDashboard';
import { Overview } from './pages/Overview';
import { Developer } from './pages/Developer';
import { Settings } from './pages/Settings';
import { Profiles } from './pages/Profiles';
import { History } from './pages/History';
import { Notifications } from './pages/Notifications';
import { About } from './pages/About';
import { Onboarding } from './pages/Onboarding';
import { cn } from './lib/utils';
import { AnimatePresence } from 'framer-motion';

import { Monitor, LayoutDashboard, Settings2, Code2, Users, Info as InfoIcon, History as HistoryIcon, Bell } from 'lucide-react';

const NAV_ITEMS = [
  { id: 'Overview', label: 'Overview', icon: LayoutDashboard },
  { id: 'Profiles', label: 'Profiles', icon: Users },
  { id: 'History', label: 'History', icon: HistoryIcon },
  { id: 'Notifications', label: 'Notifications', icon: Bell },
  { id: 'Settings', label: 'Settings', icon: Settings2 },
  { id: 'Developer', label: 'Developer', icon: Code2 },
  { id: 'About', label: 'About', icon: InfoIcon },
];

function App() {
  // Start the background polling engine.
  // In production, this might be replaced/augmented by Tauri events.
  useDashboard(500);

  const { activeTab, setActiveTab, onboardingCompleted } = useStore();

  return (
    <div className="flex h-screen w-full bg-background text-foreground overflow-hidden">
      
      <AnimatePresence>
        {!onboardingCompleted && <Onboarding />}
      </AnimatePresence>
      {/* Sidebar Navigation */}
      <aside className="w-[240px] border-r border-border bg-sidebar flex flex-col pt-6 pb-4 shadow-sm z-10">
        <div className="flex items-center gap-3 px-6 mb-8 text-sidebar-primary">
          <Monitor className="w-6 h-6 text-accent" />
          <span className="font-semibold text-lg tracking-tight">PixelSense</span>
        </div>
        
        <nav className="flex flex-col gap-1 px-3">
          {NAV_ITEMS.map(item => {
            const Icon = item.icon;
            const isActive = activeTab === item.id;
            
            return (
              <button
                key={item.id}
                onClick={() => setActiveTab(item.id)}
                className={cn(
                  "flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm font-medium transition-all duration-200",
                  isActive 
                    ? "bg-accent text-accent-foreground shadow-sm" 
                    : "text-muted-foreground hover:bg-secondary/50 hover:text-foreground"
                )}
              >
                <Icon className={cn("w-4 h-4", isActive ? "text-accent-foreground" : "text-muted-foreground opacity-70")} />
                {item.label}
              </button>
            )
          })}
        </nav>

        <div className="mt-auto px-6">
          <div className="text-xs text-muted-foreground/60 font-medium">v1.0.0-beta</div>
        </div>
      </aside>

      {/* Main Content Area */}
      <main className="flex-1 bg-background relative overflow-hidden">
        {activeTab === 'Overview' && <Overview />}
        {activeTab === 'Profiles' && <Profiles />}
        {activeTab === 'History' && <History />}
        {activeTab === 'Notifications' && <Notifications />}
        {activeTab === 'Settings' && <Settings />}
        {activeTab === 'Developer' && <Developer />}
        {activeTab === 'About' && <About />}
      </main>
    </div>
  );
}

export default App;
