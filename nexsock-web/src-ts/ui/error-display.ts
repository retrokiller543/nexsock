/**
 * Enhanced error display utilities for Nexsock UI
 * Extracts and displays detailed error information from server responses
 */

import {MessageType} from '../types/ui';
import {showMessage} from './messages';
import {ErrorModal} from '../components/ErrorModal';
import {ErrorNotification} from '../components/ErrorNotification';

export interface ErrorDetails {
  errorCode: string;
  errorMessage: string;
  diagnostics: string;
  debugInfo?: string;
  originalUrl?: string;
  fullErrorPageUrl?: string;
}

/**
 * Creates a modal overlay to display detailed error information using JSX
 */
export function showErrorModal(errorDetails: ErrorDetails): void {
  // Remove any existing error modal
  const existingModal = document.querySelector('.error-modal-overlay');
  if (existingModal) {
    existingModal.remove();
  }

  // Create modal using JSX component
  const modalElement = ErrorModal({ 
    errorDetails,
    onClose: () => {
      const modal = document.querySelector('.error-modal-overlay');
      if (modal) {
        modal.remove();
      }
    }
  }) as unknown as HTMLElement;

  document.body.appendChild(modalElement);
}

/**
 * Shows an inline error notification in the messages container using JSX
 */
export function showInlineError(errorDetails: ErrorDetails): void {
  const container = getOrCreateMessagesContainer();
  
  // Create enhanced error notification using JSX component
  const errorElement = ErrorNotification({ 
    errorDetails,
    onViewDetails: () => showErrorModal(errorDetails),
    onClose: () => {
      const elem = errorElement as HTMLElement;
      if (elem.parentNode) {
        elem.parentNode.removeChild(elem);
      }
    }
  }) as unknown as HTMLElement;
  
  container.appendChild(errorElement);

  // Auto-remove after 10 seconds (longer than regular messages)
  setTimeout(() => {
    if (errorElement.parentNode) {
      errorElement.parentNode.removeChild(errorElement);
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
  return (
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
    window.location.href = createErrorPageUrl(originalUrl);
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
 * Global function to navigate to error page (called from modal buttons)
 */
(window as any).navigateToErrorPage = navigateToErrorPage;