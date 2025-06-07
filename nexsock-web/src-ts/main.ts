/**
 * Main TypeScript file for Nexsock Web Interface
 * Provides JSX-based UI components as web components for use in Rust templates
 */

// Import JSX utilities first to set up global functions
import './core/jsx-utils';

// Import styles
import './ui/error-display.css';

import {registerAllComponents} from './generated/component-registry';
import {initializeApp} from './core/init';
import {createGlobalAPI} from './core/api';

// ===============================================
// Initialization
// ===============================================

// Initialize when page loads
document.addEventListener('DOMContentLoaded', () => {
  // Initialize the application
  initializeApp();

  // Create and make API available globally
  window.nexsock = createGlobalAPI();

  // Register all UI components as web components  
  registerAllComponents();

  console.log('Nexsock web interface initialized successfully');
});
