import { useConfigStore } from '../store/configStore';

export const General = () => {
  const { config, updateConfig } = useConfigStore();

  const handleThemeChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    const val = e.target.value as 'System' | 'Light' | 'Dark';
    updateConfig(draft => { draft.appearance.theme = val; });
  };

  return (
    <div>
      <h1>General Settings</h1>
      <div className="card">
        <h2>Appearance</h2>
        <p>Choose the color theme for the PixelSense application.</p>
        <div className="form-group">
          <label className="form-label" htmlFor="theme-select">Theme</label>
          <select id="theme-select" value={config.appearance.theme} onChange={handleThemeChange}>
            <option value="System">System Default</option>
            <option value="Light">Light</option>
            <option value="Dark">Dark</option>
          </select>
        </div>
      </div>
    </div>
  );
};
