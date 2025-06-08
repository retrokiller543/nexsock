/**
 * ErrorModal JSX Component for displaying detailed error information
 */

import {ErrorDetails} from '../ui/error-display';

interface ErrorModalProps {
  errorDetails: ErrorDetails;
  onClose?: () => void;
}

/**
 * Enhanced Error Modal Component
 * Shows detailed error information with debug controls
 */
function ErrorModalComponent(props: ErrorModalProps): JSX.Element {
  const { errorDetails, onClose } = props;

  const handleClose = () => {
    if (onClose) {
      onClose();
    } else {
      // Fallback: remove the modal element
      const modal = document.querySelector('.error-modal-overlay');
      if (modal) {
        modal.remove();
      }
    }
  };

  const handleViewErrorPage = () => {
    if (errorDetails.fullErrorPageUrl) {
      window.open(errorDetails.fullErrorPageUrl, '_blank');
    }
  };

  const handleDebugNavigate = () => {
    if (errorDetails.originalUrl) {
      (window as any).navigateToErrorPage(errorDetails.originalUrl);
    }
  };

  const isDebugMode = () => {
    return !!(
      new URLSearchParams(window.location.search).has('debug') ||
      localStorage.getItem('nexsock-debug') === 'true' ||
      window.location.hostname === 'localhost' ||
      window.location.hostname === '127.0.0.1' ||
      (window as any).NEXSOCK_DEBUG === true
    );
  };

  return (
    <div className="error-modal-overlay modal-overlay" onClick={(e) => {
      if ((e.target as HTMLElement).classList.contains('error-modal-overlay')) {
        handleClose();
      }
    }}>
      <div className="error-modal modal">
        <div className="error-modal-header">
          <h2>🚨 Error Details</h2>
          <button className="close-button" onClick={handleClose}>×</button>
        </div>
        
        <div className="error-modal-body">
          <div className="error-code">{errorDetails.errorCode}</div>
          <div className="error-message">{errorDetails.errorMessage}</div>
          
          <div className="error-diagnostics">
            <h3>🔍 Diagnostics</h3>
            <div className="diagnostics-content" innerHTML={errorDetails.diagnostics} />
          </div>
          
          {errorDetails.debugInfo && (
            <div className="error-debug">
              <button 
                className="debug-toggle" 
                onClick={() => {
                  const buttons = document.querySelectorAll('.debug-toggle');
                  const button = buttons[buttons.length - 1] as HTMLElement;
                  const debugContent = button.nextElementSibling as HTMLElement;
                  if (debugContent.style.display === 'none') {
                    debugContent.style.display = 'block';
                    button.textContent = '🐛 Hide Debug Info';
                  } else {
                    debugContent.style.display = 'none';
                    button.textContent = '🐛 Show Debug Info';
                  }
                }}
              >
                🐛 Show Debug Info
              </button>
              <div className="debug-content" style={{ display: 'none' }}>
                <pre>{errorDetails.debugInfo}</pre>
              </div>
            </div>
          )}
        </div>
        
        <div className="error-modal-footer">
          {errorDetails.fullErrorPageUrl && (
            <button className="button button-primary" onClick={handleViewErrorPage}>
              🔍 View Full Error Page
            </button>
          )}
          
          {isDebugMode() && errorDetails.originalUrl && (
            <button className="button button-warning" onClick={handleDebugNavigate}>
              🚧 Debug Mode: Go to Error Page
            </button>
          )}
          
          <button className="button button-secondary" onClick={handleClose}>
            Close
          </button>
        </div>
      </div>
    </div>
  );
}

// CSS for the ErrorModal component
const css = `
.error-modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.6);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  backdrop-filter: blur(2px);
}

.error-modal {
  background: var(--surface-color);
  border-radius: 12px;
  border: 1px solid var(--error-color);
  max-width: 90vw;
  max-height: 90vh;
  width: 800px;
  overflow: hidden;
  box-shadow: 0 20px 40px rgba(0, 0, 0, 0.3);
  animation: errorModalSlideIn 0.3s ease-out;
}

@keyframes errorModalSlideIn {
  from {
    opacity: 0;
    transform: translateY(-20px) scale(0.95);
  }
  to {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
}

.error-modal-header {
  background: linear-gradient(90deg, var(--error-color) 0%, var(--error-hover-color) 100%);
  color: var(--surface-color);
  padding: 20px 30px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  border-bottom: 1px solid var(--border-color);
}

.error-modal-header h2 {
  margin: 0;
  font-size: 1.3em;
  font-weight: 600;
}

.close-button {
  background: none;
  border: none;
  color: var(--surface-color);
  font-size: 1.5em;
  cursor: pointer;
  padding: 0;
  width: 30px;
  height: 30px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
  transition: background-color 0.2s ease;
}

.close-button:hover {
  background: rgba(255, 255, 255, 0.2);
}

.error-modal-body {
  padding: 30px;
  overflow-y: auto;
  max-height: 60vh;
}

.error-code {
  background: var(--secondary-bg-color);
  color: var(--warning-color);
  padding: 6px 12px;
  border-radius: 6px;
  font-size: 0.9em;
  font-weight: bold;
  display: inline-block;
  margin-bottom: 15px;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
}

.error-message {
  font-size: 1.1em;
  margin-bottom: 25px;
  color: var(--text-color);
  line-height: 1.5;
}

.error-diagnostics {
  margin: 20px 0;
}

.error-diagnostics h3 {
  color: var(--primary-color);
  margin-bottom: 15px;
  font-size: 1.1em;
}

.diagnostics-content {
  background: var(--secondary-bg-color);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  padding: 20px;
  overflow-x: auto;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-size: 0.9em;
  line-height: 1.4;
  color: var(--text-color);
}

.error-debug {
  margin-top: 20px;
}

.debug-toggle {
  background: var(--secondary-color);
  color: var(--text-color);
  border: none;
  padding: 8px 16px;
  border-radius: 6px;
  cursor: pointer;
  font-family: inherit;
  font-size: 0.9em;
  transition: all 0.2s ease;
  margin-bottom: 15px;
}

.debug-toggle:hover {
  background: var(--secondary-hover-color);
}

.debug-content pre {
  background: var(--secondary-bg-color);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  padding: 15px;
  overflow-x: auto;
  font-size: 0.85em;
  color: var(--muted-text-color);
  white-space: pre-wrap;
  margin: 0;
}

.error-modal-footer {
  padding: 20px 30px;
  background: var(--secondary-bg-color);
  border-top: 1px solid var(--border-color);
  display: flex;
  justify-content: flex-end;
  gap: 10px;
}

.button {
  padding: 8px 16px;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 0.9em;
  font-weight: 500;
  transition: all 0.2s ease;
  text-decoration: none;
  display: inline-flex;
  align-items: center;
  gap: 6px;
}

.button-primary {
  background: var(--primary-color);
  color: var(--surface-color);
}

.button-primary:hover {
  background: var(--primary-hover-color);
  transform: translateY(-1px);
}

.button-warning {
  background: var(--warning-color);
  color: var(--surface-color);
}

.button-warning:hover {
  background: var(--warning-hover-color, #f59e0b);
  transform: translateY(-1px);
}

.button-secondary {
  background: var(--secondary-color);
  color: var(--text-color);
  border: 1px solid var(--border-color);
}

.button-secondary:hover {
  background: var(--secondary-hover-color);
}
`;

// Attach CSS to component
ErrorModalComponent.css = css;

export const ErrorModal = ErrorModalComponent;