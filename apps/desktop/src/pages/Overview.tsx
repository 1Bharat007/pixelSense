import { ShieldCheck, AlertCircle, Activity, Sun, Lightbulb } from "lucide-react";
import { useStore } from "../store/useStore";
import { Card } from "../components/ui/Card";
import { invoke } from "@tauri-apps/api/core";
import { useState, useEffect } from "react";

export function Overview() {
  const { dashboard, error, clearError } = useStore();
  const [isProtectionEnabled, setIsProtectionEnabled] = useState(false);
  const [referenceBrightness, setReferenceBrightness] = useState(50);
  const [isStarting, setIsStarting] = useState(false);
  const [supportMessage, setSupportMessage] = useState("");
  
  // Try to load reference brightness on mount
  useEffect(() => {
    invoke<any>("get_config").then(cfg => {
      if (cfg?.brightness?.reference_brightness) {
        setReferenceBrightness(cfg.brightness.reference_brightness);
      }
      if (cfg?.adaptive?.enabled) {
        setIsProtectionEnabled(true);
      }
    }).catch(console.error);
  }, []);

  const handleSliderChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const val = parseInt(e.target.value);
    setReferenceBrightness(val);
    
    // Instantly push to hardware without delay or smoothing
    invoke("set_brightness_live", { level: val }).catch(console.error);
    
    // Save to config asynchronously
    invoke("save_config", { config: { brightness: { reference_brightness: val } } }).catch(console.error);
  };

  const toggleProtection = async () => {
    try {
      const newState = !isProtectionEnabled;
      setSupportMessage("");
      
      if (newState) {
          setIsStarting(true);
          
          // Wait for backend to detect capabilities and boot workers
          const capabilities = await invoke<any>("start_engine");
          
          if (!capabilities.supported) {
              setSupportMessage("Automatic brightness isn't supported on this display. You can still use manual brightness adjustment.");
              setIsStarting(false);
              return;
          }
          
          // Only save to config AFTER successful hardware verification
          await invoke("save_config", { 
              config: { adaptive: { enabled: true } } 
          });
          
          setIsStarting(false);
          setIsProtectionEnabled(true);
      } else {
          await invoke("save_config", { config: { adaptive: { enabled: false } } });
          await invoke("stop_engine");
          setIsProtectionEnabled(false);
      }
    } catch (e) {
      console.error("Failed to toggle protection", e);
      setSupportMessage("An unexpected error occurred while starting the engine.");
      setIsStarting(false);
    }
  };

  if (error) {
    return (
      <div className="flex-1 p-10 overflow-y-auto w-full max-w-2xl mx-auto flex flex-col items-center justify-center">
        <Card className="max-w-md w-full bg-destructive/5 border-destructive/20 items-center text-center p-8 flex flex-col">
          <div className="w-16 h-16 rounded-full bg-destructive/10 flex items-center justify-center text-destructive mb-4">
            <AlertCircle className="w-8 h-8" />
          </div>
          <h2 className="text-xl font-semibold text-foreground mb-2">{error.title}</h2>
          <p className="text-muted-foreground mb-6" aria-live="polite">
            {error.description}
          </p>
          <div className="flex w-full gap-3">
            {error.recoverable && (
              <button 
                onClick={clearError}
                className="flex-1 py-2 bg-primary hover:bg-primary/90 text-primary-foreground font-medium rounded-md transition-colors focus-visible:outline-ring"
              >
                Retry
              </button>
            )}
          </div>
        </Card>
      </div>
    );
  }

  // Derive Engine States from Backend Data
  const protectionState = dashboard?.comfort?.status || (isProtectionEnabled ? "Running" : "Disabled");
  const brightnessState = dashboard?.health?.transition_engine || (isProtectionEnabled ? "Adjusting" : "Paused");
  const ambientState = dashboard?.health?.ambient_engine || (isProtectionEnabled ? "Watching" : "Paused");

  return (
    <div className="flex-1 p-10 overflow-y-auto w-full max-w-3xl mx-auto flex flex-col justify-center min-h-full">
      <section className="mb-8 w-full flex flex-col items-center">
        <div className="w-16 h-16 rounded-full bg-primary/10 flex items-center justify-center text-primary mb-4">
          <ShieldCheck className="w-8 h-8" />
        </div>
        <h1 className="text-4xl font-semibold tracking-tight text-foreground mb-12">
          PixelSense
        </h1>
        
        <div className="w-full max-w-sm mb-12 flex flex-col gap-3">
          <div className="flex items-center justify-between text-sm font-medium text-foreground px-1">
            <span>Reference Brightness</span>
            <span className="text-muted-foreground">{referenceBrightness}%</span>
          </div>
          <input 
            type="range" 
            min="0" 
            max="100" 
            value={referenceBrightness}
            onChange={handleSliderChange}
            className="w-full h-2 bg-secondary rounded-lg appearance-none cursor-pointer accent-primary" 
          />
        </div>

        <button 
          onClick={toggleProtection}
          disabled={isStarting}
          className={`px-12 py-4 font-semibold rounded-full transition-all duration-300 text-lg focus-visible:outline-ring shadow-lg w-full max-w-sm mb-6 flex justify-center items-center gap-3 ${
            isStarting 
              ? 'bg-secondary text-secondary-foreground border border-border cursor-not-allowed opacity-80'
              : isProtectionEnabled 
                ? 'bg-primary hover:bg-primary/90 text-primary-foreground shadow-primary/20'
                : 'bg-transparent border-2 border-border text-foreground hover:bg-secondary/50'
          }`}
        >
          {isStarting && <Activity className="w-5 h-5 animate-spin" />}
          {!isStarting && isProtectionEnabled && (
            <div className="relative flex items-center justify-center">
               <ShieldCheck className="w-5 h-5 animate-in zoom-in spin-in-12 duration-500" />
            </div>
          )}
          {isStarting ? "Starting..." : isProtectionEnabled ? "Protection Enabled" : "Enable Protection"}
        </button>

        {supportMessage && (
          <p className="text-sm text-destructive font-medium max-w-sm text-center mb-6">
            {supportMessage}
          </p>
        )}

        {/* Live Engine Cards */}
        <div className="w-full max-w-2xl grid grid-cols-1 md:grid-cols-3 gap-4 mt-8">
          <Card className="p-4 flex items-start gap-4 bg-card/50 border-border/50">
            <div className="p-2 bg-primary/10 rounded-md text-primary mt-1">
              <ShieldCheck className="w-5 h-5" />
            </div>
            <div className="flex flex-col">
              <span className="text-xs font-semibold text-muted-foreground uppercase tracking-wider mb-1">Protection</span>
              <span className="text-sm font-medium text-foreground">{protectionState}</span>
            </div>
          </Card>

          <Card className="p-4 flex items-start gap-4 bg-card/50 border-border/50">
            <div className="p-2 bg-primary/10 rounded-md text-primary mt-1">
              <Sun className="w-5 h-5" />
            </div>
            <div className="flex flex-col">
              <span className="text-xs font-semibold text-muted-foreground uppercase tracking-wider mb-1">Display</span>
              <span className="text-sm font-medium text-foreground">{brightnessState}</span>
            </div>
          </Card>

          <Card className="p-4 flex items-start gap-4 bg-card/50 border-border/50">
            <div className="p-2 bg-primary/10 rounded-md text-primary mt-1">
              <Lightbulb className="w-5 h-5" />
            </div>
            <div className="flex flex-col">
              <span className="text-xs font-semibold text-muted-foreground uppercase tracking-wider mb-1">Environment</span>
              <span className="text-sm font-medium text-foreground">{ambientState}</span>
            </div>
          </Card>
        </div>
      </section>
    </div>
  );
}

