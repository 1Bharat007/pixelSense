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
      console.warn('Using mock dashboard data for preview.');
      // Mock Data for GitHub Showcase Preview
      return {
        comfort: {
          status: 'Optimal',
          recommendation: 'Perfect for current ambient lighting',
          confidence: 0.96,
          active_profile: 'Productivity',
          mode: 'Auto',
        },
        ambient: {
          lux: 245,
          environment: 'Indoor Office',
          health: 'Healthy',
          confidence: 0.95,
          source: 'System Mock',
        },
        screen: {
          average_luminance: 0.82,
          peak_luminance: 1.0,
          visual_complexity: 0.65,
          current_analysis_time_ms: 12,
        },
        brightness: {
          current: 45,
          target: 45,
          transition_status: 'Idle',
          transition_progress: 1.0,
          eye_comfort_score: 9.2,
        },
        performance: {
          cpu_usage_pct: 1.2,
          ram_usage_mb: 24,
          current_poll_interval_ms: 500,
          battery_mode: 'High Performance',
          power_state: 'AC',
          pipeline_duration_ms: 22,
        },
        health: {
          background_worker: 'Running',
          watchdog: 'Active',
          ambient_engine: 'Online',
          screen_engine: 'Online',
          comfort_engine: 'Online',
          transition_engine: 'Online',
        },
        intelligence: {
          comfort_score: {
            total_score: 9.2,
          },
          recommendations: [
            {
              title: "Reduce Brightness",
              reason: "The ambient light is low (245 lux) and the current brightness might cause eye strain.",
              action: "Dim Screen",
            }
          ]
        }
      };
    }
  }
};
