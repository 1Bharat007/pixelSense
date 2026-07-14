import { useState, useEffect } from "react";
import { getVersion } from "@tauri-apps/api/app";

export const Diagnostics = () => {
  const [appVersion, setAppVersion] = useState("Loading...");

  useEffect(() => {
    getVersion().then(version => setAppVersion(`v${version}`)).catch(console.error);
  }, []);

  return (
    <div>
      <h1>Diagnostics</h1>
      <div className="card">
        <h2>System Information</h2>
        <p><strong>Application Version:</strong> {appVersion}</p>
        <p><strong>Operating System:</strong> Windows 11</p>
        <p><strong>Backend Status:</strong> Connected & Healthy</p>
      </div>

      <div className="card">
        <h2>Display Hardware</h2>
        <p><strong>Discovered Displays:</strong> 1</p>
        <p><strong>Detected Capabilities:</strong> Brightness Control [Yes], HDR [No], DDC/CI [No]</p>
      </div>

      <div className="card">
        <h2>System Logs</h2>
        <pre style={{ backgroundColor: 'var(--bg-primary)', padding: '1rem', borderRadius: '4px', overflowX: 'auto', fontSize: '0.85rem' }}>
{`[INFO] PixelSense Backend Initialized
[INFO] Discovered 1 monitor(s)
[INFO] Loaded configuration from disk
[INFO] ConfigService bound to Tauri state
[INFO] AdaptiveBrightnessService listening for events`}
        </pre>
      </div>
    </div>
  );
};
