/**
 * Environment variable management service for Nexsock
 * Handles getting, setting, and managing environment variables for services
 */

import {showMessage} from '../ui/messages';

/**
 * Gets the current environment variables from the form
 */
export function getCurrentEnvVars(serviceName: string): Record<string, string> {
  const envVars: Record<string, string> = {};
  const container = document.getElementById(`env-vars-${serviceName}`);

  if (container) {
    container.querySelectorAll('.env-var-pair').forEach(pair => {
      const inputs = pair.querySelectorAll<HTMLInputElement>('input');
      const [keyInput, valueInput] = inputs;
      if (keyInput?.value) {
        envVars[keyInput.value] = valueInput?.value || '';
      }
    });
  }
  return envVars;
}

/**
 * Applies environment variables to the form using HTMX
 */
export function applyEnvVarsToForm(serviceName: string, envVars: Record<string, string>): void {
  const container = document.getElementById(`env-vars-${serviceName}`);
  if (!container) return;

  // Clear existing variables
  container.innerHTML = '';

  // Load environment variables using HTMX
  Object.entries(envVars).forEach(([key, value]) => {
    window.htmx.ajax('GET', `/api/templates/env-var-pair?key=${encodeURIComponent(key)}&value=${encodeURIComponent(value)}`, {
      target: `#env-vars-${serviceName}`,
      swap: 'beforeend'
    });
  });

  // Add one empty pair for additional variables
  window.htmx.ajax('GET', '/api/templates/env-var-pair', {
    target: `#env-vars-${serviceName}`,
    swap: 'beforeend'
  });
}

/**
 * Clears all current environment variables
 */
export function clearCurrentEnvVars(serviceName: string): void {
  const container = document.getElementById(`env-vars-${serviceName}`);
  if (!container) return;

  if (confirm('Clear all current environment variables?')) {
    container.innerHTML = '';
    // Add one empty pair
    window.htmx.ajax('GET', '/api/templates/env-var-pair', {
      target: `#env-vars-${serviceName}`,
      swap: 'beforeend'
    });
    showMessage('Environment variables cleared', 'info');
  }
}
