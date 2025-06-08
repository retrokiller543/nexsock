/**
 * API interface definitions for Nexsock Web Interface
 */

import {ServiceConfig, ServiceConfigs} from './service';
import {MessageType} from './ui';

// Debug utilities interface
export interface DebugAPI {
  enable: (options?: any) => void;
  disable: () => void;
  configure: (options: any) => void;
  status: () => void;
  testError: () => void;
  getConfig: () => any;
  log: (message: string, ...args: any[]) => void;
}

// API interface
export interface NexsockAPI {
  // Configuration management
  saveServiceConfig: (serviceName: string, configName: string, envVars: Record<string, string>, description?: string) => void;
  getServiceConfigs: (serviceName: string) => ServiceConfigs;
  loadServiceConfig: (serviceName: string, configName: string) => ServiceConfig | null;
  deleteServiceConfig: (serviceName: string, configName: string) => boolean;
  
  // Environment variable management
  getCurrentEnvVars: (serviceName: string) => Record<string, string>;
  applyEnvVarsToForm: (serviceName: string, envVars: Record<string, string>) => void;
  clearCurrentEnvVars: (serviceName: string) => void;
  
  // UI helpers
  loadConfigFromSelection: (serviceName: string, configName: string) => void;
  showSaveConfigModal: (serviceName: string) => void;
  refreshConfigUI: (serviceName: string) => void;
  deleteConfigAndRefresh: (serviceName: string, configName: string) => void;
  toggleManagement: (serviceName: string) => void;
  closeModal: () => void;
  showMessage: (message: string, type?: MessageType) => void;
  confirmRemove: (serviceName: string) => Promise<void>;
  
  // Git operations
  showGitTab: (tabName: 'commits' | 'branches', serviceName: string) => void;
  createNewBranch: (serviceName: string) => void;
  refreshGitSection: (serviceName: string) => void;
  toggleGitContent: (contentId: string) => void;
  restoreGitContentVisibility: () => void;
  
  // Dropdown management
  toggleDropdown: (dropdownId: string) => void;
  closeAllDropdowns: () => void;
  
  // Debug utilities
  debug: DebugAPI;
}