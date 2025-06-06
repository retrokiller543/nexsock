// Auto-generated CSS module for src-ts/components/NsButton.css
export const css = `.ns-button {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  padding: var(--spacing-sm, 8px) var(--spacing-lg, 16px);
  border: 1px solid transparent;
  border-radius: var(--border-radius-md, 6px);
  cursor: pointer;
  font-weight: 500;
  font-size: var(--font-size-base, 14px);
  line-height: 1.4;
  text-decoration: none;
  transition: all var(--transition-fast, 0.15s) ease;
  user-select: none;
  min-height: 36px;
  gap: 6px;
  font-family: var(--font-family, inherit);
}

.ns-button:focus {
  outline: 2px solid var(--primary, #0070f3);
  outline-offset: 2px;
}

.ns-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
  pointer-events: none;
}

/* Variants */
.ns-button.primary {
  background-color: var(--primary, #0070f3);
  color: var(--text-inverse, white);
  border-color: var(--primary, #0070f3);
}

.ns-button.primary:hover:not(:disabled) {
  background-color: var(--primary-hover, #0061d5);
  border-color: var(--primary-hover, #0061d5);
  transform: translateY(-1px);
}

.ns-button.secondary {
  background-color: var(--color-surface-elevated, #f8f9fa);
  color: var(--text-secondary, #666);
  border-color: var(--color-border, #e1e5e9);
}

.ns-button.secondary:hover:not(:disabled) {
  background-color: var(--color-surface-hover, #f5f5f5);
  color: var(--text-primary, #2d3748);
  border-color: var(--color-border, #e1e5e9);
}

.ns-button.danger {
  background-color: var(--danger, #dc3545);
  color: var(--text-inverse, white);
  border-color: var(--danger, #dc3545);
}

.ns-button.danger:hover:not(:disabled) {
  background-color: var(--danger-hover, #c82333);
  border-color: var(--danger-hover, #c82333);
  transform: translateY(-1px);
}

.ns-button.ghost {
  background-color: transparent;
  color: var(--text-secondary, #666);
  border: 1px solid transparent;
}

.ns-button.ghost:hover:not(:disabled) {
  background-color: var(--color-surface-elevated, #f8f9fa);
  color: var(--text-primary, #2d3748);
}

/* Sizes */
.ns-button.small {
  padding: 4px 12px;
  font-size: 12px;
  min-height: 28px;
}

.ns-button.large {
  padding: 12px 24px;
  font-size: 16px;
  min-height: 44px;
}

/* Loading state */
.ns-button.loading {
  position: relative;
  color: transparent;
}

.ns-button.loading::after {
  content: '';
  position: absolute;
  width: 16px;
  height: 16px;
  border: 2px solid currentColor;
  border-radius: 50%;
  border-top-color: transparent;
  animation: spin 0.8s linear infinite;
  color: inherit;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}`;
export default css;
