/**
 * Main TypeScript file for Nexsock Web Interface
 * Uses HTMX-first approach with minimal vanilla TS for enhanced functionality
 */

import {HTMXEvent, MessageType, NexsockAPI, ServiceConfig, ServiceConfigs, STORAGE_KEYS} from './types.js';
import {createServiceCard} from "./example.tsx";

// ===============================================
// Configuration Management with localStorage
// ===============================================

/**
 * Saves environment variable configuration for a service to localStorage
 */
function saveServiceConfig(
  serviceName: string, 
  configName: string, 
  envVars: Record<string, string>, 
  description: string = ''
): void {
  const key = STORAGE_KEYS.SERVICE_CONFIG(serviceName);
  const configs = getServiceConfigs(serviceName);
  
  configs[configName] = {
    envVars,
    description,
    lastUsed: new Date().toISOString(),
    created: configs[configName]?.created || new Date().toISOString()
  };
  
  localStorage.setItem(key, JSON.stringify(configs));
  console.log(`Saved configuration '${configName}' for service '${serviceName}'`);
}

/**
 * Gets all saved configurations for a service
 */
function getServiceConfigs(serviceName: string): ServiceConfigs {
  const key = STORAGE_KEYS.SERVICE_CONFIG(serviceName);
  const stored = localStorage.getItem(key);
  return stored ? JSON.parse(stored) : {};
}

/**
 * Loads a specific configuration for a service
 */
function loadServiceConfig(serviceName: string, configName: string): ServiceConfig | null {
  const configs = getServiceConfigs(serviceName);
  return configs[configName] || null;
}

/**
 * Deletes a configuration for a service
 */
function deleteServiceConfig(serviceName: string, configName: string): boolean {
  const key = STORAGE_KEYS.SERVICE_CONFIG(serviceName);
  const configs = getServiceConfigs(serviceName);
  
  if (configs[configName]) {
    delete configs[configName];
    localStorage.setItem(key, JSON.stringify(configs));
    console.log(`Deleted configuration '${configName}' for service '${serviceName}'`);
    return true;
  }
  return false;
}

// ===============================================
// Service Management Helpers
// ===============================================

/**
 * Gets the current environment variables from the form
 */
function getCurrentEnvVars(serviceName: string): Record<string, string> {
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
function applyEnvVarsToForm(serviceName: string, envVars: Record<string, string>): void {
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
function clearCurrentEnvVars(serviceName: string): void {
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
 * Loads a configuration from selection
 */
function loadConfigFromSelection(serviceName: string, configName: string): void {
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
function showSaveConfigModal(serviceName: string): void {
  const envVars = getCurrentEnvVars(serviceName);
  
  if (Object.keys(envVars).length === 0) {
    showMessage('Please add some environment variables before saving a configuration.', 'warning');
    return;
  }
  
  const configName = prompt('Enter a name for this configuration:');
  if (!configName) return;
  
  const description = prompt('Enter a description (optional):') || '';
  
  saveServiceConfig(serviceName, configName, envVars, description);
  refreshConfigUI(serviceName);
  showMessage(`Configuration '${configName}' saved successfully!`, 'success');
}

/**
 * Refreshes the configuration UI components using HTMX
 */
function refreshConfigUI(serviceName: string): void {
  window.htmx.ajax('GET', `/api/templates/config-section?service=${encodeURIComponent(serviceName)}`, {
    target: `#config-section-${serviceName}`,
    swap: 'innerHTML'
  });
}

/**
 * Deletes a configuration and refreshes the modal
 */
function deleteConfigAndRefresh(serviceName: string, configName: string): void {
  if (confirm(`Are you sure you want to delete the configuration '${configName}'?`)) {
    deleteServiceConfig(serviceName, configName);
    // Refresh the modal content
    window.htmx.ajax('GET', `/api/templates/config-modal-content?service=${encodeURIComponent(serviceName)}`, {
      target: '.modal-body',
      swap: 'innerHTML'
    });
    // Also refresh the main config UI
    refreshConfigUI(serviceName);
    showMessage(`Configuration '${configName}' deleted successfully.`, 'success');
  }
}

// ===============================================
// UI Helpers
// ===============================================

/**
 * Toggles the visibility of service management sections
 */
function toggleManagement(serviceName: string): void {
  const managementDiv = document.getElementById(`management-${serviceName}`);
  if (managementDiv) {
    const isHidden = managementDiv.style.display === 'none';
    managementDiv.style.display = isHidden ? 'block' : 'none';
  }
}

/**
 * Closes any open modal
 */
function closeModal(): void {
  const modal = document.querySelector<HTMLElement>('.modal-overlay');
  if (modal) {
    modal.remove();
  }
}

/**
 * Shows a temporary message to the user
 */
function showMessage(message: string, type: MessageType = 'info'): void {
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

/**
 * Confirms service removal with better UX
 */
async function confirmRemove(serviceName: string): Promise<void> {
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

// ===============================================
// Git Operations
// ===============================================

/**
 * Shows a specific git tab (commits or branches)
 */
function showGitTab(tabName: 'commits' | 'branches', serviceName: string): void {
  // Update tab button states
  document.querySelectorAll('.tab-button').forEach(btn => {
    btn.classList.remove('active');
  });
  
  // Find and activate the clicked tab button
  const clickedTab = (event as any)?.target as HTMLElement;
  if (clickedTab) {
    clickedTab.classList.add('active');
  }
  
  // Load the appropriate content
  const tabContent = document.getElementById('git-tab-content');
  if (!tabContent) return;
  
  if (tabName === 'commits') {
    tabContent.innerHTML = '<div class="loading">Loading commits...</div>';
    window.htmx.ajax('GET', `/api/templates/git-log?service=${serviceName}`, {
      target: '#git-tab-content',
      swap: 'innerHTML'
    });
  } else if (tabName === 'branches') {
    tabContent.innerHTML = '<div class="loading">Loading branches...</div>';
    window.htmx.ajax('GET', `/api/templates/git-branches?service=${serviceName}`, {
      target: '#git-tab-content',
      swap: 'innerHTML'
    });
  }
}

/**
 * Creates a new git branch
 */
function createNewBranch(serviceName: string): void {
  const input = document.getElementById('new-branch-name') as HTMLInputElement;
  if (!input) return;
  
  const branchName = input.value.trim();
  if (!branchName) {
    showMessage('Please enter a branch name', 'warning');
    return;
  }
  
  if (!confirm(`Create new branch "${branchName}" and switch to it?`)) {
    return;
  }
  
  // Use fetch to create the branch
  const formData = new FormData();
  formData.append('branch', branchName);
  formData.append('create', 'true');
  
  fetch(`/api/services/${serviceName}/git/checkout/branch`, {
    method: 'POST',
    body: formData
  })
  .then(response => {
    if (!response.ok) {
      throw new Error(`HTTP error: ${response.status}`);
    }
    return response.json();
  })
  .then(data => {
    // Clear the input
    input.value = '';
    
    // Refresh the git section
    window.htmx.ajax('GET', `/api/templates/git-section?service=${serviceName}`, {
      target: '#git-section',
      swap: 'outerHTML'
    });
    
    showMessage(`Successfully created and switched to branch "${branchName}"`, 'success');
  })
  .catch(error => {
    console.error('Error creating branch:', error);
    showMessage('Failed to create branch', 'error');
  });
}

/**
 * Refreshes the git section for a service
 */
function refreshGitSection(serviceName: string): void {
  window.htmx.ajax('GET', `/api/templates/git-section?service=${serviceName}`, {
    target: '#git-section',
    swap: 'outerHTML'
  });
}

/**
 * Toggles git content visibility (commits or branches)
 */
function toggleGitContent(contentId: string): void {
  const content = document.getElementById(contentId);
  if (!content) return;
  
  content.classList.toggle('collapsed');
  
  // Update local storage to remember user preference
  const isCollapsed = content.classList.contains('collapsed');
  localStorage.setItem(STORAGE_KEYS.GIT_CONTENT_COLLAPSED(contentId), isCollapsed.toString());
}

/**
 * Restores git content visibility from user preferences
 */
function restoreGitContentVisibility(): void {
  // Restore commits visibility
  const commitsCollapsed = localStorage.getItem(STORAGE_KEYS.GIT_CONTENT_COLLAPSED('git-commits-list')) === 'true';
  const commitsList = document.getElementById('git-commits-list');
  if (commitsList && commitsCollapsed) {
    commitsList.classList.add('collapsed');
  }
  
  // Restore branches visibility
  const branchesCollapsed = localStorage.getItem(STORAGE_KEYS.GIT_CONTENT_COLLAPSED('git-branches-list')) === 'true';
  const branchesList = document.getElementById('git-branches-list');
  if (branchesList && branchesCollapsed) {
    branchesList.classList.add('collapsed');
  }
}

// ===============================================
// Dropdown Management
// ===============================================

/**
 * Toggles dropdown menu visibility
 */
function toggleDropdown(dropdownId: string): void {
  const dropdown = document.getElementById(dropdownId);
  if (!dropdown) return;
  
  // Close all other dropdowns first
  document.querySelectorAll('.dropdown.active').forEach(dd => {
    if (dd.id !== dropdownId) {
      dd.classList.remove('active');
    }
  });
  
  // Toggle this dropdown
  dropdown.classList.toggle('active');
}

/**
 * Closes all open dropdowns
 */
function closeAllDropdowns(): void {
  document.querySelectorAll('.dropdown.active').forEach(dropdown => {
    dropdown.classList.remove('active');
  });
}

// ===============================================
// Initialization and Event Handling
// ===============================================

// Initialize when page loads
document.addEventListener('DOMContentLoaded', () => {
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

// Export functions for global access
const nexsockAPI: NexsockAPI = {
  saveServiceConfig,
  getServiceConfigs,
  loadServiceConfig,
  deleteServiceConfig,
  getCurrentEnvVars,
  applyEnvVarsToForm,
  loadConfigFromSelection,
  showSaveConfigModal,
  refreshConfigUI,
  deleteConfigAndRefresh,
  toggleManagement,
  closeModal,
  showMessage,
  confirmRemove,
  showGitTab,
  createNewBranch,
  refreshGitSection,
  toggleDropdown,
  closeAllDropdowns,
  clearCurrentEnvVars,
  toggleGitContent,
  restoreGitContentVisibility
};

// Make API available globally
window.nexsock = nexsockAPI;
window.createServiceCard = createServiceCard;

// Test function for TSX functionality
window.testTSX = function() {
  const container = document.getElementById('tsx-test-container');
  if (container) {
    const serviceCard = createServiceCard('Test Service', 'running', 3000);
    container.appendChild(serviceCard);
    console.log('TSX test executed successfully!');
  }
};