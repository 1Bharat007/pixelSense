import { useEffect } from 'react';
import { useStore } from '../store/useStore';
import { DashboardService } from '../services/dashboard';

export function useDashboard(pollIntervalMs = 500) {
  const setDashboard = useStore((state) => state.setDashboard);
  const setError = useStore((state) => state.setError);

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
          errorCount = 0; // reset on success
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

      if (active) {
        // Exponential backoff up to 10 seconds on consecutive errors
        const nextDelay = errorCount > 0 
          ? Math.min(pollIntervalMs * Math.pow(2, errorCount), 10000)
          : pollIntervalMs;
        timeoutId = window.setTimeout(fetchDashboard, nextDelay);
      }
    };

    const handleVisibilityChange = () => {
      if (document.visibilityState === 'visible') {
        // Immediately fetch when becoming visible
        clearTimeout(timeoutId);
        fetchDashboard();
      }
    };

    document.addEventListener('visibilitychange', handleVisibilityChange);
    
    // Initial fetch triggers the loop
    fetchDashboard();

    return () => {
      active = false;
      clearTimeout(timeoutId);
      document.removeEventListener('visibilitychange', handleVisibilityChange);
    };
  }, [pollIntervalMs, setDashboard]);
}
