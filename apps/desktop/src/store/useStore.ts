import { create } from 'zustand'

export interface ComfortState {
  status: 'Comfortable' | 'Adjusting' | 'Strained';
  recommendation: string;
  confidence: number;
}

export interface EngineHealth {
  nativeSensorActive: boolean;
  screenEngineActive: boolean;
  performanceMode: 'AC' | 'BatteryHigh' | 'BatteryLow' | 'BatterySaver';
}

interface PixelSenseState {
  comfort: ComfortState;
  health: EngineHealth;
  activeProfile: string;
  
  // Actions
  setComfort: (comfort: ComfortState) => void;
  setHealth: (health: EngineHealth) => void;
  setActiveProfile: (profile: string) => void;
}

export const useStore = create<PixelSenseState>((set) => ({
  comfort: {
    status: 'Comfortable',
    recommendation: 'Optimal viewing conditions.',
    confidence: 1.0,
  },
  health: {
    nativeSensorActive: true,
    screenEngineActive: true,
    performanceMode: 'AC',
  },
  activeProfile: 'Productivity',
  
  setComfort: (comfort) => set({ comfort }),
  setHealth: (health) => set({ health }),
  setActiveProfile: (profile) => set({ activeProfile: profile }),
}))
