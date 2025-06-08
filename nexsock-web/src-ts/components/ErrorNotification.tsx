/**
 * ErrorNotification JSX Component for inline error display
 */

import {ErrorDetails} from '../ui/error-display';

interface ErrorNotificationProps {
  errorDetails: ErrorDetails;
  onViewDetails?: () => void;
  onClose?: () => void;
}

/**
 * Enhanced Error Notification Component
 * Shows a compact error notification with option to view details
 */
function ErrorNotificationComponent(props: ErrorNotificationProps): JSX.Element {
  const { errorDetails, onViewDetails, onClose } = props;

  const handleViewDetails = () => {
    if (onViewDetails) {
      onViewDetails();
    } else {
      // Import and use the modal directly
      import('./ErrorModal').then(({ ErrorModal }) => {
        const modalElement = ErrorModal({ errorDetails }) as unknown as HTMLElement;
        document.body.appendChild(modalElement);
      });
    }
  };

  const handleClose = () => {
    if (onClose) {
      onClose();
    } else {
      // Remove the notification element
      const notification = document.querySelector('.enhanced-error') as HTMLElement;
      if (notification && notification.parentNode) {
        notification.parentNode.removeChild(notification);
      }
    }
  };

  return (
    <div className="message message-error enhanced-error">
      <div className="error-summary">
        <strong>{errorDetails.errorCode}</strong>
        <p>{errorDetails.errorMessage}</p>
        <div className="error-actions">
          <button className="error-details-button" onClick={handleViewDetails}>
            View Details
          </button>
          <button className="error-close-button" onClick={handleClose}>
            ×
          </button>
        </div>
      </div>
    </div>
  );
}

// CSS for the ErrorNotification component
const css = `
.enhanced-error {
  border-left: 4px solid var(--error-color);
  background: var(--surface-color);
  border: 1px solid var(--error-color);
  border-radius: 8px;
  padding: 16px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  animation: messageSlideIn 0.3s ease-out;
  position: relative;
}

@keyframes messageSlideIn {
  from {
    opacity: 0;
    transform: translateX(100%);
  }
  to {
    opacity: 1;
    transform: translateX(0);
  }
}

.error-summary {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.error-summary strong {
  color: var(--error-color);
  font-size: 0.9em;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-weight: bold;
}

.error-summary p {
  margin: 0;
  color: var(--text-color);
  line-height: 1.4;
  font-size: 0.95em;
}

.error-actions {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: 8px;
}

.error-details-button {
  background: var(--error-color);
  color: var(--surface-color);
  border: none;
  padding: 6px 12px;
  border-radius: 4px;
  font-size: 0.85em;
  cursor: pointer;
  transition: all 0.2s ease;
  font-weight: 500;
}

.error-details-button:hover {
  background: var(--error-hover-color);
  transform: translateY(-1px);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

.error-close-button {
  background: none;
  border: none;
  color: var(--muted-text-color);
  font-size: 1.2em;
  cursor: pointer;
  padding: 4px;
  border-radius: 50%;
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s ease;
}

.error-close-button:hover {
  background: var(--secondary-bg-color);
  color: var(--text-color);
}

/* Messages container positioning */
.messages {
  position: fixed;
  top: 80px;
  right: 20px;
  max-width: 400px;
  z-index: 999;
  display: flex;
  flex-direction: column;
  gap: 10px;
}

/* Responsive design */
@media (max-width: 768px) {
  .messages {
    right: 10px;
    left: 10px;
    max-width: none;
  }
  
  .enhanced-error {
    font-size: 0.9em;
  }
  
  .error-actions {
    flex-direction: column;
    align-items: stretch;
    gap: 8px;
  }
  
  .error-close-button {
    position: absolute;
    top: 8px;
    right: 8px;
  }
}
`;

// Attach CSS to component
ErrorNotificationComponent.css = css;

export const ErrorNotification = ErrorNotificationComponent;