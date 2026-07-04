export interface SystemHealth {
  platform: string;
  engineStatus: string;
  sensorStatus: string;
  applicationVersion: string;
  overallHealth: 'Healthy' | 'Degraded' | 'Offline';
}

export interface AmbientState {
  currentLux: number;
  environment: string;
  confidence: number;
  sensorStatus: string;
}

export interface ScreenState {
  averageLuminance: number;
  peakLuminance: number;
}

export interface DisplayState {
  currentBrightness: number;
  recommendedBrightness: number;
  transitionStatus: string;
}

export interface OverviewState {
  hero: {
    comfortStatus: 'Comfortable' | 'Adjusting' | 'Attention' | 'Disabled';
    recommendation: string;
    reason: string;
    confidence: number;
  };
  ambient: AmbientState;
  screen: ScreenState;
  display: DisplayState;
  health: SystemHealth;
}
