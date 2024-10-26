import { toaster, ToastData } from '@decky/api';
import { log, error } from './logger';

export const setupNotifications = () => {
  const handleMessage = (e: MessageEvent) => {
    if (e.type !== 'text' || typeof e.data !== 'string') {
      error('Unexpected message type', e.type);
      return;
    }

    const toastData: ToastData = {
      title: "Controller Tools",
      body: e.data,
      showToast: true,
    }

    toaster.toast(toastData);
  }

  const setupWebsocket = (): void => {
    const ws = new WebSocket('ws://localhost:33220/ws');

    ws.onopen = () => {
      log('WebSocket connected');
    };

    ws.onmessage = (e: MessageEvent) => {
      handleMessage(e);
    };

    ws.onclose = (e: CloseEvent) => {
      log('Socket is closed. Reconnect will be attempted in 10 seconds.', e.reason);
      setTimeout(() => {
        setupWebsocket();
      }, 10000);
    };

    ws.onerror = (err: Event) => {
      error('Socket encountered error: ', (err as ErrorEvent).message, 'Closing socket');
      ws.close();
    };
  }

  setupWebsocket();
}