import { useConfigStore } from '../store/configStore';

export const Transition = () => {
  const { config, updateConfig } = useConfigStore();

  return (
    <div>
      <h1>Transition Engine</h1>
      <div className="card">
        <h2>Smooth Transitions</h2>
        <p>Gradually fade brightness instead of snapping instantly.</p>
        <div className="form-group">
          <label className="toggle-switch" aria-label="Enable Transitions">
            <input 
              type="checkbox" 
              checked={config.transition.enabled} 
              onChange={(e) => updateConfig(d => { d.transition.enabled = e.target.checked; })}
            />
            <span className="slider-round"></span>
          </label>
        </div>
      </div>

      <div className="card">
        <h2>Transition Duration</h2>
        <p>How long should standard fades take?</p>
        <div className="form-group">
          <label className="form-label" htmlFor="duration-select">Duration</label>
          <select 
            id="duration-select"
            value={config.transition.duration_ms} 
            onChange={(e) => updateConfig(d => { d.transition.duration_ms = parseInt(e.target.value, 10); })}
            disabled={!config.transition.enabled}
          >
            <option value={100}>100ms (Fast)</option>
            <option value={500}>500ms (Balanced)</option>
            <option value={1000}>1000ms (Smooth)</option>
          </select>
        </div>
      </div>
    </div>
  );
};
