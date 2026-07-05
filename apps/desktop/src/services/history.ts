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
      return await invoke<HistoryEvent[]>('get_history');
    } catch (error) {
      console.error('Failed to fetch history via IPC:', error);
      return [];
    }
  }
};
