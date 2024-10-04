import { toaster, ToastData } from '@decky/api';
import { log, error } from './logger';

export const setupNotifications = (): void => {
  const handleMessage = (e: MessageEvent): void => {
    let toastData: ToastData = {
      title: "Controller Tools",
      body: e.data,
      showToast: true
    }

    toaster.toast(toastData);
  }

  const setupWebsocket = (): void => {
    const ws = new WebSocket('ws://localhost:33220/ws');

    ws.onopen = (): void => {
      log('WebSocket connected');
    };

    ws.onmessage = (e: MessageEvent): void => {
      handleMessage(e);
    };

    ws.onclose = (e: CloseEvent): void => {
      log('Socket is closed. Reconnect will be attempted in 10 seconds.', e.reason);
      setTimeout(() => {
        setupWebsocket();
      }, 10000);
    };

    ws.onerror = (err: Event): void => {
      error('Socket encountered error: ', (err as ErrorEvent).message, 'Closing socket');
      ws.close();
    };
  }

  setupWebsocket();
}