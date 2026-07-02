import { motion } from "framer-motion";
import { Sun, Moon, Monitor, BatteryMedium, Cpu, Activity, Zap, CheckCircle2, AlertTriangle, XCircle, Info } from "lucide-react";
import { useStore } from "../store/useStore";
import { Card } from "../components/ui/Card";
import { Metric } from "../components/ui/Metric";
import { StatusPill } from "../components/ui/StatusPill";
import { AnimatedValue } from "../components/ui/AnimatedValue";

export function Overview() {
  const { dashboard } = useStore();

  if (!dashboard) {
    return (
      <div className="flex-1 flex items-center justify-center p-10 h-full">
        <motion.div 
          animate={{ opacity: [0.5, 1, 0.5] }}
          transition={{ repeat: Infinity, duration: 1.5 }}
          className="text-muted-foreground flex items-center gap-3"
        >
          <Activity className="w-5 h-5 animate-spin" />
          <span>Connecting to PixelSense Engine...</span>
        </motion.div>
      </div>
    );
  }

  const { comfort, ambient, screen, brightness, performance, health } = dashboard;

  const getStatusVariant = (status: string) => {
    if (status === "Comfortable" || status === "Good" || status === "Active" || status === "Idle") return "success";
    if (status === "Adjusting" || status === "Warning") return "warning";
    if (status === "Strained" || status === "Error" || status === "Offline") return "error";
    return "default";
  };

  return (
    <div className="flex-1 p-10 overflow-y-auto w-full max-w-[1600px] mx-auto">
      <header className="mb-10">
        <h1 className="text-3xl font-semibold tracking-tight text-foreground">Overview</h1>
        <p className="text-muted-foreground mt-1">Real-time comfort analysis and engine intelligence.</p>
      </header>

      <div className="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-6">
        
        {/* 1. Hero Comfort Card */}
        <Card className="xl:col-span-3 bg-gradient-to-br from-card to-sidebar border-l-4 border-l-accent shadow-md">
          <div className="flex items-center justify-between mb-4">
            <h2 className="text-xl font-medium tracking-tight">Current Comfort</h2>
            <StatusPill variant={getStatusVariant(comfort.status)} pulse={comfort.status === "Comfortable"}>
              {comfort.status}
            </StatusPill>
          </div>
          <div className="grid grid-cols-2 md:grid-cols-4 gap-6 items-end">
            <Metric 
              label="Recommendation" 
              value={comfort.recommendation} 
              className="col-span-2 md:col-span-2 text-xl"
            />
            <Metric 
              label="Active Profile" 
              value={comfort.active_profile} 
            />
            <div className="flex flex-col gap-2">
              <span className="text-sm font-medium uppercase tracking-wider text-muted-foreground">Confidence</span>
              <div className="flex items-center gap-3">
                <span className="text-2xl font-semibold"><AnimatedValue value={comfort.confidence * 100} format={(v) => Math.round(v).toString()} />%</span>
                <div className="flex-1 h-2 bg-secondary rounded-full overflow-hidden">
                  <motion.div 
                    className="h-full bg-accent rounded-full" 
                    initial={{ width: 0 }}
                    animate={{ width: `${comfort.confidence * 100}%` }}
                    transition={{ duration: 0.5 }}
                  />
                </div>
              </div>
            </div>
          </div>
        </Card>

        {/* 2. Environment Card */}
        <Card>
          <div className="flex items-center justify-between mb-4">
            <div className="flex items-center gap-2">
              <Sun className="w-5 h-5 text-muted-foreground" />
              <h2 className="text-lg font-medium">Environment</h2>
            </div>
            <StatusPill variant={getStatusVariant(ambient.health)}>{ambient.source}</StatusPill>
          </div>
          <div className="grid grid-cols-2 gap-4">
            <Metric 
              label="Ambient Light" 
              value={<><AnimatedValue value={ambient.lux} /> <span className="text-sm text-muted-foreground font-normal">lux</span></>}
            />
            <Metric 
              label="Type" 
              value={ambient.environment}
            />
          </div>
        </Card>

        {/* 3. Screen Analysis Card */}
        <Card>
          <div className="flex items-center justify-between mb-4">
            <div className="flex items-center gap-2">
              <Monitor className="w-5 h-5 text-muted-foreground" />
              <h2 className="text-lg font-medium">Screen Analysis</h2>
            </div>
            <StatusPill variant="default">{screen.current_analysis_time_ms} ms</StatusPill>
          </div>
          <div className="grid grid-cols-2 gap-4">
            <Metric 
              label="Avg Luminance" 
              value={<><AnimatedValue value={screen.average_luminance} /> <span className="text-sm text-muted-foreground font-normal">nits</span></>}
            />
            <Metric 
              label="Visual Complexity" 
              value={<><AnimatedValue value={screen.visual_complexity * 100} />%</>}
            />
          </div>
        </Card>

        {/* 4. Brightness Card */}
        <Card>
          <div className="flex items-center justify-between mb-4">
            <div className="flex items-center gap-2">
              <Zap className="w-5 h-5 text-muted-foreground" />
              <h2 className="text-lg font-medium">Display Brightness</h2>
            </div>
            <StatusPill variant={getStatusVariant(brightness.transition_status)}>{brightness.transition_status}</StatusPill>
          </div>
          <div className="grid grid-cols-2 gap-4">
            <Metric 
              label="Current Target" 
              value={<><AnimatedValue value={brightness.target} />%</>}
            />
            <Metric 
              label="Eye Comfort" 
              value={<><AnimatedValue value={brightness.eye_comfort_score} format={v => v.toFixed(1)} /> <span className="text-sm text-muted-foreground font-normal">/ 10</span></>}
            />
          </div>
        </Card>

        {/* 5. Performance Card */}
        <Card>
          <div className="flex items-center justify-between mb-4">
            <div className="flex items-center gap-2">
              <Cpu className="w-5 h-5 text-muted-foreground" />
              <h2 className="text-lg font-medium">Performance Engine</h2>
            </div>
            <StatusPill variant={performance.power_state === "AC" ? "success" : "warning"}>{performance.power_state}</StatusPill>
          </div>
          <div className="grid grid-cols-2 gap-4">
            <Metric 
              label="CPU Usage" 
              value={<><AnimatedValue value={performance.cpu_usage_pct} format={v => v.toFixed(2)} />%</>}
            />
            <Metric 
              label="Interval" 
              value={<AnimatedValue value={performance.current_poll_interval_ms} format={v => `${Math.round(v)}ms`} />}
            />
          </div>
        </Card>

        {/* 6. Engine Health Card */}
        <Card className="lg:col-span-2 xl:col-span-1 border-border">
          <div className="flex items-center justify-between mb-4">
            <div className="flex items-center gap-2">
              <Activity className="w-5 h-5 text-muted-foreground" />
              <h2 className="text-lg font-medium">Engine Health</h2>
            </div>
          </div>
          <div className="flex flex-col gap-3">
            {[
              { name: "Background Worker", status: health.background_worker },
              { name: "Ambient Engine", status: health.ambient_engine },
              { name: "Screen Analysis", status: health.screen_engine },
              { name: "Visual Comfort", status: health.comfort_engine },
            ].map((engine) => (
              <div key={engine.name} className="flex items-center justify-between text-sm">
                <span className="text-muted-foreground">{engine.name}</span>
                <span className="flex items-center gap-1.5 font-medium text-foreground">
                  {engine.status === "Active" ? (
                    <CheckCircle2 className="w-4 h-4 text-green-500" />
                  ) : engine.status === "Warning" ? (
                    <AlertTriangle className="w-4 h-4 text-yellow-500" />
                  ) : (
                    <XCircle className="w-4 h-4 text-red-500" />
                  )}
                  {engine.status}
                </span>
              </div>
            ))}
          </div>
        </Card>

      </div>
    </div>
  );
}
