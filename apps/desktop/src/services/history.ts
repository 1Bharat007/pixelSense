import { invoke } from '@tauri-apps/api/core';

export interface HistoryEvent {
  id: string;
  timestamp: number;
  category: string;
  description: string;
  before_value?: string;
  after_value?: string;
}

export const HistoryService = {
  async getHistory(): Promise<HistoryEvent[]> {
    try {
      if (window.__TAURI_INTERNALS__) {
        return await invoke<HistoryEvent[]>('get_history');
      }
      throw new Error('Tauri API not available');
    } catch (error) {
      console.warn('Using mock history data for preview.');
      const now = Date.now();
      return [
        {
          id: 'evt_1',
          timestamp: now - 1000 * 60 * 2,
          category: 'Brightness',
          description: 'Auto-adjusted due to room lighting change (cloud cover)',
          before_value: '80%',
          after_value: '65%'
        },
        {
          id: 'evt_2',
          timestamp: now - 1000 * 60 * 15,
          category: 'Comfort',
          description: 'Compensated for bright on-screen content (IDE to Browser)',
          before_value: '65%',
          after_value: '50%'
        },
        {
          id: 'evt_3',
          timestamp: now - 1000 * 60 * 45,
          category: 'Profile',
          description: 'Automatically engaged "Productivity" profile based on active application',
          before_value: 'Gaming',
          after_value: 'Productivity'
        },
        {
          id: 'evt_4',
          timestamp: now - 1000 * 60 * 120,
          category: 'System',
          description: 'PixelSense started up and calibrated to ambient lighting',
          before_value: 'Off',
          after_value: 'Active'
        }
      ];
    }
  }
};
