// Auto-generated CSS module for src-ts/ui/error-display.css
export const css = `/**
 * Enhanced error display styles for Nexsock UI
 */

/* Error Modal Styles */
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

.error-modal .error-code {
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

.error-modal .error-message {
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
  display: flex;
  align-items: center;
  gap: 8px;
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
}

/* Enhanced Inline Error Message Styles */
.enhanced-error {
  border-left: 4px solid var(--error-color);
  background: var(--error-bg-color, rgba(var(--error-color-rgb, 239, 68, 68), 0.1));
  position: relative;
}

.enhanced-error .error-summary {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.enhanced-error .error-summary strong {
  color: var(--error-color);
  font-size: 0.9em;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
}

.enhanced-error .error-summary p {
  margin: 0;
  color: var(--text-color);
  line-height: 1.4;
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
  align-self: flex-start;
  margin-top: 4px;
}

.error-details-button:hover {
  background: var(--error-hover-color);
  transform: translateY(-1px);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

/* Messages container positioning */
.messages {
  position: fixed;
  top: 80px; /* Below the navbar */
  right: 20px;
  max-width: 400px;
  z-index: 999;
  display: flex;
  flex-direction: column;
  gap: 10px;
}

/* Enhanced message styles */
.message.enhanced-error {
  background: var(--surface-color);
  border: 1px solid var(--error-color);
  border-radius: 8px;
  padding: 16px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  animation: messageSlideIn 0.3s ease-out;
  max-width: none;
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

/* Miette diagnostic styling in modals */
.diagnostics-content .miette-error {
  color: var(--error-color);
  font-weight: bold;
}

.diagnostics-content .miette-chain {
  color: var(--warning-color);
}

.diagnostics-content .miette-help {
  color: var(--success-color);
  font-style: italic;
}

.diagnostics-content .miette-source {
  color: var(--primary-color);
}

.diagnostics-content .miette-border {
  color: var(--muted-text-color);
}

/* Responsive design */
@media (max-width: 768px) {
  .error-modal {
    width: 95vw;
    margin: 20px;
  }
  
  .error-modal-header,
  .error-modal-body,
  .error-modal-footer {
    padding: 20px;
  }
  
  .messages {
    right: 10px;
    left: 10px;
    max-width: none;
  }
  
  .enhanced-error {
    font-size: 0.9em;
  }
}

/* Button styles for error modals */
.error-modal-footer .button {
  margin-left: 10px;
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

.error-modal-footer .button-primary {
  background: var(--primary-color);
  color: var(--surface-color);
}

.error-modal-footer .button-primary:hover {
  background: var(--primary-hover-color);
  transform: translateY(-1px);
}

.error-modal-footer .button-warning {
  background: var(--warning-color);
  color: var(--surface-color);
}

.error-modal-footer .button-warning:hover {
  background: var(--warning-hover-color, #f59e0b);
  transform: translateY(-1px);
}

.error-modal-footer .button-secondary {
  background: var(--secondary-color);
  color: var(--text-color);
  border: 1px solid var(--border-color);
}

.error-modal-footer .button-secondary:hover {
  background: var(--secondary-hover-color);
}

/* Debug indicator styles */
.debug-mode-indicator {
  position: fixed;
  top: 10px;
  left: 10px;
  background: var(--warning-color);
  color: var(--surface-color);
  padding: 4px 8px;
  border-radius: 4px;
  font-size: 0.8em;
  font-weight: bold;
  z-index: 1001;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
}`;
export default css;
