async function startService(serviceId) {
    try {
        const btn = document.querySelector(`[data-service-id="${serviceId}"]`);
        btn.disabled = true;
        btn.textContent = 'Starting...';

        const response = await fetch('/api/services/start', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ serviceId })
        });

        if (!response.ok) throw new Error('Failed to start service');

        const service = btn.closest('.service');
        const status = service.querySelector('.status');
        status.textContent = 'Starting service...';
        status.style.background = '#fff3dc';

    } catch (error) {
        console.error('Error:', error);
        const btn = document.querySelector(`[data-service-id="${serviceId}"]`);
        btn.disabled = false;
        btn.textContent = `Start ${serviceId}`;
        alert('Failed to start service');
    }
}