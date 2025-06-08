/**
 * Global API object for Nexsock Web Interface
 */

import {NexsockAPI} from '../types/api';

// Import services
import {deleteServiceConfig, getServiceConfigs, loadServiceConfig, saveServiceConfig} from '../services/config-service';

import {applyEnvVarsToForm, clearCurrentEnvVars, getCurrentEnvVars} from '../services/env-vars-service';

import {
  createNewBranch,
  refreshGitSection,
  restoreGitContentVisibility,
  showGitTab,
  toggleGitContent
} from '../services/git-service';

import {
  confirmRemove,
  deleteConfigAndRefresh,
  loadConfigFromSelection,
  refreshConfigUI,
  showSaveConfigModal,
  toggleManagement
} from '../services/service-management';

// Import UI utilities
import {showMessage} from '../ui/messages';
import {closeModal} from '../ui/modals';
import {closeAllDropdowns, toggleDropdown} from '../ui/dropdowns';

// Import debug utilities
import {debugUtils} from './debug';

/**
 * Create and export the global API object
 */
export const createGlobalAPI = (): NexsockAPI => {
  return {
    saveServiceConfig,
    getServiceConfigs,
    loadServiceConfig,
    deleteServiceConfig,
    getCurrentEnvVars,
    applyEnvVarsToForm,
    loadConfigFromSelection,
    showSaveConfigModal,
    refreshConfigUI,
    deleteConfigAndRefresh,
    toggleManagement,
    closeModal,
    showMessage,
    confirmRemove,
    showGitTab,
    createNewBranch,
    refreshGitSection,
    toggleDropdown,
    closeAllDropdowns,
    clearCurrentEnvVars,
    toggleGitContent,
    restoreGitContentVisibility,
    debug: debugUtils
  };
};