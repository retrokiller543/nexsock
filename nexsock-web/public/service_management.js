async function controlService(serviceName, action) {
    const endpoint = `/api/service/${serviceName}/${action}`;
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
 * Adds a new input pair for environment variables to the DOM.
 *
 * This function creates a new div element containing two text input fields for a key and a value,
 * along with a button that removes the pair when clicked. The new environment variable pair is
 * appended to the container element with the ID 'env-vars'.
 */
function addEnvVar() {
    const container = document.getElementById('env-vars');
    if (!container) {
        console.error('Environment variables container not found');
        return;
    }
    const pair = document.createElement('div');
    pair.className = 'env-var-pair';

    const keyInput = document.createElement('input');
    keyInput.type = 'text';
    keyInput.className = 'form-input';
    keyInput.placeholder = 'Key';

    const valueInput = document.createElement('input');
    valueInput.type = 'text';
    valueInput.className = 'form-input';
    valueInput.placeholder = 'Value';

    const removeButton = document.createElement('button');
    removeButton.type = 'button';
    removeButton.className = 'button button-icon';
    removeButton.textContent = '×';
    removeButton.onclick = function() { this.parentElement.remove(); };

    pair.appendChild(keyInput);
    pair.appendChild(valueInput);
    pair.appendChild(removeButton);
    container.appendChild(pair);
}

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
            const endpoint = `/api/service/${serviceName}`;
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
