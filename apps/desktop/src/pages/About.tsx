import { Info, Monitor, ShieldCheck, Clock, HardDrive, Shield, CheckCircle2, XCircle, Code2 } from "lucide-react";
import { Card } from "../components/ui/Card";
import { useStore } from "../store/useStore";
import { getVersion } from "@tauri-apps/api/app";
import { useState, useEffect } from "react";

// Global start time since module loaded
const START_TIME = Date.now();

export function About() {
  const { dashboard, hardwareCapabilities } = useStore();
  const [appVersion, setAppVersion] = useState("Loading...");
  const [runningMinutes, setRunningMinutes] = useState(0);

  useEffect(() => {
    getVersion().then(v => setAppVersion(`v${v}`)).catch(() => setAppVersion("Unknown"));
    
    const interval = setInterval(() => {
      setRunningMinutes(Math.floor((Date.now() - START_TIME) / 60000));
    }, 1000);
    return () => clearInterval(interval);
  }, []);

  const ramUsage = dashboard?.performance?.ram_usage_mb ? `${Math.round(dashboard.performance.ram_usage_mb)} MB` : "Watching...";
  const isProtected = dashboard?.comfort?.status === "Protection Enabled";
  
  const hasInternal = hardwareCapabilities?.internal_display ?? false;
  const hasBrightness = hardwareCapabilities?.brightness_api ? true : false;
  const hasSensor = hardwareCapabilities?.ambient_available ?? false;
  const hasExternal = false; // Primary display only for now

  return (
    <div className="flex-1 p-10 overflow-y-auto w-full max-w-3xl mx-auto">
      <header className="mb-10">
        <h1 className="text-3xl font-semibold tracking-tight text-foreground flex items-center gap-3">
          <Info className="w-8 h-8 text-primary" />
          About PixelSense
        </h1>
        <p className="text-muted-foreground mt-1">Vision, architecture, and live diagnostics.</p>
      </header>

      <div className="flex flex-col gap-8">
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
          <Card className="p-4 flex flex-col gap-1 bg-card/50 border-border/50">
            <span className="text-xs font-semibold text-muted-foreground uppercase tracking-wider">Version</span>
            <span className="text-lg font-medium flex items-center gap-2"><Monitor className="w-4 h-4 text-primary" /> {appVersion}</span>
          </Card>
          <Card className="p-4 flex flex-col gap-1 bg-card/50 border-border/50">
            <span className="text-xs font-semibold text-muted-foreground uppercase tracking-wider">Running</span>
            <span className="text-lg font-medium flex items-center gap-2"><Clock className="w-4 h-4 text-primary" /> {runningMinutes} minutes</span>
          </Card>
          <Card className="p-4 flex flex-col gap-1 bg-card/50 border-border/50">
            <span className="text-xs font-semibold text-muted-foreground uppercase tracking-wider">Memory</span>
            <span className="text-lg font-medium flex items-center gap-2"><HardDrive className="w-4 h-4 text-primary" /> {ramUsage}</span>
          </Card>
          <Card className="p-4 flex flex-col gap-1 bg-card/50 border-border/50">
            <span className="text-xs font-semibold text-muted-foreground uppercase tracking-wider">Protection</span>
            <span className="text-lg font-medium flex items-center gap-2">
              <Shield className={`w-4 h-4 ${isProtected ? 'text-primary' : 'text-muted-foreground'}`} /> 
              {isProtected ? "Enabled" : "Disabled"}
            </span>
          </Card>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          <Card className="flex flex-col gap-4 p-6 bg-card/50 border-border/50">
            <div className="flex items-center gap-2 mb-2">
               <Monitor className="w-5 h-5 text-primary" />
               <h3 className="font-semibold text-lg">Hardware Status</h3>
            </div>
            
            <ul className="flex flex-col gap-3">
              <li className="flex items-center justify-between text-sm">
                <span className="text-muted-foreground">Internal Display</span>
                <span className="font-medium flex items-center gap-1">
                  {hasInternal ? <><CheckCircle2 className="w-4 h-4 text-success" /> Detected</> : <><XCircle className="w-4 h-4 text-destructive" /> Not found</>}
                </span>
              </li>
              <li className="flex items-center justify-between text-sm">
                <span className="text-muted-foreground">Brightness Control</span>
                <span className="font-medium flex items-center gap-1">
                  {hasBrightness ? <><CheckCircle2 className="w-4 h-4 text-success" /> Available</> : <><XCircle className="w-4 h-4 text-destructive" /> Unsupported</>}
                </span>
              </li>
              <li className="flex items-center justify-between text-sm">
                <span className="text-muted-foreground">Ambient Sensor</span>
                <span className="font-medium flex items-center gap-1">
                  {hasSensor ? <><CheckCircle2 className="w-4 h-4 text-success" /> Detected</> : <><XCircle className="w-4 h-4 text-destructive" /> Not found</>}
                </span>
              </li>
              <li className="flex items-center justify-between text-sm">
                <span className="text-muted-foreground">External Display</span>
                <span className="font-medium flex items-center gap-1 text-muted-foreground">
                  {hasExternal ? <><CheckCircle2 className="w-4 h-4 text-success" /> Active</> : <><XCircle className="w-4 h-4" /> Not detected</>}
                </span>
              </li>
            </ul>
          </Card>

          <Card className="flex flex-col gap-4 p-6 bg-card/50 border-border/50">
            <div className="flex items-center gap-2 mb-2">
               <ShieldCheck className="w-5 h-5 text-primary" />
               <h3 className="font-semibold text-lg">Privacy First</h3>
            </div>
            
            <ul className="flex flex-col gap-3">
              <li className="flex items-center gap-2 text-sm font-medium">
                <CheckCircle2 className="w-4 h-4 text-success" /> No cloud connection
              </li>
              <li className="flex items-center gap-2 text-sm font-medium">
                <CheckCircle2 className="w-4 h-4 text-success" /> No accounts required
              </li>
              <li className="flex items-center gap-2 text-sm font-medium">
                <CheckCircle2 className="w-4 h-4 text-success" /> No telemetry or tracking
              </li>
              <li className="flex items-center gap-2 text-sm font-medium">
                <CheckCircle2 className="w-4 h-4 text-success" /> Everything runs on your computer
              </li>
            </ul>

            <div className="mt-auto pt-4 border-t border-border/50">
              <a 
                href="https://github.com/pixelsense/pixelsense" 
                target="_blank" 
                rel="noreferrer"
                className="inline-flex items-center gap-2 text-sm font-medium text-muted-foreground hover:text-primary transition-colors"
              >
                <Code2 className="w-4 h-4" /> Open Source on GitHub
              </a>
            </div>
          </Card>
        </div>
      </div>
    </div>
  );
}
