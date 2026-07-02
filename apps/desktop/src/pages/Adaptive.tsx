import { useConfigStore } from '../store/configStore';

export const Adaptive = () => {
  const { config, updateConfig } = useConfigStore();

  return (
    <div>
      <h1>Adaptive Intelligence</h1>
      <div className="card">
        <h2>Adaptive Brightness</h2>
        <p>Automatically adjust screen brightness based on environment and time of day.</p>
        <div className="form-group">
          <label className="toggle-switch" aria-label="Enable Adaptive Brightness">
            <input 
              type="checkbox" 
              checked={config.adaptive.enabled} 
              onChange={(e) => updateConfig(d => { d.adaptive.enabled = e.target.checked; })}
            />
            <span className="slider-round"></span>
          </label>
        </div>
      </div>

      <div className="card">
        <h2>Confidence Threshold</h2>
        <p>Require high confidence from sensors before applying changes.</p>
        <div className="form-group">
          <label className="form-label" htmlFor="confidence-slider">
            Required Confidence: {Math.round(config.adaptive.confidence_threshold * 100)}%
          </label>
          <input 
            id="confidence-slider"
            type="range" 
            min="0" 
            max="100" 
            value={config.adaptive.confidence_threshold * 100} 
            onChange={(e) => updateConfig(d => { d.adaptive.confidence_threshold = parseInt(e.target.value, 10) / 100; })}
            disabled={!config.adaptive.enabled}
          />
        </div>
      </div>
    </div>
  );
};
