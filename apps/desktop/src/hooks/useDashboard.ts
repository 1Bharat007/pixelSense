import { useEffect } from 'react';
import { useStore } from '../store/useStore';
import { DashboardService } from '../services/dashboard';

export function useDashboard(pollIntervalMs = 500) {
  const setDashboard = useStore((state) => state.setDashboard);

  useEffect(() => {
    let active = true;

    const fetchDashboard = async () => {
      try {
        const state = await DashboardService.getDashboardState();
        if (active) {
          setDashboard(state);
        }
      } catch (error) {
        console.error('Error fetching dashboard state:', error);
      }
    };

    // Initial fetch
    fetchDashboard();

    // Poll interval
    const intervalId = setInterval(fetchDashboard, pollIntervalMs);

    return () => {
      active = false;
      clearInterval(intervalId);
    };
  }, [pollIntervalMs, setDashboard]);
}
