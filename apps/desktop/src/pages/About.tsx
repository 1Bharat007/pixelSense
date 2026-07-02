import { Info, Monitor, Github, ShieldCheck, Heart } from "lucide-react";
import { Card } from "../components/ui/Card";

export function About() {
  return (
    <div className="flex-1 p-10 overflow-y-auto w-full max-w-[1600px] mx-auto">
      <header className="mb-10">
        <h1 className="text-3xl font-semibold tracking-tight text-foreground flex items-center gap-3">
          <Info className="w-8 h-8 text-accent" />
          About PixelSense
        </h1>
        <p className="text-muted-foreground mt-1">Vision, architecture, and licensing.</p>
      </header>

      <div className="max-w-3xl flex flex-col gap-8">
        <div className="flex flex-col items-center justify-center p-10 bg-secondary/50 rounded-xl border border-border">
          <Monitor className="w-16 h-16 text-accent mb-4" />
          <h2 className="text-2xl font-bold tracking-tight">PixelSense</h2>
          <p className="text-muted-foreground font-medium mt-1">v1.0.0-beta</p>
          <p className="text-center mt-6 text-foreground/80 max-w-lg leading-relaxed">
            PixelSense is an intelligent desktop companion designed to optimize visual comfort in real-time. 
            By merging native ambient sensors with screen content analysis, it ensures your display is always perfectly tuned to your environment.
          </p>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          <Card>
            <ShieldCheck className="w-6 h-6 text-green-500 mb-3" />
            <h3 className="font-semibold text-lg mb-2">Privacy First</h3>
            <p className="text-sm text-muted-foreground leading-relaxed">
              PixelSense operates 100% offline. No webcam data is captured, no telemetry is uploaded, and no sensor readings are ever stored to disk.
            </p>
          </Card>
          
          <Card>
            <Heart className="w-6 h-6 text-red-500 mb-3" />
            <h3 className="font-semibold text-lg mb-2">Open Source</h3>
            <p className="text-sm text-muted-foreground leading-relaxed">
              Built with Rust and Tauri for maximum performance and a near-zero memory footprint. 
              Licensed under MIT.
            </p>
            <button className="mt-4 flex items-center gap-2 text-sm font-medium text-foreground hover:text-accent transition-colors">
              <Github className="w-4 h-4" /> View Source on GitHub
            </button>
          </Card>
        </div>
      </div>
    </div>
  );
}
