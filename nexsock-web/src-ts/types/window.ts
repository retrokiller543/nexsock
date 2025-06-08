/**
 * Window interface extensions for Nexsock Web Interface
 */

import {NexsockAPI} from './api';

declare global {
  interface Window {
    htmx: {
      ajax: (method: string, url: string, options?: {
        target?: string;
        swap?: string;
        values?: Record<string, string>;
      }) => void;
    };
    nexsock: NexsockAPI;
  }
}

export {};
