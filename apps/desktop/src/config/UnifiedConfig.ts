export interface AppConfig {
  onboarding: {
    completed: boolean;
    version: number;
    last_completed: number;
  };
  adaptive: {
    enabled: boolean;
    confidence_threshold: number;
  };
  transition: {
    enabled: boolean;
    duration_ms: number;
  };
  brightness: {
    manual_override_timeout_ms: number;
  };
  appearance: {
    theme: 'System' | 'Light' | 'Dark';
  };
  performance: {
    mode: 'PowerSaving' | 'Balanced' | 'Performance';
  };
}

export const defaultConfig: AppConfig = {
  onboarding: { completed: false, version: 1, last_completed: 0 },
  adaptive: { enabled: true, confidence_threshold: 0.5 },
  transition: { enabled: true, duration_ms: 500 },
  brightness: { manual_override_timeout_ms: 3600000 },
  appearance: { theme: 'System' },
  performance: { mode: 'Balanced' },
};

