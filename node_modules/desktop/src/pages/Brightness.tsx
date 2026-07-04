import { useState } from 'react';
import { useConfigStore } from '../store/configStore';

export const Brightness = () => {
  const { config, updateConfig, previewBrightness } = useConfigStore();
  const [sliderValue, setSliderValue] = useState(50);

  const handleSliderChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const val = parseInt(e.target.value, 10);
    setSliderValue(val);
    previewBrightness("primary", val);
  };

  const handleTimeoutChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    const val = parseInt(e.target.value, 10);
    updateConfig(draft => { draft.brightness.manual_override_timeout_ms = val; });
  };

  return (
    <div>
      <h1>Brightness</h1>
      <div className="card">
        <h2>Manual Override Preview</h2>
        <p>Preview manual brightness override on the primary display.</p>
        <div className="form-group">
          <label className="form-label" htmlFor="brightness-slider">Brightness Level: {sliderValue}%</label>
          <input 
            id="brightness-slider"
            type="range" 
            min="0" 
            max="100" 
            value={sliderValue} 
            onChange={handleSliderChange} 
          />
        </div>
      </div>

      <div className="card">
        <h2>Override Settings</h2>
        <p>How long should manual adjustments pause adaptive brightness?</p>
        <div className="form-group">
          <label className="form-label" htmlFor="timeout-select">Pause Duration</label>
          <select id="timeout-select" value={config.brightness.manual_override_timeout_ms} onChange={handleTimeoutChange}>
            <option value={3600000}>1 Hour</option>
            <option value={7200000}>2 Hours</option>
            <option value={86400000}>Until Tomorrow</option>
          </select>
        </div>
      </div>
    </div>
  );
};
