/**
 * Configuration management service for Nexsock
 * Handles saving, loading, and managing service configurations
 */

import {ServiceConfig, ServiceConfigs} from '../types/service';
import {STORAGE_KEYS} from '../types/storage';

/**
 * Saves environment variable configuration for a service to localStorage
 */
export function saveServiceConfig(
  serviceName: string, 
  configName: string, 
  envVars: Record<string, string>, 
  description: string = ''
): void {
  const key = STORAGE_KEYS.SERVICE_CONFIG(serviceName);
  const configs = getServiceConfigs(serviceName);

  configs[configName] = {
    envVars,
    description,
    lastUsed: new Date().toISOString(),
    created: configs[configName]?.created || new Date().toISOString()
  };

  localStorage.setItem(key, JSON.stringify(configs));
  console.log(`Saved configuration '${configName}' for service '${serviceName}'`);
}

/**
 * Gets all saved configurations for a service
 */
export function getServiceConfigs(serviceName: string): ServiceConfigs {
  const key = STORAGE_KEYS.SERVICE_CONFIG(serviceName);
  const stored = localStorage.getItem(key);
  return stored ? JSON.parse(stored) : {};
}

/**
 * Loads a specific configuration for a service
 */
export function loadServiceConfig(serviceName: string, configName: string): ServiceConfig | null {
  const configs = getServiceConfigs(serviceName);
  return configs[configName] || null;
}

/**
 * Deletes a configuration for a service
 */
export function deleteServiceConfig(serviceName: string, configName: string): boolean {
  const key = STORAGE_KEYS.SERVICE_CONFIG(serviceName);
  const configs = getServiceConfigs(serviceName);

  if (configs[configName]) {
    delete configs[configName];
    localStorage.setItem(key, JSON.stringify(configs));
    console.log(`Deleted configuration '${configName}' for service '${serviceName}'`);
    return true;
  }
  return false;
}
