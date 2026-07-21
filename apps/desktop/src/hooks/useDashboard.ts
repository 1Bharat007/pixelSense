import { useEffect } from 'react';
import { useStore } from '../store/useStore';
import { DashboardService } from '../services/dashboard';
import { invoke } from '@tauri-apps/api/core';
import type { EventLogEntry } from '../store/useStore';

export function useDashboard(pollIntervalMs = 500) {
  const setDashboard = useStore((state) => state.setDashboard);
  const setError = useStore((state) => state.setError);
  const setEventLog = useStore((state) => state.setEventLog);

  useEffect(() => {
    let active = true;
    let timeoutId: number;
    let errorCount = 0;

    const fetchDashboard = async () => {
      // Don't poll if window is hidden (e.g., in tray)
      if (document.visibilityState === 'hidden') {
        timeoutId = window.setTimeout(fetchDashboard, pollIntervalMs);
        return;
      }

      try {
        const state = await DashboardService.getDashboardState();
        if (active) {
          setDashboard(state);
          setError(null);
          errorCount = 0;
        }
      } catch (error) {
        console.error('Error fetching dashboard state:', error);
        errorCount++;
        if (active) {
          setError({
            code: "ERR_BACKEND_UNREACHABLE",
            title: "Background Service Unreachable",
            description: "PixelSense lost connection to the native hardware worker. It may have crashed or stalled.",
            recoverable: true
          });
        }
      }

      // Fetch event log on a slower cadence (every 5 polls).
      if (active && errorCount === 0) {
        try {
          const events = await invoke<EventLogEntry[]>('get_event_log');
          if (active) setEventLog(events);
        } catch {
          // Non-critical — don't escalate to error state.
        }
      }

      if (active) {
        const nextDelay = errorCount > 0 
          ? Math.min(pollIntervalMs * Math.pow(2, errorCount), 10000)
          : pollIntervalMs;
        timeoutId = window.setTimeout(fetchDashboard, nextDelay);
      }
    };

    const handleVisibilityChange = () => {
      if (document.visibilityState === 'visible') {
        clearTimeout(timeoutId);
        fetchDashboard();
      }
    };

    document.addEventListener('visibilitychange', handleVisibilityChange);
    fetchDashboard();

    return () => {
      active = false;
      clearTimeout(timeoutId);
      document.removeEventListener('visibilitychange', handleVisibilityChange);
    };
  }, [pollIntervalMs, setDashboard]);
}
