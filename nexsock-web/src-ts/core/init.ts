/**
 * Initialization and event handling for Nexsock Web Interface
 */

import {HTMXEvent} from '../types';
import {showMessage} from '../ui/messages';
import {closeModal} from '../ui/modals';
import {closeAllDropdowns} from '../ui/dropdowns';
import {restoreGitContentVisibility} from '../services/git-service';
import {refreshConfigUI} from '../services/service-management';

/**
 * Initialize the application when the DOM is loaded
 */
export function initializeApp(): void {
  // Initialize config UI for all services
  document.querySelectorAll<HTMLElement>('[data-service-name]').forEach(element => {
    const serviceName = element.getAttribute('data-service-name');
    if (serviceName) {
      refreshConfigUI(serviceName);
    }
  });

  // Restore git content visibility preferences
  restoreGitContentVisibility();

  // Add global error handler for HTMX
  document.body.addEventListener('htmx:responseError', (event: Event) => {
    const htmxEvent = event as HTMXEvent;
    console.error('HTMX Error:', htmxEvent.detail);
    showMessage('An error occurred while loading content', 'error');
  });

  // Add loading indicator for HTMX requests
  document.body.addEventListener('htmx:beforeRequest', (event: Event) => {
    const target = event.target as HTMLElement;
    if (target.classList.contains('button')) {
      target.classList.add('button-loading');
    }
  });

  document.body.addEventListener('htmx:afterRequest', (event: Event) => {
    const target = event.target as HTMLElement;
    if (target.classList.contains('button')) {
      target.classList.remove('button-loading');
    }

    // Restore git content visibility after HTMX updates
    restoreGitContentVisibility();
  });

  // Handle click outside modal to close
  document.addEventListener('click', (event: Event) => {
    const target = event.target as HTMLElement;
    if (target.classList.contains('modal-overlay')) {
      closeModal();
    }

    // Close dropdowns when clicking outside
    if (!target.closest('.dropdown')) {
      closeAllDropdowns();
    }
  });

  // Handle ESC key to close modal and dropdowns
  document.addEventListener('keydown', (event: KeyboardEvent) => {
    if (event.key === 'Escape') {
      closeModal();
      closeAllDropdowns();
    }
  });
}