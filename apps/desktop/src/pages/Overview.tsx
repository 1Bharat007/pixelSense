import { Sun, Monitor, ShieldCheck, AlertCircle } from "lucide-react";
import { useStore } from "../store/useStore";
import { Card } from "../components/ui/Card";
import { StatusPill } from "../components/ui/StatusPill";
import { AnimatedValue } from "../components/ui/AnimatedValue";
import { Tooltip } from "../components/ui/Tooltip";

export function Overview() {
  const { dashboard, error } = useStore();

  // 1. Error Experience System (Graceful Degradation)
  if (error) {
    return (
      <div className="flex-1 p-10 overflow-y-auto w-full max-w-6xl mx-auto flex flex-col items-center justify-center">
        <Card className="max-w-md w-full bg-destructive/5 border-destructive/20 items-center text-center p-8">
          <div className="w-16 h-16 rounded-full bg-destructive/10 flex items-center justify-center text-destructive mb-4">
            <AlertCircle className="w-8 h-8" />
          </div>
          <h2 className="text-xl font-semibold text-foreground mb-2">Sensor Connection Lost</h2>
          <p className="text-muted-foreground mb-6">
            We couldn't connect to your monitor's built-in light sensor. This usually happens when the monitor goes to sleep.
          </p>
          <div className="flex w-full gap-3">
            <button className="flex-1 py-2 bg-secondary hover:bg-secondary/80 text-secondary-foreground font-medium rounded-md transition-colors">
              Use Software Estimation
            </button>
            <button className="flex-1 py-2 bg-primary hover:bg-primary/90 text-primary-foreground font-medium rounded-md transition-colors">
              Retry Sensor
            </button>
          </div>
        </Card>
      </div>
    );
  }

  // 2. Progressive Hydration (Skeleton Cards)
  if (!dashboard) {
    return (
      <div className="flex-1 p-10 overflow-y-auto w-full max-w-6xl mx-auto animate-pulse">
        <section className="mb-8">
          <Card className="h-40 bg-secondary/20 border-transparent" />
        </section>
        <section className="mb-8">
          <Card className="h-28 bg-secondary/20 border-transparent" />
        </section>
        <section className="grid grid-cols-1 md:grid-cols-2 gap-6">
          <Card className="h-32 bg-secondary/20 border-transparent" />
          <Card className="h-32 bg-secondary/20 border-transparent" />
        </section>
      </div>
    );
  }

  const { ambient, screen, brightness, intelligence } = dashboard;
  const score = intelligence.comfort_score;
  const topRec = intelligence.recommendations[0];

  return (
    <div className="flex-1 p-10 overflow-y-auto w-full max-w-6xl mx-auto">
      
      {/* 1. Hero: How comfortable am I? */}
      <section className="mb-8">
        <Card className="bg-card shadow-sm border-l-4 border-l-primary flex flex-col md:flex-row items-center justify-between p-8">
          <div className="flex flex-col gap-2">
            <h1 className="text-4xl font-semibold tracking-tight text-foreground flex items-center gap-2">
              Your comfort is optimal.
              <Tooltip content="We calculate comfort based on the difference between your screen's brightness and the room's ambient light." />
            </h1>
            <p className="text-lg text-muted-foreground">
              PixelSense is actively balancing your screen with the current room lighting.
            </p>
          </div>
          <div className="mt-6 md:mt-0 flex flex-col items-end">
            <span className="text-6xl font-bold text-foreground">
              <AnimatedValue value={score.total_score} format={(v) => Math.round(v).toString()} />
            </span>
            <StatusPill variant="success" className="mt-2 text-sm uppercase tracking-widest">
              Comfort Score
            </StatusPill>
          </div>
        </Card>
      </section>

      {/* 2. Empty State / Recommendations */}
      <section className="mb-8">
        {topRec ? (
          <Card className="border-border/50 bg-secondary/30 flex items-center justify-between p-6">
            <div className="flex items-center gap-4">
              <div className="p-3 bg-primary/10 rounded-full text-primary">
                <ShieldCheck className="w-6 h-6" />
              </div>
              <div>
                <h3 className="text-lg font-medium text-foreground">{topRec.title}</h3>
                <p className="text-muted-foreground">{topRec.reason}</p>
              </div>
            </div>
            <button className="px-6 py-2 bg-primary hover:bg-primary/90 text-primary-foreground font-medium rounded-md transition-colors focus-visible:outline-ring">
              {topRec.action.replace(/_/g, " ")}
            </button>
          </Card>
        ) : (
          <Card className="border-dashed border-border/50 bg-transparent flex flex-col items-center justify-center p-8 text-center">
             <div className="w-12 h-12 rounded-full bg-secondary flex items-center justify-center text-muted-foreground mb-4">
               <ShieldCheck className="w-6 h-6" />
             </div>
             <h3 className="text-lg font-medium text-foreground mb-1">No pending recommendations</h3>
             <p className="text-muted-foreground text-sm max-w-md">
               Your system is currently tuned perfectly for your environment. We will notify you if any adjustments are needed.
             </p>
          </Card>
        )}
      </section>

      {/* 3. System Status */}
      <section className="grid grid-cols-1 md:grid-cols-2 gap-6">
        <Card interactive>
          <div className="flex items-center justify-between mb-4">
            <h2 className="text-lg font-medium flex items-center gap-2">
              <Sun className="w-5 h-5 text-muted-foreground" />
              Environmental Light
              <Tooltip content="Measured in lux. A normal office is around 300-500 lux." />
            </h2>
            <StatusPill variant="default">Tracking</StatusPill>
          </div>
          <div className="flex items-baseline gap-2">
            <span className="text-4xl font-semibold text-foreground">
              <AnimatedValue value={ambient.lux} />
            </span>
            <span className="text-muted-foreground font-medium">lux</span>
          </div>
          <p className="text-sm text-muted-foreground mt-2">
            The room is considered {ambient.environment.toLowerCase()}.
          </p>
        </Card>

        <Card interactive>
          <div className="flex items-center justify-between mb-4">
            <h2 className="text-lg font-medium flex items-center gap-2">
              <Monitor className="w-5 h-5 text-muted-foreground" />
              Display Brightness
              <Tooltip content="Target brightness calculated for your main display via DDC/CI." />
            </h2>
            <StatusPill variant="default">Auto-adjusted</StatusPill>
          </div>
          <div className="flex items-baseline gap-2">
            <span className="text-4xl font-semibold text-foreground">
              <AnimatedValue value={brightness.target} />
            </span>
            <span className="text-muted-foreground font-medium">%</span>
          </div>
          <p className="text-sm text-muted-foreground mt-2">
            Compensating for screen luminance ({Math.round(screen.average_luminance)} nits).
          </p>
        </Card>
      </section>

    </div>
  );
}
