/**
 * Modal handling utilities for Nexsock UI
 */

/**
 * Closes any open modal
 */
export function closeModal(): void {
  const modal = document.querySelector<HTMLElement>('.modal-overlay');
  if (modal) {
    modal.remove();
  }
}