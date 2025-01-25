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

function addEnvVar() {
    const container = document.getElementById('env-vars');
    const pair = document.createElement('div');
    pair.className = 'env-var-pair';
    pair.innerHTML = `
            <input type="text" class="env-input" placeholder="Key">
            <input type="text" class="env-input" placeholder="Value">
            <button type="button" class="button button-remove" onclick="this.parentElement.remove()">×</button>
        `;
    container.appendChild(pair);
}

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