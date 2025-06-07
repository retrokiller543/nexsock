/**
 * Debug utilities for Nexsock Web Interface
 * Provides easy ways to enable debug mode and configure error handling behavior
 */

/**
 * Debug mode configuration interface
 */
export interface DebugConfig {
  enabled: boolean;
  autoRedirectToErrorPage: boolean;
  verboseLogging: boolean;
}

/**
 * Default debug configuration
 */
const DEFAULT_DEBUG_CONFIG: DebugConfig = {
  enabled: false,
  autoRedirectToErrorPage: false,
  verboseLogging: false,
};

/**
 * Gets the current debug configuration
 */
export function getDebugConfig(): DebugConfig {
  try {
    const stored = localStorage.getItem('nexsock-debug-config');
    if (stored) {
      return { ...DEFAULT_DEBUG_CONFIG, ...JSON.parse(stored) };
    }
  } catch (e) {
    console.warn('Failed to parse debug config from localStorage:', e);
  }
  
  return DEFAULT_DEBUG_CONFIG;
}

/**
 * Updates the debug configuration
 */
export function setDebugConfig(config: Partial<DebugConfig>): void {
  const currentConfig = getDebugConfig();
  const newConfig = { ...currentConfig, ...config };
  
  try {
    localStorage.setItem('nexsock-debug-config', JSON.stringify(newConfig));
    
    // Update individual flags for backward compatibility
    localStorage.setItem('nexsock-debug', newConfig.enabled.toString());
    localStorage.setItem('nexsock-debug-auto-redirect', newConfig.autoRedirectToErrorPage.toString());
    
    console.log('Debug config updated:', newConfig);
  } catch (e) {
    console.error('Failed to save debug config:', e);
  }
}

/**
 * Enables debug mode with optional configuration
 */
export function enableDebugMode(options: Partial<DebugConfig> = {}): void {
  setDebugConfig({ enabled: true, ...options });
  console.log('🚧 Debug mode enabled for Nexsock');
  console.log('To disable: nexsock.debug.disable()');
  console.log('To configure: nexsock.debug.configure({ autoRedirectToErrorPage: true })');
}

/**
 * Disables debug mode
 */
export function disableDebugMode(): void {
  setDebugConfig({ 
    enabled: false, 
    autoRedirectToErrorPage: false,
    verboseLogging: false 
  });
  console.log('Debug mode disabled for Nexsock');
}

/**
 * Configures debug mode settings
 */
export function configureDebugMode(options: Partial<DebugConfig>): void {
  const currentConfig = getDebugConfig();
  if (!currentConfig.enabled) {
    console.warn('Debug mode is not enabled. Enable it first with nexsock.debug.enable()');
    return;
  }
  
  setDebugConfig(options);
  console.log('Debug configuration updated:', { ...currentConfig, ...options });
}

/**
 * Logs debug information if verbose logging is enabled
 */
export function debugLog(message: string, ...args: any[]): void {
  const config = getDebugConfig();
  if (config.enabled && config.verboseLogging) {
    console.log(`[Nexsock Debug] ${message}`, ...args);
  }
}

/**
 * Shows current debug status and available commands
 */
export function showDebugStatus(): void {
  const config = getDebugConfig();
  
  console.group('🚧 Nexsock Debug Status');
  console.log('Enabled:', config.enabled);
  console.log('Auto-redirect to error page:', config.autoRedirectToErrorPage);
  console.log('Verbose logging:', config.verboseLogging);
  console.groupEnd();
  
  if (config.enabled) {
    console.log('\nAvailable debug commands:');
    console.log('- nexsock.debug.disable() - Disable debug mode');
    console.log('- nexsock.debug.configure({ autoRedirectToErrorPage: true }) - Auto-redirect to error pages');
    console.log('- nexsock.debug.configure({ verboseLogging: true }) - Enable verbose logging');
    console.log('- nexsock.debug.testError() - Trigger a test error to see error handling');
  } else {
    console.log('Enable debug mode with: nexsock.debug.enable()');
  }
}

/**
 * Triggers a test error to demonstrate error handling
 */
export function triggerTestError(): void {
  console.log('Triggering test error...');
  
  // Make a request to one of our test error endpoints
  fetch('/api/test-query-error')
    .then(response => {
      if (!response.ok) {
        console.log('Test error response received:', response.status);
      }
    })
    .catch(error => {
      console.error('Test error triggered:', error);
    });
}

/**
 * Debug utilities object for global access
 */
export const debugUtils = {
  enable: enableDebugMode,
  disable: disableDebugMode,
  configure: configureDebugMode,
  status: showDebugStatus,
  testError: triggerTestError,
  getConfig: getDebugConfig,
  log: debugLog,
};