import { useStore } from "../store/useStore";
import { Card } from "../components/ui/Card";
import { Metric } from "../components/ui/Metric";
import { Server, Clock, Database, Cpu } from "lucide-react";
import { AnimatedValue } from "../components/ui/AnimatedValue";
import { StatusPill } from "../components/ui/StatusPill";

export function Developer() {
  const { dashboard, eventLog } = useStore();

  if (!dashboard) {
    return <div className="p-10">Loading diagnostics...</div>;
  }

  const { performance, screen, ambient, brightness } = dashboard;

  return (
    <div className="flex-1 p-10 overflow-y-auto w-full max-w-3xl mx-auto">
      <header className="mb-10 flex justify-between items-end">
        <div>
          <h1 className="text-3xl font-semibold tracking-tight text-foreground flex items-center gap-3">
            <Server className="w-8 h-8 text-accent" />
            Developer Diagnostics
          </h1>
          <p className="text-muted-foreground mt-1">Raw telemetry and pipeline internals.</p>
        </div>
        <StatusPill variant={performance.power_state === "AC" ? "success" : "warning"} pulse>
          Live Telemetry Active
        </StatusPill>
      </header>

      <div className="grid grid-cols-1 xl:grid-cols-2 gap-6">
        
        {/* Pipeline Profiler */}
        <Card>
          <div className="flex items-center gap-2 mb-6 border-b border-border pb-4">
            <Clock className="w-5 h-5 text-muted-foreground" />
            <h2 className="text-lg font-medium">Pipeline Profiler</h2>
          </div>
          <div className="grid grid-cols-2 gap-y-6 gap-x-4">
            <Metric label="Total Cycle Time" value={<><AnimatedValue value={performance.pipeline_duration_ms ?? 0} />ms</>} />
            <Metric label="Screen Analysis" value={<><AnimatedValue value={screen.current_analysis_time_ms ?? 0} />ms</>} />
            <Metric label="Current Poll Interval" value={<><AnimatedValue value={performance.current_poll_interval_ms ?? 0} />ms</>} />
            <Metric label="Visual Complexity" value={<><AnimatedValue value={(screen.visual_complexity ?? 0) * 100} format={v => v.toFixed(2)} />%</>} />
          </div>
        </Card>

        {/* Event Log */}
        <Card className="col-span-1 xl:col-span-2 overflow-hidden flex flex-col min-h-[400px]">
          <div className="flex items-center gap-2 p-5 border-b border-border shrink-0">
            <Server className="w-5 h-5 text-muted-foreground" />
            <h2 className="text-lg font-medium">Event Log (Recent Activity)</h2>
          </div>
          <div className="flex-1 overflow-y-auto p-0">
            <table className="w-full text-sm text-left">
              <thead className="text-xs text-muted-foreground uppercase bg-secondary/20 sticky top-0">
                <tr>
                  <th className="px-5 py-3">Time</th>
                  <th className="px-5 py-3">Category</th>
                  <th className="px-5 py-3">Description</th>
                  <th className="px-5 py-3">Change</th>
                </tr>
              </thead>
              <tbody>
                {eventLog.map((event, idx) => (
                  <tr key={idx} className="border-b border-border/50 hover:bg-secondary/10">
                    <td className="px-5 py-3 text-muted-foreground whitespace-nowrap">
                      {new Date(event.timestamp_ms).toLocaleTimeString()}
                    </td>
                    <td className="px-5 py-3 font-medium">
                      {event.category}
                    </td>
                    <td className="px-5 py-3">
                      {event.description}
                    </td>
                    <td className="px-5 py-3 text-muted-foreground whitespace-nowrap">
                      {event.before_value && event.after_value ? (
                        <span>{event.before_value} → <span className="text-foreground">{event.after_value}</span></span>
                      ) : (
                        "-"
                      )}
                    </td>
                  </tr>
                ))}
                {eventLog.length === 0 && (
                  <tr>
                    <td colSpan={4} className="px-5 py-8 text-center text-muted-foreground">
                      No recent events recorded.
                    </td>
                  </tr>
                )}
              </tbody>
            </table>
          </div>
        </Card>

        {/* Ambient Internals */}
        <Card>
          <div className="flex items-center gap-2 mb-6 border-b border-border pb-4">
            <Database className="w-5 h-5 text-muted-foreground" />
            <h2 className="text-lg font-medium">Ambient Engine State</h2>
          </div>
          <div className="grid grid-cols-2 gap-y-6 gap-x-4">
            <Metric label="Raw Lux" value={<AnimatedValue value={ambient.lux ?? 0} format={v => v.toFixed(1)} />} />
            <Metric label="Confidence Multiplier" value={<AnimatedValue value={ambient.confidence ?? 0} format={v => v.toFixed(4)} />} />
            <Metric label="Hardware Source" value={<span className="text-lg">{ambient.source}</span>} />
            <Metric label="Computed Environment" value={<span className="text-lg">{ambient.environment}</span>} />
          </div>
        </Card>

        {/* Brightness Controller */}
        <Card>
          <div className="flex items-center gap-2 mb-6 border-b border-border pb-4">
            <Cpu className="w-5 h-5 text-muted-foreground" />
            <h2 className="text-lg font-medium">Brightness Controller</h2>
          </div>
          <div className="grid grid-cols-2 gap-y-6 gap-x-4">
            <Metric label="Current Display Level" value={<><AnimatedValue value={brightness.current ?? 0} />%</>} />
            <Metric label="Target Display Level" value={<><AnimatedValue value={brightness.target ?? 0} />%</>} />
            <Metric label="Transition State" value={<span className="text-lg">{brightness.transition_status}</span>} />
            <Metric label="Transition Progress" value={<><AnimatedValue value={(brightness.transition_progress ?? 0) * 100} format={v => Math.round(v).toString()} />%</>} />
          </div>
        </Card>

        {/* Intelligence Engine */}
        <Card>
          <div className="flex items-center gap-2 mb-6 border-b border-border pb-4">
            <Server className="w-5 h-5 text-muted-foreground" />
            <h2 className="text-lg font-medium">Intelligence Engine</h2>
          </div>
          <div className="grid grid-cols-2 gap-y-6 gap-x-4">
            <Metric label="Current Context" value={<span className="text-lg">{screen.context || "Desktop"}</span>} />
            <Metric label="Decision Action" value={<span className="text-md leading-tight">{dashboard?.intelligence?.current_decision?.action || "None"}</span>} />
            <Metric label="Decision Confidence" value={<><AnimatedValue value={dashboard?.intelligence?.current_decision?.confidence ?? 0} />%</>} />
            <Metric label="Comfort Score" value={<><AnimatedValue value={dashboard?.intelligence?.comfort_score?.total_score ?? 0} />%</>} />
          </div>
        </Card>

      </div>
    </div>
  );
}
