import { invoke } from '@tauri-apps/api/core';
import type { DashboardState } from '../store/useStore';

export const DashboardService = {
  /**
   * Fetches the complete real-time state of the PixelSense intelligence layer.
   */
  async getDashboardState(): Promise<DashboardState> {
    try {
      if (window.__TAURI_INTERNALS__) {
        return await invoke<DashboardState>('get_dashboard_state');
      }
      throw new Error('Tauri API not available');
    } catch (error) {
      console.warn('Tauri API unreachable. Cannot fetch dashboard state.');
      throw error;
    }
  }
};
