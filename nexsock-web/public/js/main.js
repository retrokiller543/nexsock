/**
 * Main JavaScript file for Nexsock Web Interface
 * Uses HTMX-first approach with minimal vanilla JS for enhanced functionality
 */

// ===============================================
// Configuration Management with localStorage
// ===============================================

/**
 * Saves environment variable configuration for a service to localStorage
 * @param {string} serviceName - The name of the service
 * @param {string} configName - The name of the configuration template
 * @param {Object} envVars - Object containing environment variables {key: value}
 * @param {string} description - Optional description for the configuration
 */
function saveServiceConfig(serviceName, configName, envVars, description = '') {
    const key = `nexsock_service_config_${serviceName}`;
    let configs = getServiceConfigs(serviceName);
    
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
 * @param {string} serviceName - The name of the service
 * @returns {Object} Object containing all configurations for the service
 */
function getServiceConfigs(serviceName) {
    const key = `nexsock_service_config_${serviceName}`;
    const stored = localStorage.getItem(key);
    return stored ? JSON.parse(stored) : {};
}

/**
 * Loads a specific configuration for a service
 * @param {string} serviceName - The name of the service
 * @param {string} configName - The name of the configuration to load
 * @returns {Object|null} The configuration object or null if not found
 */
function loadServiceConfig(serviceName, configName) {
    const configs = getServiceConfigs(serviceName);
    return configs[configName] || null;
}

/**
 * Deletes a configuration for a service
 * @param {string} serviceName - The name of the service
 * @param {string} configName - The name of the configuration to delete
 */
function deleteServiceConfig(serviceName, configName) {
    const key = `nexsock_service_config_${serviceName}`;
    let configs = getServiceConfigs(serviceName);
    
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
 * @param {string} serviceName - The name of the service
 * @returns {Object} Object containing current environment variables
 */
function getCurrentEnvVars(serviceName) {
    const envVars = {};
    const container = document.getElementById(`env-vars-${serviceName}`);
    if (container) {
        container.querySelectorAll('.env-var-pair').forEach(pair => {
            const [keyInput, valueInput] = pair.querySelectorAll('input');
            if (keyInput && keyInput.value) {
                envVars[keyInput.value] = valueInput ? valueInput.value : '';
            }
        });
    }
    return envVars;
}

/**
 * Applies environment variables to the form using HTMX
 * @param {string} serviceName - The name of the service
 * @param {Object} envVars - Object containing environment variables to apply
 */
function applyEnvVarsToForm(serviceName, envVars) {
    const container = document.getElementById(`env-vars-${serviceName}`);
    if (!container) return;
    
    // Clear existing variables
    container.innerHTML = '';
    
    // Load environment variables using HTMX
    Object.entries(envVars).forEach(([key, value]) => {
        htmx.ajax('GET', `/api/templates/env-var-pair?key=${encodeURIComponent(key)}&value=${encodeURIComponent(value)}`, {
            target: `#env-vars-${serviceName}`,
            swap: 'beforeend'
        });
    });
    
    // Add one empty pair for additional variables
    htmx.ajax('GET', '/api/templates/env-var-pair', {
        target: `#env-vars-${serviceName}`,
        swap: 'beforeend'
    });
}

/**
 * Clears all current environment variables
 * @param {string} serviceName - The name of the service
 */
function clearCurrentEnvVars(serviceName) {
    const container = document.getElementById(`env-vars-${serviceName}`);
    if (!container) return;
    
    if (confirm('Clear all current environment variables?')) {
        container.innerHTML = '';
        // Add one empty pair
        htmx.ajax('GET', '/api/templates/env-var-pair', {
            target: `#env-vars-${serviceName}`,
            swap: 'beforeend'
        });
        showMessage('Environment variables cleared', 'info');
    }
}

/**
 * Loads a configuration from selection
 * @param {string} serviceName - The name of the service
 * @param {string} configName - The name of the configuration to load
 */
function loadConfigFromSelection(serviceName, configName) {
    if (!configName) return;
    
    const config = loadServiceConfig(serviceName, configName);
    if (config) {
        applyEnvVarsToForm(serviceName, config.envVars);
        console.log(`Loaded configuration '${configName}' for service '${serviceName}'`);
    }
}

/**
 * Shows a modal to save current environment variables as a configuration
 * @param {string} serviceName - The name of the service
 */
function showSaveConfigModal(serviceName) {
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
 * @param {string} serviceName - The name of the service
 */
function refreshConfigUI(serviceName) {
    htmx.ajax('GET', `/api/templates/config-section?service=${encodeURIComponent(serviceName)}`, {
        target: `#config-section-${serviceName}`,
        swap: 'innerHTML'
    });
}

/**
 * Deletes a configuration and refreshes the modal
 * @param {string} serviceName - The name of the service
 * @param {string} configName - The name of the configuration to delete
 */
function deleteConfigAndRefresh(serviceName, configName) {
    if (confirm(`Are you sure you want to delete the configuration '${configName}'?`)) {
        deleteServiceConfig(serviceName, configName);
        // Refresh the modal content
        htmx.ajax('GET', `/api/templates/config-modal-content?service=${encodeURIComponent(serviceName)}`, {
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
 * @param {string} serviceName - The name of the service
 */
function toggleManagement(serviceName) {
    const managementDiv = document.getElementById(`management-${serviceName}`);
    if (managementDiv) {
        const isHidden = managementDiv.style.display === 'none';
        managementDiv.style.display = isHidden ? 'block' : 'none';
    }
}

/**
 * Closes any open modal
 */
function closeModal() {
    const modal = document.querySelector('.modal-overlay');
    if (modal) {
        modal.remove();
    }
}

/**
 * Shows a temporary message to the user
 * @param {string} message - The message to show
 * @param {string} type - The type of message (success, error, warning, info)
 */
function showMessage(message, type = 'info') {
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
 * @param {string} serviceName - The name of the service to remove
 */
async function confirmRemove(serviceName) {
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
 * @param {string} tabName - The name of the tab to show ('commits' or 'branches')
 * @param {string} serviceName - The name of the service
 */
function showGitTab(tabName, serviceName) {
    // Update tab button states
    document.querySelectorAll('.tab-button').forEach(btn => {
        btn.classList.remove('active');
    });
    
    // Find and activate the clicked tab button
    const clickedTab = event?.target;
    if (clickedTab) {
        clickedTab.classList.add('active');
    }
    
    // Load the appropriate content
    const tabContent = document.getElementById('git-tab-content');
    if (!tabContent) return;
    
    if (tabName === 'commits') {
        tabContent.innerHTML = '<div class="loading">Loading commits...</div>';
        htmx.ajax('GET', `/api/templates/git-log?service=${serviceName}`, {
            target: '#git-tab-content',
            swap: 'innerHTML'
        });
    } else if (tabName === 'branches') {
        tabContent.innerHTML = '<div class="loading">Loading branches...</div>';
        htmx.ajax('GET', `/api/templates/git-branches?service=${serviceName}`, {
            target: '#git-tab-content',
            swap: 'innerHTML'
        });
    }
}

/**
 * Creates a new git branch
 * @param {string} serviceName - The name of the service
 */
function createNewBranch(serviceName) {
    const input = document.getElementById('new-branch-name');
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
        htmx.ajax('GET', `/api/templates/git-section?service=${serviceName}`, {
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
 * @param {string} serviceName - The name of the service
 */
function refreshGitSection(serviceName) {
    htmx.ajax('GET', `/api/templates/git-section?service=${serviceName}`, {
        target: '#git-section',
        swap: 'outerHTML'
    });
}

/**
 * Toggles git content visibility (commits or branches)
 * @param {string} contentId - The ID of the content to toggle
 */
function toggleGitContent(contentId) {
    const content = document.getElementById(contentId);
    if (!content) return;
    
    content.classList.toggle('collapsed');
    
    // Update local storage to remember user preference
    const isCollapsed = content.classList.contains('collapsed');
    localStorage.setItem(`git_${contentId}_collapsed`, isCollapsed);
}

/**
 * Restores git content visibility from user preferences
 */
function restoreGitContentVisibility() {
    // Restore commits visibility
    const commitsCollapsed = localStorage.getItem('git_git-commits-list_collapsed') === 'true';
    const commitsList = document.getElementById('git-commits-list');
    if (commitsList && commitsCollapsed) {
        commitsList.classList.add('collapsed');
    }
    
    // Restore branches visibility
    const branchesCollapsed = localStorage.getItem('git_git-branches-list_collapsed') === 'true';
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
 * @param {string} dropdownId - The ID of the dropdown element
 */
function toggleDropdown(dropdownId) {
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
function closeAllDropdowns() {
    document.querySelectorAll('.dropdown.active').forEach(dropdown => {
        dropdown.classList.remove('active');
    });
}

// ===============================================
// Initialization and Event Handling
// ===============================================

// Initialize when page loads
document.addEventListener('DOMContentLoaded', function() {
    // Initialize config UI for all services
    document.querySelectorAll('[data-service-name]').forEach(element => {
        const serviceName = element.getAttribute('data-service-name');
        refreshConfigUI(serviceName);
    });
    
    // Restore git content visibility preferences
    restoreGitContentVisibility();
    
    // Add global error handler for HTMX
    document.body.addEventListener('htmx:responseError', function(event) {
        console.error('HTMX Error:', event.detail);
        showMessage('An error occurred while loading content', 'error');
    });
    
    // Add loading indicator for HTMX requests
    document.body.addEventListener('htmx:beforeRequest', function(event) {
        const target = event.target;
        if (target.classList.contains('button')) {
            target.classList.add('button-loading');
        }
    });
    
    document.body.addEventListener('htmx:afterRequest', function(event) {
        const target = event.target;
        if (target.classList.contains('button')) {
            target.classList.remove('button-loading');
        }
        
        // Restore git content visibility after HTMX updates
        restoreGitContentVisibility();
    });
});

// Handle click outside modal to close
document.addEventListener('click', function(event) {
    if (event.target.classList.contains('modal-overlay')) {
        closeModal();
    }
    
    // Close dropdowns when clicking outside
    if (!event.target.closest('.dropdown')) {
        closeAllDropdowns();
    }
});

// Handle ESC key to close modal and dropdowns
document.addEventListener('keydown', function(event) {
    if (event.key === 'Escape') {
        closeModal();
        closeAllDropdowns();
    }
});

// Export functions for global access
window.nexsock = {
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