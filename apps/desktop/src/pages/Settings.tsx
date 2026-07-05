import { Settings2, Eye, Battery, MonitorPlay, Save, Download, RotateCcw, Play } from "lucide-react";
import { Card } from "../components/ui/Card";
import { Switch } from "../components/ui/Switch";
import { Tooltip } from "../components/ui/Tooltip";
import { useState } from "react";
import { useStore } from "../store/useStore";

export function Settings() {
  const { setOnboardingCompleted } = useStore();
  
  const [useContentAnalysis, setUseContentAnalysis] = useState(true);
  const [useSensorAssist, setUseSensorAssist] = useState(true);
  const [batterySaver, setBatterySaver] = useState(false);
  
  const [statusMessage, setStatusMessage] = useState("");

  const triggerStatus = (msg: string) => {
    setStatusMessage(msg);
    setTimeout(() => setStatusMessage(""), 3000);
  };

  return (
    <div className="flex-1 p-10 overflow-y-auto w-full max-w-4xl mx-auto">
      <header className="mb-10 flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-semibold tracking-tight text-foreground flex items-center gap-3">
            <Settings2 className="w-8 h-8 text-primary" />
            Settings
          </h1>
          <p className="text-muted-foreground mt-2">Manage your visual comfort preferences.</p>
        </div>
        
        {statusMessage && (
          <div className="bg-success/15 border border-success/30 text-success-foreground px-4 py-2 rounded-md font-medium text-sm animate-in fade-in slide-in-from-top-2">
            {statusMessage}
          </div>
        )}
      </header>

      <div className="flex flex-col gap-8">
        
        {/* Core Engine */}
        <section>
          <h2 className="text-sm font-semibold tracking-wider text-muted-foreground uppercase mb-4 pl-1">Intelligence</h2>
          <Card className="flex flex-col gap-6">
            <div className="flex items-center justify-between">
              <div className="flex gap-4">
                <div className="mt-1"><Eye className="w-5 h-5 text-muted-foreground" /></div>
                <div>
                  <h3 className="font-medium text-foreground text-lg flex items-center gap-2">
                    Screen Content Analysis
                    <Tooltip content="Continuously samples the colors on your screen to calculate required brightness compensation." />
                  </h3>
                  <p className="text-muted-foreground">Adjust display brightness dynamically based on the colors currently on your screen.</p>
                </div>
              </div>
              <Switch checked={useContentAnalysis} onCheckedChange={setUseContentAnalysis} />
            </div>
            
            <div className="h-px w-full bg-border/50" />
            
            <div className="flex items-center justify-between">
              <div className="flex gap-4">
                <div className="mt-1"><SunIcon /></div>
                <div>
                  <h3 className="font-medium text-foreground text-lg flex items-center gap-2">
                    Hardware Sensor Assist
                    <Tooltip content="Requires a supported internal light sensor (common on modern laptops)." />
                  </h3>
                  <p className="text-muted-foreground">Use your device's built-in ambient light sensor to refine brightness calculations.</p>
                </div>
              </div>
              <Switch checked={useSensorAssist} onCheckedChange={setUseSensorAssist} />
            </div>
          </Card>
        </section>

        {/* Behavior */}
        <section>
          <h2 className="text-sm font-semibold tracking-wider text-muted-foreground uppercase mb-4 pl-1">Behavior</h2>
          <Card className="flex flex-col gap-6">
            <div className="flex items-center justify-between">
              <div className="flex gap-4">
                <div className="mt-1"><MonitorPlay className="w-5 h-5 text-muted-foreground" /></div>
                <div>
                  <h3 className="font-medium text-foreground text-lg flex items-center gap-2">
                    Fullscreen Media
                    <Tooltip content="Prevents brightness changes from distracting you while watching videos or gaming." />
                  </h3>
                  <p className="text-muted-foreground">How PixelSense behaves when you are watching a movie or playing a game.</p>
                </div>
              </div>
              <select className="bg-input/50 border border-border text-foreground px-4 py-2 rounded-md outline-none focus-visible:ring-2 focus-visible:ring-ring cursor-pointer">
                <option>Pause dynamically (Recommended)</option>
                <option>Pause completely</option>
                <option>Continue adjusting</option>
              </select>
            </div>
            
            <div className="h-px w-full bg-border/50" />
            
            <div className="flex items-center justify-between">
              <div className="flex gap-4">
                <div className="mt-1"><Battery className="w-5 h-5 text-muted-foreground" /></div>
                <div>
                  <h3 className="font-medium text-foreground text-lg">Battery Saver</h3>
                  <p className="text-muted-foreground">Reduce background processing when your laptop is unplugged.</p>
                </div>
              </div>
              <Switch checked={batterySaver} onCheckedChange={setBatterySaver} />
            </div>
          </Card>
        </section>
        
        {/* Configuration Management */}
        <section>
          <h2 className="text-sm font-semibold tracking-wider text-muted-foreground uppercase mb-4 pl-1">System</h2>
          <Card className="flex flex-col gap-6">
            
            <div className="flex items-center justify-between">
              <div>
                <h3 className="font-medium text-foreground text-lg">Product Tour</h3>
                <p className="text-muted-foreground">Replay the PixelSense onboarding experience.</p>
              </div>
              <button 
                onClick={() => setOnboardingCompleted(false)}
                className="flex items-center gap-2 px-4 py-2 bg-secondary hover:bg-secondary/80 text-foreground font-medium rounded-md transition-colors focus-visible:outline-ring"
              >
                <Play className="w-4 h-4" /> Restart Tour
              </button>
            </div>

            <div className="h-px w-full bg-border/50" />

            <div className="flex items-center justify-between">
              <div>
                <h3 className="font-medium text-foreground text-lg">Configuration Backup</h3>
                <p className="text-muted-foreground">Save or restore your personal preferences.</p>
              </div>
              <div className="flex gap-2">
                <button 
                  onClick={() => triggerStatus("Settings exported successfully.")}
                  className="flex items-center gap-2 px-4 py-2 bg-secondary hover:bg-secondary/80 text-foreground font-medium rounded-md transition-colors focus-visible:outline-ring"
                >
                  <Download className="w-4 h-4" /> Export
                </button>
                <button 
                  onClick={() => triggerStatus("Settings imported successfully.")}
                  className="flex items-center gap-2 px-4 py-2 bg-secondary hover:bg-secondary/80 text-foreground font-medium rounded-md transition-colors focus-visible:outline-ring"
                >
                  <Save className="w-4 h-4" /> Import
                </button>
              </div>
            </div>

            <div className="h-px w-full bg-border/50" />
            
            <div className="flex items-center justify-between">
              <div>
                <h3 className="font-medium text-foreground text-lg text-destructive">Factory Reset</h3>
                <p className="text-muted-foreground">Revert all settings to their default values.</p>
              </div>
              <button 
                onClick={() => triggerStatus("Settings restored to defaults.")}
                className="flex items-center gap-2 px-4 py-2 bg-destructive/10 hover:bg-destructive/20 text-destructive font-medium rounded-md transition-colors focus-visible:outline-ring focus-visible:ring-destructive"
              >
                <RotateCcw className="w-4 h-4" /> Reset
              </button>
            </div>

          </Card>
        </section>

      </div>
    </div>
  );
}

function SunIcon() {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" className="text-muted-foreground">
      <circle cx="12" cy="12" r="4"/><path d="M12 2v2"/><path d="M12 20v2"/><path d="m4.93 4.93 1.41 1.41"/><path d="m17.66 17.66 1.41 1.41"/><path d="M2 12h2"/><path d="M20 12h2"/><path d="m6.34 17.66-1.41 1.41"/><path d="m19.07 4.93-1.41 1.41"/>
    </svg>
  )
}
