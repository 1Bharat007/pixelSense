import { useState, useEffect } from "react";
import { getVersion } from "@tauri-apps/api/app";
import { useStore } from "../store/useStore";
import { Card } from "../components/ui/Card";

export const Diagnostics = () => {
  const [appVersion, setAppVersion] = useState("Loading...");
  const { dashboard, hardwareCapabilities } = useStore();

  useEffect(() => {
    getVersion().then(version => setAppVersion(`v${version}`)).catch(console.error);
    // hardwareCapabilities is fetched once in App.tsx or Overview, but we can rely on the store
  }, []);

  const getStatusColor = (status: string) => {
    if (!status) return "text-muted-foreground";
    if (status.includes("Running") || status.includes("Healthy")) return "text-green-500";
    if (status.includes("Stopped") || status.includes("Paused")) return "text-muted-foreground";
    if (status.includes("Failed") || status.includes("Error") || status.includes("Unavailable")) return "text-red-500";
    return "text-yellow-500"; // Warning
  };

  return (
    <div className="flex-1 p-10 overflow-y-auto w-full max-w-3xl mx-auto flex flex-col min-h-full">
      <h1 className="text-3xl font-bold tracking-tight mb-8">Diagnostics</h1>
      
      <Card className="p-6 mb-6">
        <h2 className="text-xl font-semibold mb-4 border-b border-border pb-2">Hardware</h2>
        <div className="grid grid-cols-1 gap-4">
          <div className="flex justify-between items-center">
            <div className="flex flex-col">
              <span className="text-foreground font-medium">Internal Display</span>
              <span className="text-xs text-muted-foreground">Required for brightness control</span>
            </div>
            <span className="font-medium">{hardwareCapabilities?.internal_display ? "Supported" : "Unsupported"}</span>
          </div>
          <div className="flex justify-between items-center">
            <div className="flex flex-col">
              <span className="text-foreground font-medium">Brightness API</span>
              <span className="text-xs text-muted-foreground">How PixelSense controls the screen</span>
            </div>
            <span className="font-medium">{hardwareCapabilities?.brightness_api || "Scanning..."}</span>
          </div>
          <div className="flex justify-between items-center">
            <div className="flex flex-col">
              <span className="text-foreground font-medium">Ambient Sensor</span>
              <span className="text-xs text-muted-foreground">Used for room lighting detection</span>
            </div>
            <span className={`font-medium ${hardwareCapabilities?.ambient_available ? "text-green-500" : "text-yellow-500"}`}>
              {hardwareCapabilities?.ambient_sensor || "Scanning..."}
            </span>
          </div>
        </div>
      </Card>

      <Card className="p-6 mb-6">
        <h2 className="text-xl font-semibold mb-4 border-b border-border pb-2">Background Services</h2>
        <div className="grid grid-cols-1 gap-4">
          <div className="flex justify-between items-center">
            <div className="flex flex-col">
              <span className="text-foreground font-medium">Brightness Fader</span>
              <span className="text-xs text-muted-foreground">Handles smooth brightness fading</span>
            </div>
            <span className={`font-medium ${getStatusColor(dashboard?.health?.transition_engine || "")}`}>
              {dashboard?.health?.transition_engine || "Stopped"}
            </span>
          </div>
          <div className="flex justify-between items-center">
            <div className="flex flex-col">
              <span className="text-foreground font-medium">Smart Logic</span>
              <span className="text-xs text-muted-foreground">Computes target brightness</span>
            </div>
            <span className={`font-medium ${getStatusColor(dashboard?.health?.comfort_engine || "")}`}>
              {dashboard?.health?.comfort_engine || "Stopped"}
            </span>
          </div>
          <div className="flex justify-between items-center">
            <div className="flex flex-col">
              <span className="text-foreground font-medium">Coordinator</span>
              <span className="text-xs text-muted-foreground">Coordinates all intelligence features</span>
            </div>
            <span className={`font-medium ${getStatusColor(dashboard?.health?.background_worker || "")}`}>
              {dashboard?.health?.background_worker || "Stopped"}
            </span>
          </div>
          <div className="flex justify-between items-center">
            <div className="flex flex-col">
              <span className="text-foreground font-medium">System Monitor</span>
              <span className="text-xs text-muted-foreground">Ensures everything keeps running</span>
            </div>
            <span className={`font-medium ${getStatusColor(dashboard?.health?.watchdog || "")}`}>
              {dashboard?.health?.watchdog || "Stopped"}
            </span>
          </div>
          <div className="flex justify-between items-center">
            <div className="flex flex-col">
              <span className="text-foreground font-medium">Light Sensor Reader</span>
              <span className="text-xs text-muted-foreground">Reads hardware light sensors</span>
            </div>
            <span className={`font-medium ${getStatusColor(dashboard?.health?.ambient_engine || "")}`}>
              {dashboard?.health?.ambient_engine || "Stopped"}
            </span>
          </div>
          <div className="flex justify-between items-center">
            <div className="flex flex-col">
              <span className="text-foreground font-medium">Screen Engine</span>
              <span className="text-xs text-muted-foreground">Analyzes screen content and context</span>
            </div>
            <span className={`font-medium ${getStatusColor(dashboard?.health?.screen_engine || "")}`}>
              {dashboard?.health?.screen_engine || "Stopped"}
            </span>
          </div>
        </div>
      </Card>
      <Card className="p-6">
        <h2 className="text-xl font-semibold mb-4 border-b border-border pb-2">System Information</h2>
        <div className="grid grid-cols-2 gap-4">
          <div className="flex justify-between">
            <span className="text-muted-foreground">Application Version</span>
            <span className="font-medium">{appVersion}</span>
          </div>
          <div className="flex justify-between">
            <span className="text-muted-foreground">Operating System</span>
            <span className="font-medium">Windows</span>
          </div>
        </div>
      </Card>
    </div>
  );
};
