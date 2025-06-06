/**
 * Service management utilities for Nexsock
 * Handles service-specific operations
 */

import {showMessage} from '../ui/messages';
import {loadServiceConfig} from './config-service';
import {applyEnvVarsToForm} from './env-vars-service';

/**
 * Toggles the visibility of service management sections
 */
export function toggleManagement(serviceName: string): void {
  const managementDiv = document.getElementById(`management-${serviceName}`);
  if (managementDiv) {
    const isHidden = managementDiv.style.display === 'none';
    managementDiv.style.display = isHidden ? 'block' : 'none';
  }
}

/**
 * Loads a configuration from selection
 */
export function loadConfigFromSelection(serviceName: string, configName: string): void {
  if (!configName) return;

  const config = loadServiceConfig(serviceName, configName);
  if (config) {
    applyEnvVarsToForm(serviceName, config.envVars);
    console.log(`Loaded configuration '${configName}' for service '${serviceName}'`);
  }
}

/**
 * Shows a modal to save current environment variables as a configuration
 */
export function showSaveConfigModal(serviceName: string): void {
  const envVars = window.nexsock.getCurrentEnvVars(serviceName);

  if (Object.keys(envVars).length === 0) {
    showMessage('Please add some environment variables before saving a configuration.', 'warning');
    return;
  }

  const configName = prompt('Enter a name for this configuration:');
  if (!configName) return;

  const description = prompt('Enter a description (optional):') || '';

  window.nexsock.saveServiceConfig(serviceName, configName, envVars, description);
  window.nexsock.refreshConfigUI(serviceName);
  showMessage(`Configuration '${configName}' saved successfully!`, 'success');
}

/**
 * Refreshes the configuration UI components using HTMX
 */
export function refreshConfigUI(serviceName: string): void {
  window.htmx.ajax('GET', `/api/templates/config-section?service=${encodeURIComponent(serviceName)}`, {
    target: `#config-section-${serviceName}`,
    swap: 'innerHTML'
  });
}

/**
 * Deletes a configuration and refreshes the modal
 */
export function deleteConfigAndRefresh(serviceName: string, configName: string): void {
  if (confirm(`Are you sure you want to delete the configuration '${configName}'?`)) {
    window.nexsock.deleteServiceConfig(serviceName, configName);
    // Refresh the modal content
    window.htmx.ajax('GET', `/api/templates/config-modal-content?service=${encodeURIComponent(serviceName)}`, {
      target: '.modal-body',
      swap: 'innerHTML'
    });
    // Also refresh the main config UI
    window.nexsock.refreshConfigUI(serviceName);
    showMessage(`Configuration '${configName}' deleted successfully.`, 'success');
  }
}

/**
 * Confirms service removal with better UX
 */
export async function confirmRemove(serviceName: string): Promise<void> {
  if (!serviceName) {
    showMessage('Invalid service name', 'error');
    return;
  }

  if (confirm(`Are you sure you want to remove ${serviceName}? This action cannot be undone.`)) {
    try {
      const response = await fetch(`/api/services/${serviceName}`, {
        method: 'DELETE'
      });

      if (!response.ok) {
        throw new Error(`HTTP error: ${response.status}`);
      }

      showMessage(`Service '${serviceName}' removed successfully.`, 'success');

      // Navigate back to services list
      window.location.href = '/';
    } catch (error) {
      console.error('Error removing service:', error);
      showMessage('Failed to remove service', 'error');
    }
  }
}