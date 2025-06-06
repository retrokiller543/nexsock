/**
 * Event-related type definitions for Nexsock Web Interface
 */

// Event-related types
export interface HTMXEvent extends Event {
  detail: {
    xhr?: XMLHttpRequest;
    target?: HTMLElement;
    requestConfig?: {
      elt: HTMLElement;
      path: string;
      verb: string;
    };
  };
}