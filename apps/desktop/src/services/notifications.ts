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
      throw new Error('Tauri API not available');
    } catch (error) {
      console.warn('Using mock notifications for preview.');
      const now = Date.now();
      return [
        {
          id: 'notif_1',
          timestamp: now - 1000 * 60 * 5,
          priority: 'High',
          title: 'Direct Sunlight Detected',
          message: 'Ambient sensors detected sudden bright light. Brightness temporarily locked to 100%.',
          read: false,
          action_type: 'Review',
        },
        {
          id: 'notif_2',
          timestamp: now - 1000 * 60 * 60 * 3,
          priority: 'Normal',
          title: 'Comfort Profile Engaged',
          message: 'Transitioned to "Evening Comfort" profile to reduce blue light exposure.',
          read: true,
        },
        {
          id: 'notif_3',
          timestamp: now - 1000 * 60 * 60 * 24,
          priority: 'Normal',
          title: 'Weekly Analytics Report',
          message: 'PixelSense optimized your screen brightness 342 times this week.',
          read: true,
          action_type: 'View Analytics',
        }
      ];
    }
  }
};
