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
  context?: string;
}

export interface EventLogEntry {
  timestamp_ms: number;
  category: string;
  description: string;
  before_value: string | null;
  after_value: string | null;
}

export interface HardwareCapabilities {
  brightness_api: string;
  brightness_available: boolean;
  ambient_sensor: string;
  ambient_available: boolean;
  internal_display: boolean;
  failure_reason: string | null;
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
  history: {
    lux: { time: number; value: number }[];
    screen: { time: number; value: number }[];
    brightness: { time: number; value: number }[];
  };
  activeTab: string;
  onboardingCompleted: boolean;
  developerMode: boolean;
  error: ErrorState | null;
  eventLog: EventLogEntry[];
  hardwareCapabilities: HardwareCapabilities | null;
  
  // Actions
  setDashboard: (dashboard: DashboardState) => void;
  setActiveTab: (tab: string) => void;
  setOnboardingCompleted: (val: boolean) => void;
  setDeveloperMode: (val: boolean) => void;
  setError: (val: ErrorState | null) => void;
  clearError: () => void;
  setEventLog: (entries: EventLogEntry[]) => void;
  setHardwareCapabilities: (caps: HardwareCapabilities) => void;
}

export const useStore = create<PixelSenseState>((set) => ({
  dashboard: null,
  history: {
    lux: [],
    screen: [],
    brightness: [],
  },
  activeTab: 'Overview',
  onboardingCompleted: localStorage.getItem('pixelsense-onboarding') === 'true',
  developerMode: localStorage.getItem('pixelsense-dev-mode') === 'true',
  error: null,
  eventLog: [],
  hardwareCapabilities: null,
  
  setDashboard: (dashboard) => set((state) => {
    const now = Date.now();
    const newHistory = { ...state.history };
    
    // Only keep last 30 data points
    const MAX_POINTS = 30;
    
    if (dashboard.ambient.lux !== null) {
        newHistory.lux = [...state.history.lux, { time: now, value: dashboard.ambient.lux }].slice(-MAX_POINTS);
    }
    
    if (dashboard.screen.average_luminance !== null) {
        newHistory.screen = [...state.history.screen, { time: now, value: dashboard.screen.average_luminance }].slice(-MAX_POINTS);
    }
    
    if (dashboard.brightness.current !== null) {
        newHistory.brightness = [...state.history.brightness, { time: now, value: dashboard.brightness.current }].slice(-MAX_POINTS);
    }
    
    return { dashboard, history: newHistory };
  }),
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
  setEventLog: (entries) => set({ eventLog: entries }),
  setHardwareCapabilities: (caps) => set({ hardwareCapabilities: caps }),
}))
