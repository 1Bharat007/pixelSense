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
      return [];
    } catch (error) {
      console.error('Failed to fetch history:', error);
      return [];
    }
  }
};
