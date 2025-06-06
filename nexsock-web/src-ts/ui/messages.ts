/**
 * Message handling utilities for Nexsock UI
 */

import {MessageType} from '../types/ui';

/**
 * Shows a temporary message to the user
 */
export function showMessage(message: string, type: MessageType = 'info'): void {
  // Create message element
  const messageEl = document.createElement('div');
  messageEl.className = `message message-${type}`;
  messageEl.textContent = message;

  // Add to messages container or create one
  let container = document.getElementById('messages-container');
  if (!container) {
    container = document.createElement('div');
    container.id = 'messages-container';
    container.className = 'messages';
    document.body.appendChild(container);
  }

  container.appendChild(messageEl);

  // Auto-remove after 5 seconds
  setTimeout(() => {
    if (messageEl.parentNode) {
      messageEl.parentNode.removeChild(messageEl);
    }
  }, 5000);
}
