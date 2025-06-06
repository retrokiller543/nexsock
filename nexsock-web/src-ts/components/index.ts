// Nexsock UI Components
// This file registers all components as web components for use in templates

import {registerComponent} from '../core/web-components';
import {NsButton} from './NsButton';
import {NsCard} from './NsCard';
import {NsBadge} from './NsBadge';

// Export all components
export {
  NsButton,
  NsCard,
  NsBadge
};

/**
 * Register all components as web components
 * This allows them to be used directly in HTML templates
 */
export function registerComponents() {
  // Register UI Kit components
  registerComponent('ns-button', NsButton);
  registerComponent('ns-card', NsCard);
  registerComponent('ns-badge', NsBadge);

  console.log('Nexsock UI components registered:', [
    'ns-button',
    'ns-card', 
    'ns-badge'
  ]);
}

// Legacy registry for backward compatibility (can be removed later)
export const componentRegistry = {
  // Keep empty or add legacy components if needed
};