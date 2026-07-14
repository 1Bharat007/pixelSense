import { create } from 'zustand'

export interface ComfortState {
  status: string;
  recommendation: string;
  confidence: number | null;
  active_profile: string;
  mode: string;
}

export interface AmbientState {
  lux: number | null;
  environment: string;
  health: string;
  confidence: number | null;
  source: string;
}

export interface ScreenState {
  average_luminance: number | null;
  peak_luminance: number | null;
  visual_complexity: number | null;
  current_analysis_time_ms: number | null;
}

export interface BrightnessState {
  current: number | null;
  target: number | null;
  transition_status: string;
  transition_progress: number | null;
  eye_comfort_score: number | null;
}

export interface PerformanceState {
  cpu_usage_pct: number | null;
  ram_usage_mb: number | null;
  current_poll_interval_ms: number | null;
  battery_mode: string;
  power_state: string;
  pipeline_duration_ms: number | null;
}

export interface EngineHealthState {
  background_worker: string;
  watchdog: string;
  ambient_engine: string;
  screen_engine: string;
  comfort_engine: string;
  transition_engine: string;
}

export interface DashboardState {
  comfort: ComfortState;
  ambient: AmbientState;
  screen: ScreenState;
  brightness: BrightnessState;
  performance: PerformanceState;
  health: EngineHealthState;
  intelligence?: any;
}

export interface ErrorState {
  code: string;
  title: string;
  description: string;
  recoverable: boolean;
}

interface PixelSenseState {
  dashboard: DashboardState | null;
  activeTab: string;
  onboardingCompleted: boolean;
  developerMode: boolean;
  error: ErrorState | null;
  
  // Actions
  setDashboard: (dashboard: DashboardState) => void;
  setActiveTab: (tab: string) => void;
  setOnboardingCompleted: (val: boolean) => void;
  setDeveloperMode: (val: boolean) => void;
  setError: (val: ErrorState | null) => void;
  clearError: () => void;
}

export const useStore = create<PixelSenseState>((set) => ({
  dashboard: null,
  activeTab: 'Overview',
  onboardingCompleted: localStorage.getItem('pixelsense-onboarding') === 'true',
  developerMode: localStorage.getItem('pixelsense-dev-mode') === 'true',
  error: null,
  
  setDashboard: (dashboard) => set({ dashboard }),
  setActiveTab: (tab) => set({ activeTab: tab }),
  setOnboardingCompleted: (val) => {
    localStorage.setItem('pixelsense-onboarding', val.toString());
    set({ onboardingCompleted: val });
  },
  setDeveloperMode: (val) => {
    localStorage.setItem('pixelsense-dev-mode', val.toString());
    set({ developerMode: val });
  },
  setError: (val) => set({ error: val }),
  clearError: () => set({ error: null }),
}))
