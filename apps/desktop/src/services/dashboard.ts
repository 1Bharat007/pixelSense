import { invoke } from '@tauri-apps/api/core';
import type { DashboardState } from '../store/useStore';

export const DashboardService = {
  /**
   * Fetches the complete real-time state of the PixelSense intelligence layer.
   */
  async getDashboardState(): Promise<DashboardState> {
    try {
      return await invoke<DashboardState>('get_dashboard_state');
    } catch (error) {
      console.error('Failed to fetch dashboard state via IPC:', error);
      throw error;
    }
  }
};
