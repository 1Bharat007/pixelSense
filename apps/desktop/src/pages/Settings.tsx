import { Settings2, Eye, Battery, MonitorPlay } from "lucide-react";
import { Card } from "../components/ui/Card";
import { Switch } from "../components/ui/Switch";
import { useState } from "react";

export function Settings() {
  const [useContentAnalysis, setUseContentAnalysis] = useState(true);
  const [useSensorAssist, setUseSensorAssist] = useState(true);
  const [batterySaver, setBatterySaver] = useState(false);

  return (
    <div className="flex-1 p-10 overflow-y-auto w-full max-w-4xl mx-auto">
      <header className="mb-10">
        <h1 className="text-3xl font-semibold tracking-tight text-foreground flex items-center gap-3">
          <Settings2 className="w-8 h-8 text-primary" />
          Settings
        </h1>
        <p className="text-muted-foreground mt-2">Manage your visual comfort preferences.</p>
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
                  <h3 className="font-medium text-foreground text-lg">Screen Content Analysis</h3>
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
                  <h3 className="font-medium text-foreground text-lg">Hardware Sensor Assist</h3>
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
                  <h3 className="font-medium text-foreground text-lg">Fullscreen Media</h3>
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
