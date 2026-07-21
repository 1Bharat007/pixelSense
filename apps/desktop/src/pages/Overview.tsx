import { ShieldCheck, AlertCircle, Activity, Sun, Lightbulb } from "lucide-react";
import { useStore } from "../store/useStore";
import { Card } from "../components/ui/Card";
import { invoke } from "@tauri-apps/api/core";
import { useState, useEffect, useRef } from "react";
import { AreaChart, Area, LineChart, Line, ResponsiveContainer } from "recharts";

export function Overview() {
  const { dashboard, history, error, clearError } = useStore();
  const [isProtectionEnabled, setIsProtectionEnabled] = useState(false);
  const [referenceBrightness, setReferenceBrightness] = useState(50);
  const [isStarting, setIsStarting] = useState(false);
  const [showCalibration, setShowCalibration] = useState(false);
  const [calibRoom, setCalibRoom] = useState("Normal");
  const [supportMessage, setSupportMessage] = useState("");

  // Try to load reference brightness on mount
  useEffect(() => {
    invoke<any>("get_config").then(cfg => {
      if (cfg?.brightness?.comfort_profile?.reference_brightness) {
        setReferenceBrightness(cfg.brightness.comfort_profile.reference_brightness);
      }
      if (cfg?.adaptive?.enabled) {
        setIsProtectionEnabled(true);
      }
    }).catch(console.error);
  }, []);

  const isUpdatingRef = useRef(false);
  const pendingBrightnessRef = useRef<number | null>(null);

  const processBrightnessQueue = async () => {
    if (isUpdatingRef.current || pendingBrightnessRef.current === null) return;
    
    isUpdatingRef.current = true;
    const target = pendingBrightnessRef.current;
    pendingBrightnessRef.current = null;
    
    try {
      await invoke("set_brightness_live", { level: target });
      setSupportMessage(""); // Clear old error on success
    } catch(err: any) {
      setSupportMessage(err.toString());
    } finally {
      isUpdatingRef.current = false;
      if (pendingBrightnessRef.current !== null) {
        processBrightnessQueue();
      }
    }
  };

  const handleSliderChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const val = parseInt(e.target.value);
    setReferenceBrightness(val);
    
    pendingBrightnessRef.current = val;
    processBrightnessQueue();
  };

  const startEngine = async () => {
    setIsStarting(true);
    try {
      const capabilities = await invoke<any>("start_engine");
      if (!capabilities.supported) {
          setSupportMessage(capabilities.failure_reason ? `Hardware Error: ${capabilities.failure_reason}` : "Automatic brightness isn't supported on this display.");
          setIsStarting(false);
          return;
      }
      await invoke("save_config", { config: { adaptive: { enabled: true } } });
      
      // Simulate 2-second startup animation
      setTimeout(() => {
        setIsStarting(false);
        setIsProtectionEnabled(true);
      }, 2000);
      
    } catch(e) {
      console.error(e);
      setSupportMessage("Initialization Failed");
      setIsStarting(false);
    }
  };

  const toggleProtection = async () => {
    try {
      const newState = !isProtectionEnabled;
      setSupportMessage("");
      
      if (newState) {
          // Always ask for the comfort baseline once when enabling protection
          setShowCalibration(true);
      } else {
          await invoke("save_config", { config: { adaptive: { enabled: false } } });
          await invoke("stop_engine");
          setIsProtectionEnabled(false);
      }
    } catch (e) {
      console.error("Failed to toggle protection", e);
      setSupportMessage("Initialization Failed");
      setIsStarting(false);
    }
  };

  const saveCalibration = async (skip: boolean) => {
    setShowCalibration(false);
    if (skip) {
        setSupportMessage("Manual Adaptive Mode active. Limited adaptation.");
        await startEngine();
        return;
    }
    
    // Fallback to room estimation if sensor is unavailable
    const estimatedLux = calibRoom === "Dark" ? 10.0 : calibRoom === "Normal" ? 150.0 : 500.0;
    const actualLux = dashboard?.ambient?.lux ?? estimatedLux;
    
    const profile = {
        reference_brightness: referenceBrightness,
        reference_lux: actualLux,
        min_brightness: 10,
        max_brightness: 100,
        adaptation_speed: "Normal",
        transition_curve: "Smooth",
        sensitivity: 1.0,
        manual_override_timeout_ms: 3600000,
    };
    await invoke("save_config", { config: { brightness: { comfort_profile: profile } } });
    await startEngine();
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

  if (showCalibration) {
    return (
      <div className="flex-1 p-10 overflow-y-auto w-full max-w-2xl mx-auto flex flex-col justify-center">
        <Card className="p-8 flex flex-col items-center bg-card">
          <h2 className="text-2xl font-bold mb-4">Quick Calibration</h2>
          <p className="text-muted-foreground mb-8 text-center">
            To adapt to your environment, PixelSense needs to know what looks good to you right now.
          </p>

          <div className="w-full mb-8">
            <label className="block text-sm font-medium mb-2">1. Set your preferred brightness</label>
            <input 
              type="range" 
              min="0" 
              max="100" 
              value={referenceBrightness}
              onChange={handleSliderChange}
              className="w-full h-2 bg-secondary rounded-lg appearance-none cursor-pointer accent-primary" 
            />
            <div className="text-right text-xs text-muted-foreground mt-1">{referenceBrightness}%</div>
          </div>

          <div className="w-full mb-10">
            <label className="block text-sm font-medium mb-3">2. How bright is your room right now?</label>
            <div className="flex gap-3">
              {['Dark', 'Normal', 'Bright'].map((room) => (
                <button
                  key={room}
                  onClick={() => setCalibRoom(room)}
                  className={`flex-1 py-3 px-4 rounded-md border-2 transition-colors ${
                    calibRoom === room 
                      ? 'border-primary bg-primary/10 text-primary font-semibold' 
                      : 'border-border bg-transparent text-foreground hover:bg-secondary'
                  }`}
                >
                  {room}
                </button>
              ))}
            </div>
          </div>

          <div className="flex gap-4 w-full">
            <button 
              onClick={() => saveCalibration(true)}
              className="flex-1 py-3 border border-border text-foreground hover:bg-secondary rounded-md transition-colors"
            >
              Skip Calibration
            </button>
            <button 
              onClick={() => saveCalibration(false)}
              className="flex-1 py-3 bg-primary text-primary-foreground hover:bg-primary/90 rounded-md transition-colors font-medium"
            >
              Save Profile
            </button>
          </div>
        </Card>
      </div>
    );
  }

  return (
    <div className="flex-1 p-10 overflow-y-auto w-full max-w-3xl mx-auto flex flex-col justify-center min-h-full">
      <section className="mb-8 w-full flex flex-col items-center">
        <div className="w-16 h-16 rounded-full bg-primary/10 flex items-center justify-center text-primary mb-4">
          <ShieldCheck className="w-8 h-8" />
        </div>
        <h1 className="text-4xl font-semibold tracking-tight text-foreground mb-12">
          PixelSense
        </h1>
        


        <button 
          onClick={toggleProtection}
          disabled={isStarting}
          className={`px-12 py-4 font-semibold rounded-full transition-all duration-[2000ms] ease-in-out text-lg focus-visible:outline-ring shadow-lg w-full max-w-sm mb-6 flex justify-center items-center gap-3 ${
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
          {isStarting ? "Starting..." : isProtectionEnabled ? "Disable Protection" : "Enable Protection"}
        </button>

        {supportMessage && (
          <p className="text-sm text-destructive font-medium max-w-sm text-center mb-6">
            {supportMessage}
          </p>
        )}

        {/* Live Engine Cards */}
        <div className="w-full max-w-3xl grid grid-cols-1 md:grid-cols-3 gap-4 mt-8">
          
          {/* Protection Card — answers: "Is protection active?" and "How comfortable am I?" */}
          <Card className="p-5 flex flex-col gap-3 bg-card/50 border-border/50">
            <div className="flex items-center gap-2">
              <div className="p-1.5 bg-primary/10 rounded-md text-primary">
                <ShieldCheck className="w-4 h-4" />
              </div>
              <span className="text-xs font-semibold text-muted-foreground uppercase tracking-wider">Protection</span>
            </div>
            <div className="flex flex-col gap-1 mt-1">
              <span className={`text-lg font-semibold ${isProtectionEnabled ? 'text-foreground' : 'text-muted-foreground'}`}>
                {isProtectionEnabled ? 'Running' : 'Stopped'}
              </span>
              <span className="text-sm text-muted-foreground">
                {dashboard?.intelligence?.comfort_score?.total_score !== undefined
                  ? `Comfort: ${dashboard.intelligence.comfort_score.total_score >= 80 ? 'Excellent' : dashboard.intelligence.comfort_score.total_score >= 60 ? 'Good' : dashboard.intelligence.comfort_score.total_score >= 40 ? 'Fair' : 'Low'} — ${dashboard.intelligence.comfort_score.total_score}%`
                  : isProtectionEnabled ? 'Calculating...' : 'Enable to start'}
              </span>
            </div>
            <div className="h-8 w-full -ml-2 -mb-1 opacity-30 pointer-events-none">
              <ResponsiveContainer width="100%" height="100%">
                <AreaChart data={history.brightness} margin={{ top: 0, left: 0, right: 0, bottom: 0 }}>
                  <Area type="basis" dataKey="value" stroke="currentColor" fill="currentColor" className="text-primary" strokeWidth={2} isAnimationActive={false} />
                </AreaChart>
              </ResponsiveContainer>
            </div>
          </Card>

          {/* Display Card — answers: "What is my brightness doing?" */}
          <Card className="p-5 flex flex-col gap-3 bg-card/50 border-border/50">
            <div className="flex items-center gap-2">
              <div className="p-1.5 bg-primary/10 rounded-md text-primary">
                <Sun className="w-4 h-4" />
              </div>
              <span className="text-xs font-semibold text-muted-foreground uppercase tracking-wider">Display</span>
            </div>
            <div className="flex flex-col gap-1 mt-1">
              <div className="flex items-baseline gap-2">
                <span className="text-2xl font-semibold text-foreground">{dashboard?.brightness?.current ?? '--'}%</span>
                {dashboard?.brightness?.target !== null && dashboard?.brightness?.target !== undefined && dashboard?.brightness?.target !== dashboard?.brightness?.current && (
                  <span className="text-sm text-muted-foreground">→ {dashboard.brightness.target}%</span>
                )}
              </div>
              <span className="text-sm text-muted-foreground">
                {dashboard?.brightness?.transition_status === 'Adjusting' ? 'Transitioning...' 
                  : dashboard?.brightness?.transition_status === 'Cooldown' ? 'Stabilizing'
                  : dashboard?.brightness?.transition_status === 'Suspended (Manual)' ? 'Paused'
                  : 'Stable'}
              </span>
            </div>
            <div className="h-8 w-full -ml-2 -mb-1 opacity-30 pointer-events-none">
              <ResponsiveContainer width="100%" height="100%">
                <LineChart data={history.brightness} margin={{ top: 0, left: 0, right: 0, bottom: 0 }}>
                  <Line type="basis" dataKey="value" stroke="currentColor" dot={false} strokeWidth={2} className="text-primary" isAnimationActive={false} />
                </LineChart>
              </ResponsiveContainer>
            </div>
          </Card>

          {/* Environment Card — answers: "What is my room like?" */}
          <Card className="p-5 flex flex-col gap-3 bg-card/50 border-border/50">
            <div className="flex items-center gap-2">
              <div className="p-1.5 bg-primary/10 rounded-md text-primary">
                <Lightbulb className="w-4 h-4" />
              </div>
              <span className="text-xs font-semibold text-muted-foreground uppercase tracking-wider">Environment</span>
            </div>
            <div className="flex flex-col gap-1 mt-1">
              <span className="text-lg font-semibold text-foreground">
                {dashboard?.ambient?.lux !== null && dashboard?.ambient?.lux !== undefined
                  ? dashboard.ambient.lux < 20 ? 'Dark Room'
                    : dashboard.ambient.lux < 100 ? 'Dim Room'
                    : dashboard.ambient.lux < 400 ? 'Normal Room'
                    : dashboard.ambient.lux < 1000 ? 'Bright Room'
                    : 'Sunlit'
                  : 'Unknown'}
              </span>
              <span className="text-sm text-muted-foreground">
                {dashboard?.ambient?.lux !== null && dashboard?.ambient?.lux !== undefined
                  ? `${Math.round(dashboard.ambient.lux)} lux · ${dashboard.screen?.context || 'Desktop'}`
                  : dashboard?.health?.ambient_engine || 'Sensor unavailable'}
              </span>
            </div>
            <div className="h-8 w-full -ml-2 -mb-1 opacity-30 pointer-events-none">
              <ResponsiveContainer width="100%" height="100%">
                <LineChart data={history.lux} margin={{ top: 0, left: 0, right: 0, bottom: 0 }}>
                  <Line type="basis" dataKey="value" stroke="currentColor" dot={false} strokeWidth={2} className="text-primary" isAnimationActive={false} />
                </LineChart>
              </ResponsiveContainer>
            </div>
          </Card>
        </div>


        {/* Explainability Card */}
        {dashboard?.intelligence?.current_decision && isProtectionEnabled && (
          <div className="w-full max-w-3xl mt-4">
            <Card className="p-5 flex flex-col gap-3 bg-secondary/20 border-border/50">
              <div className="flex items-center gap-2 mb-1">
                <div className="p-1.5 bg-primary/20 rounded-full text-primary">
                  <Lightbulb className="w-4 h-4" />
                </div>
                <h3 className="text-sm font-semibold text-foreground">Why did PixelSense do this?</h3>
              </div>
              <div className="flex flex-col gap-2">
                <div className="flex justify-between items-center text-sm">
                  <span className="text-muted-foreground">Observation:</span>
                  <span className="text-foreground text-right font-medium max-w-[70%]">{dashboard.intelligence.current_decision.observation}</span>
                </div>
                <div className="flex justify-between items-center text-sm">
                  <span className="text-muted-foreground">Reasoning:</span>
                  <span className="text-foreground text-right font-medium max-w-[70%]">{dashboard.intelligence.current_decision.reason}</span>
                </div>
                <div className="flex justify-between items-center text-sm mt-2 pt-2 border-t border-border/50">
                  <span className="text-muted-foreground">Action:</span>
                  <span className="text-primary text-right font-semibold">{dashboard.intelligence.current_decision.action}</span>
                </div>
              </div>
            </Card>
          </div>
        )}
      </section>
    </div>
  );
}

