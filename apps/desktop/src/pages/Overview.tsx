import { motion } from "framer-motion";
import { Sun, Monitor, ShieldCheck, Activity } from "lucide-react";
import { useStore } from "../store/useStore";
import { Card } from "../components/ui/Card";
import { Metric } from "../components/ui/Metric";
import { StatusPill } from "../components/ui/StatusPill";
import { AnimatedValue } from "../components/ui/AnimatedValue";

export function Overview() {
  const { dashboard } = useStore();

  if (!dashboard) {
    return (
      <div className="flex-1 flex flex-col items-center justify-center p-10 h-full gap-4">
        <motion.div 
          animate={{ opacity: [0.3, 0.7, 0.3] }}
          transition={{ repeat: Infinity, duration: 2 }}
          className="w-12 h-12 rounded-full border-4 border-muted border-t-primary animate-spin"
        />
        <span className="text-muted-foreground font-medium">Initializing environmental sensors...</span>
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
            <h1 className="text-4xl font-semibold tracking-tight text-foreground">
              Your comfort is optimal.
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

      {/* 2. What should I do next? (Actionable Recommendation) */}
      {topRec && (
        <section className="mb-8">
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
        </section>
      )}

      {/* 3. Is everything healthy? (System Status) */}
      <section className="grid grid-cols-1 md:grid-cols-2 gap-6">
        <Card interactive>
          <div className="flex items-center justify-between mb-4">
            <h2 className="text-lg font-medium flex items-center gap-2">
              <Sun className="w-5 h-5 text-muted-foreground" />
              Environmental Light
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
