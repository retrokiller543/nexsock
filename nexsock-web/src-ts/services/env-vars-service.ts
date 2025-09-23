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

/**
 * Add a default environment variable for a service
 */
export function addEnvVar(serviceName: string): void {
  const key = prompt('Environment variable name:');
  if (!key) return;
  
  const value = prompt('Environment variable value:');
  if (value === null) return; // User cancelled
  
  // Send to backend to add the environment variable
  fetch(`/api/services/${serviceName}/env`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({ key, value })
  })
  .then(response => {
    if (response.ok) {
      showMessage(`Environment variable '${key}' added`, 'success');
      // Refresh the page to show updated data
      window.location.reload();
    } else {
      showMessage(`Failed to add environment variable '${key}'`, 'error');
    }
  })
  .catch(() => {
    showMessage(`Failed to add environment variable '${key}'`, 'error');
  });
}

/**
 * Edit a default environment variable for a service
 */
export function editEnvVar(serviceName: string, key: string, currentValue: string): void {
  const newValue = prompt(`Edit environment variable '${key}':`, currentValue);
  if (newValue === null || newValue === currentValue) return; // User cancelled or no change
  
  // Send to backend to update the environment variable
  fetch(`/api/services/${serviceName}/env/${key}`, {
    method: 'PUT',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({ value: newValue })
  })
  .then(response => {
    if (response.ok) {
      showMessage(`Environment variable '${key}' updated`, 'success');
      // Refresh the page to show updated data
      window.location.reload();
    } else {
      showMessage(`Failed to update environment variable '${key}'`, 'error');
    }
  })
  .catch(() => {
    showMessage(`Failed to update environment variable '${key}'`, 'error');
  });
}

/**
 * Delete a default environment variable for a service
 */
export function deleteEnvVar(serviceName: string, key: string): void {
  if (!confirm(`Delete environment variable '${key}'?`)) return;
  
  // Send to backend to delete the environment variable
  fetch(`/api/services/${serviceName}/env/${key}`, {
    method: 'DELETE'
  })
  .then(response => {
    if (response.ok) {
      showMessage(`Environment variable '${key}' deleted`, 'success');
      // Refresh the page to show updated data
      window.location.reload();
    } else {
      showMessage(`Failed to delete environment variable '${key}'`, 'error');
    }
  })
  .catch(() => {
    showMessage(`Failed to delete environment variable '${key}'`, 'error');
  });
}

/**
 * Add an alias for a service
 */
export function addAlias(serviceName: string): void {
  const alias = prompt('Alias name:');
  if (!alias) return;
  
  // Send to backend to add the alias
  fetch(`/api/services/${serviceName}/aliases`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({ alias })
  })
  .then(response => {
    if (response.ok) {
      showMessage(`Alias '${alias}' added`, 'success');
      // Refresh the page to show updated data
      window.location.reload();
    } else {
      showMessage(`Failed to add alias '${alias}'`, 'error');
    }
  })
  .catch(() => {
    showMessage(`Failed to add alias '${alias}'`, 'error');
  });
}

/**
 * Remove an alias for a service
 */
export function removeAlias(serviceName: string, alias: string): void {
  if (!confirm(`Remove alias '${alias}'?`)) return;
  
  // Send to backend to remove the alias
  fetch(`/api/services/${serviceName}/aliases/${alias}`, {
    method: 'DELETE'
  })
  .then(response => {
    if (response.ok) {
      showMessage(`Alias '${alias}' removed`, 'success');
      // Refresh the page to show updated data
      window.location.reload();
    } else {
      showMessage(`Failed to remove alias '${alias}'`, 'error');
    }
  })
  .catch(() => {
    showMessage(`Failed to remove alias '${alias}'`, 'error');
  });
}
