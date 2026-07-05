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
      return await invoke<NotificationEvent[]>('get_notifications');
    } catch (error) {
      console.error('Failed to fetch notifications via IPC:', error);
      return [];
    }
  }
};
