import { Sun, Moon, Monitor, BatteryMedium } from 'lucide-react';
import { useStore } from './store/useStore';

function App() {
  const { comfort, health } = useStore();

  return (
    <div className="flex h-screen w-full bg-background text-foreground">
      {/* Sidebar Placeholder */}
      <aside className="w-64 border-r border-border bg-sidebar p-6 flex flex-col gap-4">
        <div className="flex items-center gap-2 text-primary font-semibold text-xl mb-6">
          <Monitor className="w-6 h-6 text-accent" />
          <span>PixelSense</span>
        </div>
        
        <nav className="flex flex-col gap-2">
          <button className="flex items-center gap-3 px-3 py-2 rounded-md bg-accent text-accent-foreground font-medium transition-colors">
            Overview
          </button>
          <button className="flex items-center gap-3 px-3 py-2 rounded-md hover:bg-secondary text-muted-foreground hover:text-foreground transition-colors">
            Preferences
          </button>
          <button className="flex items-center gap-3 px-3 py-2 rounded-md hover:bg-secondary text-muted-foreground hover:text-foreground transition-colors">
            Developer
          </button>
        </nav>
      </aside>

      {/* Main Content */}
      <main className="flex-1 p-10 overflow-y-auto">
        <header className="mb-10">
          <h1 className="text-3xl font-semibold tracking-tight">Overview</h1>
          <p className="text-muted-foreground mt-1">Real-time comfort analysis</p>
        </header>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          {/* Hero Card */}
          <section className="col-span-1 rounded-xl border border-border bg-card text-card-foreground p-6 shadow-sm flex flex-col gap-4">
            <h2 className="text-lg font-medium flex items-center justify-between">
              Current Comfort
              <span className="px-2.5 py-0.5 rounded-full text-xs font-medium bg-green-500/10 text-green-500 border border-green-500/20">
                {comfort.status}
              </span>
            </h2>
            <div className="flex flex-col gap-1">
              <span className="text-sm text-muted-foreground">Recommendation</span>
              <p className="text-xl font-medium">{comfort.recommendation}</p>
            </div>
            <div className="flex flex-col gap-1 mt-auto">
              <span className="text-sm text-muted-foreground">Engine Confidence</span>
              <div className="w-full h-2 bg-secondary rounded-full overflow-hidden mt-1">
                <div 
                  className="h-full bg-accent rounded-full transition-all duration-1000" 
                  style={{ width: `${comfort.confidence * 100}%` }}
                />
              </div>
            </div>
          </section>

          {/* System Health Card */}
          <section className="col-span-1 rounded-xl border border-border bg-card text-card-foreground p-6 shadow-sm flex flex-col gap-4">
            <h2 className="text-lg font-medium">System Health</h2>
            
            <div className="flex flex-col gap-3">
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-2">
                  <Sun className="w-4 h-4 text-muted-foreground" />
                  <span className="text-sm">Native Sensor</span>
                </div>
                <span className={`text-sm font-medium ${health.nativeSensorActive ? 'text-green-500' : 'text-muted-foreground'}`}>
                  {health.nativeSensorActive ? 'Active' : 'Offline'}
                </span>
              </div>
              
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-2">
                  <Monitor className="w-4 h-4 text-muted-foreground" />
                  <span className="text-sm">Screen Engine</span>
                </div>
                <span className={`text-sm font-medium ${health.screenEngineActive ? 'text-green-500' : 'text-muted-foreground'}`}>
                  {health.screenEngineActive ? 'Active' : 'Offline'}
                </span>
              </div>

              <div className="flex items-center justify-between">
                <div className="flex items-center gap-2">
                  <BatteryMedium className="w-4 h-4 text-muted-foreground" />
                  <span className="text-sm">Performance Mode</span>
                </div>
                <span className="text-sm font-medium text-foreground">
                  {health.performanceMode}
                </span>
              </div>
            </div>
          </section>
        </div>
      </main>
    </div>
  );
}

export default App;
