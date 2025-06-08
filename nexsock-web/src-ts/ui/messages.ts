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
  const timeoutId = setTimeout(() => {
    messageEl.remove();
  }, 5000);
  
  // Allow manual removal to clear timeout
  messageEl.addEventListener('click', () => {
    clearTimeout(timeoutId);
    messageEl.remove();
  });
}
