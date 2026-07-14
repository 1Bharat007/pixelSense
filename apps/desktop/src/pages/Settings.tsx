import { Settings2, Zap, Power, MonitorPlay, Code2, Sun, Moon } from "lucide-react";
import { Card } from "../components/ui/Card";
import { Switch } from "../components/ui/Switch";
import { Tooltip } from "../components/ui/Tooltip";
import { useStore } from "../store/useStore";
import { invoke } from "@tauri-apps/api/core";
import { useState, useEffect } from "react";

export function Settings() {
  const { developerMode, setDeveloperMode } = useStore();
  
  const [startWithWindows, setStartWithWindows] = useState(true);
  const [runInBackground, setRunInBackground] = useState(true);
  const [smoothTransitions, setSmoothTransitions] = useState(true);
  const [minBrightness, setMinBrightness] = useState(10);
  const [maxBrightness, setMaxBrightness] = useState(100);
  const [statusMessage, setStatusMessage] = useState("");

  useEffect(() => {
    invoke<any>("get_config").then(cfg => {
      setStartWithWindows(cfg?.system?.start_with_windows ?? true);
      setRunInBackground(cfg?.system?.run_in_background ?? true);
      setSmoothTransitions(cfg?.adaptive?.smooth_transitions ?? true);
      setMinBrightness(cfg?.brightness?.min_brightness ?? 10);
      setMaxBrightness(cfg?.brightness?.max_brightness ?? 100);
    }).catch(console.error);
  }, []);

  const triggerStatus = (msg: string) => {
    setStatusMessage(msg);
    setTimeout(() => setStatusMessage(""), 3000);
  };

  const handleToggle = async (section: string, key: string, value: boolean, setter: (val: boolean) => void) => {
    setter(value);
    try {
      await invoke("save_config", { config: { [section]: { [key]: value } } });
      triggerStatus("Settings saved");
    } catch (e) {
      triggerStatus("Failed to save setting");
      setter(!value);
    }
  };

  const handleNumber = async (section: string, key: string, value: number, setter: (val: number) => void) => {
    setter(value);
    try {
      await invoke("save_config", { config: { [section]: { [key]: value } } });
    } catch (e) {
      console.error(e);
    }
  };

  return (
    <div className="flex-1 p-10 overflow-y-auto w-full max-w-4xl mx-auto">
      <header className="mb-10 flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-semibold tracking-tight text-foreground flex items-center gap-3">
            <Settings2 className="w-8 h-8 text-primary" />
            Settings
          </h1>
          <p className="text-muted-foreground mt-2">Manage your PixelSense preferences.</p>
        </div>
        
        {statusMessage && (
          <div className="bg-success/15 border border-success/30 text-success-foreground px-4 py-2 rounded-md font-medium text-sm animate-in fade-in slide-in-from-top-2">
            {statusMessage}
          </div>
        )}
      </header>

      <div className="flex flex-col gap-8">
        
        {/* System Behavior */}
        <section>
          <h2 className="text-sm font-semibold tracking-wider text-muted-foreground uppercase mb-4 pl-1">System</h2>
          <Card className="flex flex-col gap-6 p-6 bg-card/50">
            <div className="flex items-center justify-between">
              <div className="flex gap-4">
                <div className="mt-1"><Power className="w-5 h-5 text-muted-foreground" /></div>
                <div>
                  <h3 className="font-medium text-foreground text-lg flex items-center gap-2">
                    Start automatically with Windows
                  </h3>
                  <p className="text-muted-foreground">Launch PixelSense silently in the system tray when you log in.</p>
                </div>
              </div>
              <Switch checked={startWithWindows} onCheckedChange={(v) => handleToggle("system", "start_with_windows", v, setStartWithWindows)} />
            </div>
            
            <div className="h-px w-full bg-border/50" />
            
            <div className="flex items-center justify-between">
              <div className="flex gap-4">
                <div className="mt-1"><MonitorPlay className="w-5 h-5 text-muted-foreground" /></div>
                <div>
                  <h3 className="font-medium text-foreground text-lg flex items-center gap-2">
                    Run in background
                  </h3>
                  <p className="text-muted-foreground">Keep the protection engine running when you close this window.</p>
                </div>
              </div>
              <Switch checked={runInBackground} onCheckedChange={(v) => handleToggle("system", "run_in_background", v, setRunInBackground)} />
            </div>
          </Card>
        </section>

        {/* Protection Engine */}
        <section>
          <h2 className="text-sm font-semibold tracking-wider text-muted-foreground uppercase mb-4 pl-1">Protection Engine</h2>
          <Card className="flex flex-col gap-6 p-6 bg-card/50">
            <div className="flex items-center justify-between">
              <div className="flex gap-4">
                <div className="mt-1"><Zap className="w-5 h-5 text-muted-foreground" /></div>
                <div>
                  <h3 className="font-medium text-foreground text-lg flex items-center gap-2">
                    Smooth transition speed
                    <Tooltip content="Affects automatic engine adjustments only. Manual slider is always instant." />
                  </h3>
                  <p className="text-muted-foreground">Fade brightness changes gradually so they are less distracting.</p>
                </div>
              </div>
              <Switch checked={smoothTransitions} onCheckedChange={(v) => handleToggle("adaptive", "smooth_transitions", v, setSmoothTransitions)} />
            </div>
            
            <div className="h-px w-full bg-border/50" />
            
            <div className="flex items-center justify-between">
              <div className="flex gap-4">
                <div className="mt-1"><Moon className="w-5 h-5 text-muted-foreground" /></div>
                <div>
                  <h3 className="font-medium text-foreground text-lg">Minimum brightness</h3>
                  <p className="text-muted-foreground text-sm">The lowest level PixelSense is allowed to drop to.</p>
                </div>
              </div>
              <div className="flex items-center gap-3">
                <span className="text-sm font-medium">{minBrightness}%</span>
                <input 
                  type="range" 
                  min="0" max="50" 
                  value={minBrightness} 
                  onChange={(e) => handleNumber("brightness", "min_brightness", parseInt(e.target.value), setMinBrightness)}
                  className="w-32 h-2 bg-secondary rounded-lg appearance-none cursor-pointer accent-primary"
                />
              </div>
            </div>

            <div className="flex items-center justify-between">
              <div className="flex gap-4">
                <div className="mt-1"><Sun className="w-5 h-5 text-muted-foreground" /></div>
                <div>
                  <h3 className="font-medium text-foreground text-lg">Maximum brightness</h3>
                  <p className="text-muted-foreground text-sm">The highest level PixelSense is allowed to reach.</p>
                </div>
              </div>
              <div className="flex items-center gap-3">
                <span className="text-sm font-medium">{maxBrightness}%</span>
                <input 
                  type="range" 
                  min="50" max="100" 
                  value={maxBrightness} 
                  onChange={(e) => handleNumber("brightness", "max_brightness", parseInt(e.target.value), setMaxBrightness)}
                  className="w-32 h-2 bg-secondary rounded-lg appearance-none cursor-pointer accent-primary"
                />
              </div>
            </div>
          </Card>
        </section>

        {/* Advanced Settings */}
        <section>
          <h2 className="text-sm font-semibold tracking-wider text-muted-foreground uppercase mb-4 pl-1">Advanced</h2>
          <Card className="flex flex-col gap-6 p-6 bg-card/50">
            <div className="flex items-center justify-between">
              <div className="flex gap-4">
                <div className="mt-1"><Code2 className="w-5 h-5 text-muted-foreground" /></div>
                <div>
                  <h3 className="font-medium text-foreground text-lg">Developer Mode</h3>
                  <p className="text-muted-foreground">Expose raw engine metrics, sensor values, and detailed logs.</p>
                </div>
              </div>
              <Switch 
                checked={developerMode} 
                onCheckedChange={(v) => {
                  setDeveloperMode(v);
                  triggerStatus(v ? "Developer Mode Enabled" : "Developer Mode Disabled");
                }} 
              />
            </div>
            
            <div className="h-px w-full bg-border/50" />
            
            <div className="flex items-center justify-between">
              <div className="flex gap-4">
                <div className="mt-1"><Zap className="w-5 h-5 text-muted-foreground" /></div>
                <div>
                  <h3 className="font-medium text-foreground text-lg">Test Brightness</h3>
                  <p className="text-muted-foreground">Briefly drops brightness to 20% to verify hardware control.</p>
                </div>
              </div>
              <button 
                onClick={async () => {
                  try {
                    await invoke("test_brightness");
                    triggerStatus("Hardware test complete");
                  } catch (e) {
                    triggerStatus("Test failed: hardware unsupported");
                  }
                }}
                className="px-4 py-2 bg-secondary hover:bg-secondary/80 text-foreground font-medium rounded-md transition-colors focus-visible:outline-ring"
              >
                Test Now
              </button>
            </div>
          </Card>
        </section>

      </div>
    </div>
  );
}
