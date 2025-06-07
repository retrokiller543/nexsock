/**
 * Enhanced error display utilities for Nexsock UI
 * Extracts and displays detailed error information from server responses
 */

import {MessageType} from '../types/ui';
import {showMessage} from './messages';

export interface ErrorDetails {
  errorCode: string;
  errorMessage: string;
  diagnostics: string;
  debugInfo?: string;
  originalUrl?: string;
  fullErrorPageUrl?: string;
}

/**
 * Creates a modal overlay to display detailed error information
 */
export function showErrorModal(errorDetails: ErrorDetails): void {
  // Remove any existing error modal
  const existingModal = document.querySelector('.error-modal-overlay');
  if (existingModal) {
    existingModal.remove();
  }

  // Create modal overlay
  const overlay = document.createElement('div');
  overlay.className = 'error-modal-overlay modal-overlay';
  overlay.innerHTML = `
    <div class="error-modal modal">
      <div class="error-modal-header">
        <h2>🚨 Error Details</h2>
        <button class="close-button" onclick="this.closest('.error-modal-overlay').remove()">×</button>
      </div>
      <div class="error-modal-body">
        <div class="error-code">${escapeHtml(errorDetails.errorCode)}</div>
        <div class="error-message">${escapeHtml(errorDetails.errorMessage)}</div>
        <div class="error-diagnostics">
          <h3>🔍 Diagnostics</h3>
          <div class="diagnostics-content">${errorDetails.diagnostics}</div>
        </div>
        ${errorDetails.debugInfo ? `
          <div class="error-debug">
            <button class="debug-toggle" onclick="toggleErrorDebug(this)">🐛 Show Debug Info</button>
            <div class="debug-content" style="display: none;">
              <pre>${escapeHtml(errorDetails.debugInfo)}</pre>
            </div>
          </div>
        ` : ''}
      </div>
      <div class="error-modal-footer">
        ${errorDetails.fullErrorPageUrl ? `
          <button class="button button-primary" onclick="window.open('${errorDetails.fullErrorPageUrl}', '_blank')">
            🔍 View Full Error Page
          </button>
        ` : ''}
        ${isDebugMode() ? `
          <button class="button button-warning" onclick="navigateToErrorPage('${errorDetails.originalUrl || ''}')">
            🚧 Debug Mode: Go to Error Page
          </button>
        ` : ''}
        <button class="button button-secondary" onclick="this.closest('.error-modal-overlay').remove()">Close</button>
      </div>
    </div>
  `;

  document.body.appendChild(overlay);

  // Make debug toggle function globally available
  (window as any).toggleErrorDebug = (button: HTMLElement) => {
    const debugContent = button.nextElementSibling as HTMLElement;
    if (debugContent.style.display === 'none') {
      debugContent.style.display = 'block';
      button.textContent = '🐛 Hide Debug Info';
    } else {
      debugContent.style.display = 'none';
      button.textContent = '🐛 Show Debug Info';
    }
  };
}

/**
 * Shows an inline error notification in the messages container
 */
export function showInlineError(errorDetails: ErrorDetails): void {
  const container = getOrCreateMessagesContainer();
  
  // Create enhanced error message element
  const errorEl = document.createElement('div');
  errorEl.className = 'message message-error enhanced-error';
  errorEl.innerHTML = `
    <div class="error-summary">
      <strong>${escapeHtml(errorDetails.errorCode)}</strong>
      <p>${escapeHtml(errorDetails.errorMessage)}</p>
      <button class="error-details-button" onclick="showErrorDetailsModal(this)">View Details</button>
    </div>
  `;

  // Store error details on the element for the modal
  (errorEl as any).errorDetails = errorDetails;
  
  container.appendChild(errorEl);

  // Auto-remove after 10 seconds (longer than regular messages)
  setTimeout(() => {
    if (errorEl.parentNode) {
      errorEl.parentNode.removeChild(errorEl);
    }
  }, 10000);
}

/**
 * Parses an error response HTML and extracts error details
 */
export function parseErrorResponse(responseText: string): ErrorDetails | null {
  try {
    // Create a temporary DOM to parse the response
    const parser = new DOMParser();
    const doc = parser.parseFromString(responseText, 'text/html');

    // Extract error information from the error page structure
    const errorCodeEl = doc.querySelector('.error-code');
    const errorMessageEl = doc.querySelector('.error-message');
    const diagnosticsEl = doc.querySelector('.error-details');
    const debugOutputEl = doc.querySelector('.debug-output');

    if (!errorCodeEl || !errorMessageEl) {
      return null; // Not a recognized error page format
    }

    return {
      errorCode: errorCodeEl.textContent?.trim() || 'UNKNOWN_ERROR',
      errorMessage: errorMessageEl.textContent?.trim() || 'An unknown error occurred',
      diagnostics: diagnosticsEl?.innerHTML || 'No diagnostic information available',
      debugInfo: debugOutputEl?.textContent || undefined
    };
  } catch (e) {
    console.error('Failed to parse error response:', e);
    return null;
  }
}


/**
 * Helper function to get or create the messages container
 */
function getOrCreateMessagesContainer(): HTMLElement {
  let container = document.getElementById('messages-container');
  if (!container) {
    container = document.createElement('div');
    container.id = 'messages-container';
    container.className = 'messages';
    
    // Insert after the main navigation if it exists
    const main = document.querySelector('main');
    if (main && main.parentNode) {
      main.parentNode.insertBefore(container, main);
    } else {
      document.body.appendChild(container);
    }
  }
  return container;
}

/**
 * Helper function to escape HTML content
 */
function escapeHtml(text: string): string {
  const div = document.createElement('div');
  div.textContent = text;
  return div.innerHTML;
}

/**
 * Checks if the application is running in debug mode
 */
function isDebugMode(): boolean {
  // Check for debug indicators
  return !!(
    // URL parameter
    new URLSearchParams(window.location.search).has('debug') ||
    // Local storage flag
    localStorage.getItem('nexsock-debug') === 'true' ||
    // Development environment indicators
    window.location.hostname === 'localhost' ||
    window.location.hostname === '127.0.0.1' ||
    // Console debug flag
    (window as any).NEXSOCK_DEBUG === true
  );
}

/**
 * Creates a special URL to trigger the same error for debugging
 */
function createErrorPageUrl(originalUrl: string): string {
  const url = new URL(originalUrl, window.location.origin);
  url.searchParams.set('debug-error', 'true');
  return url.toString();
}

/**
 * Navigates to the full error page for debugging
 */
function navigateToErrorPage(originalUrl: string): void {
  if (originalUrl) {
    const errorPageUrl = createErrorPageUrl(originalUrl);
    window.location.href = errorPageUrl;
  } else {
    showMessage('No original URL available for error page navigation', 'warning');
  }
}

/**
 * Handles HTMX response errors with enhanced debug mode support
 */
export function handleHTMXErrorWithDebug(xhr: XMLHttpRequest, requestUrl?: string): void {
  // In debug mode, optionally navigate directly to error page
  if (isDebugMode()) {
    const shouldAutoRedirect = localStorage.getItem('nexsock-debug-auto-redirect') === 'true';
    
    if (shouldAutoRedirect && requestUrl) {
      console.log('Debug mode: Auto-redirecting to error page');
      navigateToErrorPage(requestUrl);
      return;
    }
  }

  // Otherwise, use enhanced error display
  handleHTMXError(xhr, requestUrl);
}

/**
 * Enhanced HTMX error handler with URL tracking
 */
export function handleHTMXError(xhr: XMLHttpRequest, requestUrl?: string): void {
  // First try to parse the response as a rich error page
  if (xhr.responseText) {
    const errorDetails = parseErrorResponse(xhr.responseText);
    if (errorDetails) {
      // Add URL information for debug navigation
      errorDetails.originalUrl = requestUrl;
      errorDetails.fullErrorPageUrl = requestUrl ? createErrorPageUrl(requestUrl) : undefined;
      
      // Show enhanced error display
      showInlineError(errorDetails);
      return;
    }
  }

  // Fallback to status-based error messages
  let errorMessage = 'An error occurred while processing your request';
  let errorType: MessageType = 'error';

  switch (xhr.status) {
    case 400:
      errorMessage = 'Bad request - please check your input and try again';
      break;
    case 401:
      errorMessage = 'Authentication required - please log in';
      break;
    case 403:
      errorMessage = 'Access denied - you don\'t have permission for this action';
      break;
    case 404:
      errorMessage = 'The requested resource was not found';
      break;
    case 429:
      errorMessage = 'Too many requests - please wait a moment and try again';
      errorType = 'warning';
      break;
    case 500:
      errorMessage = 'Internal server error - please try again later';
      break;
    case 502:
    case 503:
    case 504:
      errorMessage = 'Service temporarily unavailable - please try again later';
      break;
    default:
      if (xhr.status >= 400) {
        errorMessage = `Request failed with status ${xhr.status}`;
      }
  }

  showMessage(errorMessage, errorType);
}

/**
 * Global function to show error details modal (called from inline error buttons)
 */
(window as any).showErrorDetailsModal = (button: HTMLElement) => {
  const errorEl = button.closest('.enhanced-error') as any;
  if (errorEl && errorEl.errorDetails) {
    showErrorModal(errorEl.errorDetails);
  }
};

/**
 * Global function to navigate to error page (called from modal buttons)
 */
(window as any).navigateToErrorPage = navigateToErrorPage;