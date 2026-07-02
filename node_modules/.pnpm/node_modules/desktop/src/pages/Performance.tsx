import { useConfigStore } from '../store/configStore';

export const Performance = () => {
  const { config, updateConfig } = useConfigStore();

  const handleModeChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    const val = e.target.value as 'PowerSaving' | 'Balanced' | 'Performance';
    updateConfig(draft => { draft.performance.mode = val; });
  };

  return (
    <div>
      <h1>Performance</h1>
      <div className="card">
        <h2>Execution Mode</h2>
        <p>Optimize background polling and sensor evaluation rates.</p>
        <div className="form-group">
          <label className="form-label" htmlFor="performance-select">Mode</label>
          <select id="performance-select" value={config.performance.mode} onChange={handleModeChange}>
            <option value="PowerSaving">Power Saving</option>
            <option value="Balanced">Balanced</option>
            <option value="Performance">Performance</option>
          </select>
        </div>
      </div>
    </div>
  );
};
