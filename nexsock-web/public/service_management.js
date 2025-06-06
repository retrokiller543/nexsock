async function controlService(serviceName, action) {
    const endpoint = `/services/${serviceName}/${action}`;
    let options = {
        method: 'POST'
    };

    if (action === 'start') {
        const envVars = {};
        document.querySelectorAll('.env-var-pair').forEach(pair => {
            const [keyInput, valueInput] = pair.querySelectorAll('input');
            if (keyInput.value) {
                envVars[keyInput.value] = valueInput.value;
            }
        });
        options.body = JSON.stringify(envVars);
        options.headers = {
            'Content-Type': 'application/json'
        };
    }

    try {
        const response = await fetch(endpoint, options);
        if (!response.ok) throw new Error(`HTTP error: ${response.status}`);
        window.location.reload();
    } catch (error) {
        console.error('Error:', error);
        alert('Failed to control service');
    }
}

/**
 * Adds a new input pair for environment variables using HTMX
 * This is kept for backward compatibility but now uses HTMX for consistency
 */
function addEnvVar() {
    // Legacy function - try to find the container and use HTMX
    const container = document.getElementById('env-vars');
    if (container) {
        htmx.ajax('GET', '/api/templates/env-var-pair', {
            target: '#env-vars',
            swap: 'beforeend'
        });
        return;
    }
    console.error('Environment variables container not found');
}

/**
 * Function called by HTMX after successful requests to refresh the view
 * Currently just logs the action and reloads the page for simplicity
 */
function refreshView(element) {
    console.log('Refreshing view after successful request');
    // For now, just reload the page to ensure all data is fresh
    // In the future, we could do more targeted updates
    window.location.reload();
}

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
            if (keyInput.value) {
                envVars[keyInput.value] = valueInput.value || '';
            }
        });
    }
    return envVars;
}

/**
 * Applies environment variables to the form
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
 * Shows a modal to save current environment variables as a configuration
 * @param {string} serviceName - The name of the service
 */
function showSaveConfigModal(serviceName) {
    const envVars = getCurrentEnvVars(serviceName);
    
    if (Object.keys(envVars).length === 0) {
        alert('Please add some environment variables before saving a configuration.');
        return;
    }
    
    const configName = prompt('Enter a name for this configuration:');
    if (!configName) return;
    
    const description = prompt('Enter a description (optional):') || '';
    
    saveServiceConfig(serviceName, configName, envVars, description);
    refreshConfigUI(serviceName);
    alert(`Configuration '${configName}' saved successfully!`);
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
 * Refreshes the configuration UI components
 * @param {string} serviceName - The name of the service
 */
function refreshConfigUI(serviceName) {
    // Use HTMX to refresh the config section
    htmx.ajax('GET', `/api/templates/config-section?service=${encodeURIComponent(serviceName)}`, {
        target: `#config-section-${serviceName}`,
        swap: 'innerHTML'
    });
}

/**
 * Shows configuration management modal using HTMX
 * @param {string} serviceName - The name of the service
 */
function showConfigManagementModal(serviceName) {
    htmx.ajax('GET', `/api/templates/config-modal?service=${encodeURIComponent(serviceName)}`, {
        target: 'body',
        swap: 'beforeend'
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

// Initialize configuration system when page loads
document.addEventListener('DOMContentLoaded', function() {
    // Initialize config UI for all services
    document.querySelectorAll('[data-service-name]').forEach(element => {
        const serviceName = element.getAttribute('data-service-name');
        refreshConfigUI(serviceName);
    });
});

/**
 * Removes a service after user confirmation.
 *
 * This asynchronous function validates that the provided service name is non-empty, prompts the user for confirmation,
 * and if confirmed, sends a DELETE request to remove the service. On a successful response, it redirects the browser to 
 * the homepage. If an error occurs during the process (either due to an HTTP error or a network issue), it logs the error 
 * to the console and alerts the user.
 *
 * @param {string} serviceName - The name of the service to remove. Must be a non-empty string.
 * @throws {Error} Throws an error if the service name is empty.
 * @returns {Promise<void>} A promise that resolves when the service removal operation is complete.
 */
async function confirmRemove(serviceName) {
    if (serviceName.length === 0) {
        throw new Error('Invalid Service name');
    }

    if (confirm(`Are you sure you want to remove ${serviceName}? This action cannot be undone.`)) {
        try {
            const endpoint = `/api/services/${serviceName}`;
            console.info(endpoint)
            const response = await fetch(endpoint, {
                method: 'DELETE'
            });
            if (!response.ok) throw new Error(`HTTP error: ${response.status}`);
            window.location.href = '/';
        } catch (error) {
            console.error('Error:', error);
            alert('Failed to remove service');
        }
    }
}

/**
 * Toggles the visibility of the management section associated with a specified service.
 *
 * This function retrieves a DOM element whose ID is constructed as "management-{serviceName}" and checks its current 
 * display style. If the management section is hidden (i.e., its display style is 'none'), the function sets it to 
 * 'block' to make it visible; otherwise, it hides the section by setting its display style to 'none'.
 *
 * Note: The commented-out code suggests that there was originally an intention to hide all management sections before 
 * showing the selected one. This behavior is currently inactive.
 *
 * @param {string} serviceName - The name of the service for which the management section should be toggled.
 */
function toggleManagement(serviceName) {
    const managementDiv = document.getElementById(`management-${serviceName}`);
    const isHidden = managementDiv.style.display === 'none';

    // Hide all management sections first
    /*document.querySelectorAll('.service-card-management').forEach(div => {
        div.style.display = 'none';
    });

    document.querySelectorAll('.dependency-management').forEach(div => {
        div.style.display = 'none';
    });*/

    // Show the clicked one if it was hidden
    if (isHidden) {
        managementDiv.style.display = 'block';
    } else {
        managementDiv.style.display = 'none';
    }
}
