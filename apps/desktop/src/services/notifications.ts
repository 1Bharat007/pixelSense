import { invoke } from '@tauri-apps/api/core';

export interface NotificationEvent {
  id: string;
  timestamp: number;
  priority: string;
  title: string;
  message: string;
  read: boolean;
  action_type?: string;
}

export const NotificationService = {
  async getNotifications(): Promise<NotificationEvent[]> {
    try {
      if (window.__TAURI_INTERNALS__) {
        return await invoke<NotificationEvent[]>('get_notifications');
      }
      return [];
    } catch (error) {
      console.error('Failed to fetch notifications:', error);
      return [];
    }
  }
};
