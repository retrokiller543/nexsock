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

// ===============================================
// Git Operations
// ===============================================

/**
 * Shows git operations modal
 * @param {string} serviceName - The name of the service
 */
function showGitModal(serviceName) {
    htmx.ajax('GET', `/api/templates/git-modal?service=${encodeURIComponent(serviceName)}`, {
        target: 'body',
        swap: 'beforeend'
    });
}

/**
 * Refreshes git status for a service
 * @param {string} serviceName - The name of the service
 */
function refreshGitStatus(serviceName) {
    fetch(`/api/services/${serviceName}/git/status`)
        .then(response => response.json())
        .then(data => {
            // Convert the data to query parameters for the template
            const params = new URLSearchParams(data).toString();
            htmx.ajax('GET', `/api/templates/git-status?${params}`, {
                target: `#git-section-${serviceName} .git-status`,
                swap: 'innerHTML'
            });
        })
        .catch(error => {
            console.error('Failed to refresh git status:', error);
        });
}

/**
 * Shows a specific git tab in the modal
 * @param {string} tabName - The name of the tab to show
 */
function showGitTab(tabName) {
    // Hide all tabs
    document.querySelectorAll('.git-tab-content').forEach(tab => {
        tab.classList.remove('active');
    });
    document.querySelectorAll('.tab-button').forEach(button => {
        button.classList.remove('active');
    });
    
    // Show selected tab
    const tab = document.getElementById(`git-tab-${tabName}`);
    const button = document.querySelector(`[onclick="showGitTab('${tabName}')"]`);
    if (tab) tab.classList.add('active');
    if (button) button.classList.add('active');
}

/**
 * Checks out the selected branch
 * @param {string} serviceName - The name of the service
 */
function checkoutSelectedBranch(serviceName) {
    const selector = document.getElementById(`branch-selector-${serviceName}`);
    const branch = selector.value;
    
    if (!branch) {
        alert('Please select a branch to checkout.');
        return;
    }
    
    const formData = new FormData();
    formData.append('branch', branch);
    
    fetch(`/api/services/${serviceName}/git/checkout`, {
        method: 'POST',
        body: formData
    })
    .then(response => response.json())
    .then(data => {
        if (data.success) {
            alert(`Successfully checked out branch: ${branch}`);
            refreshGitStatus(serviceName);
            closeModal();
        } else {
            alert('Failed to checkout branch');
        }
    })
    .catch(error => {
        console.error('Error checking out branch:', error);
        alert('Failed to checkout branch');
    });
}

/**
 * Creates and checks out a new branch
 * @param {string} serviceName - The name of the service
 */
function createAndCheckoutBranch(serviceName) {
    const input = document.getElementById(`new-branch-name-${serviceName}`);
    const branchName = input.value.trim();
    
    if (!branchName) {
        alert('Please enter a branch name.');
        return;
    }
    
    const formData = new FormData();
    formData.append('branch', branchName);
    formData.append('create', 'true');
    
    fetch(`/api/services/${serviceName}/git/checkout`, {
        method: 'POST',
        body: formData
    })
    .then(response => response.json())
    .then(data => {
        if (data.success) {
            alert(`Successfully created and checked out branch: ${branchName}`);
            input.value = '';
            refreshGitStatus(serviceName);
            closeModal();
        } else {
            alert('Failed to create branch');
        }
    })
    .catch(error => {
        console.error('Error creating branch:', error);
        alert('Failed to create branch');
    });
}

/**
 * Loads commits for the git modal
 * @param {string} serviceName - The name of the service
 */
function loadCommits(serviceName) {
    const countSelect = document.getElementById(`commit-count-${serviceName}`);
    const maxCount = countSelect ? countSelect.value : 25;
    
    fetch(`/api/services/${serviceName}/git/log?max_count=${maxCount}`)
        .then(response => response.json())
        .then(data => {
            const container = document.getElementById(`commit-list-${serviceName}`);
            if (!container) return;
            
            if (data.commits && data.commits.length > 0) {
                let html = '<div class=\"commit-list-items\">';
                data.commits.forEach(commit => {
                    html += `
                        <div class=\"commit-item\">
                            <div class=\"commit-header\">
                                <code class=\"commit-hash\">${commit.short_hash}</code>
                                <span class=\"commit-author\">${commit.author_name}</span>
                                <span class=\"commit-date\">${new Date(commit.timestamp).toLocaleDateString()}</span>
                            </div>
                            <div class=\"commit-message\">${commit.message}</div>
                            <div class=\"commit-actions\">
                                <button class=\"button button-sm\" onclick=\"checkoutCommit('${serviceName}', '${commit.hash}')\">
                                    Checkout
                                </button>
                            </div>
                        </div>`;
                });
                html += '</div>';
                container.innerHTML = html;
            } else {
                container.innerHTML = '<p class=\"text-muted\">No commits found.</p>';
            }
        })
        .catch(error => {
            console.error('Failed to load commits:', error);
            const container = document.getElementById(`commit-list-${serviceName}`);
            if (container) {
                container.innerHTML = '<p class=\"text-error\">Failed to load commits.</p>';
            }
        });
}

/**
 * Checks out a specific commit
 * @param {string} serviceName - The name of the service
 * @param {string} commitHash - The commit hash to checkout
 */
function checkoutCommit(serviceName, commitHash) {
    if (!confirm(`Checkout commit ${commitHash.substring(0, 8)}? This will put the repository in a detached HEAD state.`)) {
        return;
    }
    
    const formData = new FormData();
    formData.append('commit_hash', commitHash);
    
    fetch(`/api/services/${serviceName}/git/checkout-commit`, {
        method: 'POST',
        body: formData
    })
    .then(response => response.json())
    .then(data => {
        if (data.success) {
            alert(`Successfully checked out commit: ${commitHash.substring(0, 8)}`);
            refreshGitStatus(serviceName);
            closeModal();
        } else {
            alert('Failed to checkout commit');
        }
    })
    .catch(error => {
        console.error('Error checking out commit:', error);
        alert('Failed to checkout commit');
    });
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
        // Use HTMX to load the git log template
        htmx.ajax('GET', `/api/templates/git-log?service=${serviceName}`, {
            target: '#git-tab-content',
            swap: 'innerHTML'
        });
    } else if (tabName === 'branches') {
        tabContent.innerHTML = '<div class="loading">Loading branches...</div>';
        // Use HTMX to load the git branches template
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
        alert('Please enter a branch name');
        return;
    }
    
    if (!confirm(`Create new branch "${branchName}" and switch to it?`)) {
        return;
    }
    
    // Use HTMX to create the branch
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
        
        alert(`Successfully created and switched to branch "${branchName}"`);
    })
    .catch(error => {
        console.error('Error creating branch:', error);
        alert('Failed to create branch');
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
 * Shows the git modal (keeping existing functionality)
 * @param {string} serviceName - The name of the service
 */
function showGitModal(serviceName) {
    // This function can be enhanced later for more complex git operations
    // For now, we have the structured components in the main view
    console.log('Git modal for:', serviceName);
    
    // Load the git modal template
    htmx.ajax('GET', `/api/templates/git-modal?service=${serviceName}`, {
        target: 'body',
        swap: 'beforeend'
    });
}
