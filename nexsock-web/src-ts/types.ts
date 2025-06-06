/**
 * Type definitions for Nexsock Web Interface
 */

import {createServiceCard} from "./example";

// Standalone JSX type definitions (no React dependency)
declare global {
  export namespace JSX {
    export interface IntrinsicElements {
      // Common HTML elements with basic attributes
      div: HTMLAttributes;
      span: HTMLAttributes;
      p: HTMLAttributes;
      h1: HTMLAttributes;
      h2: HTMLAttributes;
      h3: HTMLAttributes;
      h4: HTMLAttributes;
      h5: HTMLAttributes;
      h6: HTMLAttributes;
      button: HTMLAttributes & { onClick?: () => void };
      input: HTMLAttributes & { type?: string; value?: string; placeholder?: string };
      form: HTMLAttributes & { onSubmit?: (e: Event) => void };
      label: HTMLAttributes & { htmlFor?: string };
      select: HTMLAttributes & { value?: string; onChange?: (e: Event) => void };
      option: HTMLAttributes & { value?: string; selected?: boolean };
      textarea: HTMLAttributes & { value?: string; rows?: number; cols?: number };
      a: HTMLAttributes & { href?: string; target?: string };
      img: HTMLAttributes & { src?: string; alt?: string; width?: number; height?: number };
      ul: HTMLAttributes;
      ol: HTMLAttributes;
      li: HTMLAttributes;
      table: HTMLAttributes;
      thead: HTMLAttributes;
      tbody: HTMLAttributes;
      tr: HTMLAttributes;
      th: HTMLAttributes;
      td: HTMLAttributes;

      [elemName: string]: any;
    }
    
    interface Element {
      type: string;
      props: any;
      children: any[];
    }
  }

  // Basic HTML attributes interface
  interface HTMLAttributes {
    className?: string;
    id?: string;
    style?: Record<string, string>;
    onClick?: (event: MouseEvent) => void;
    onInput?: (event: InputEvent) => void;
    onChange?: (event: Event) => void;
    onSubmit?: (event: SubmitEvent) => void;
    [key: string]: any;
  }

  // HTMX global object
  interface Window {
    htmx: {
      ajax: (method: string, url: string, options?: {
        target?: string;
        swap?: string;
        values?: Record<string, string>;
      }) => void;
    };
    nexsock: NexsockAPI;
    createServiceCard: typeof createServiceCard;
    testTSX: () => void;
  }
}

// Service-related types
export interface ServiceConfig {
  envVars: Record<string, string>;
  description: string;
  lastUsed: string;
  created: string;
}

export interface ServiceConfigs {
  [configName: string]: ServiceConfig;
}

export interface ServiceInfo {
  id: string;
  name: string;
  state: 'Running' | 'Stopped' | 'Starting' | 'Failed';
  port?: number;
  repoUrl?: string;
  repoPath?: string;
}

// Git-related types
export interface GitBranch {
  name: string;
  current: boolean;
  remote: boolean;
}

export interface GitCommit {
  hash: string;
  message: string;
  author: string;
  date: string;
}

export interface GitStatus {
  branch: string;
  commit: string;
  remote?: string;
  ahead?: number;
  behind?: number;
  dirty: boolean;
}

// UI-related types
export type MessageType = 'success' | 'error' | 'warning' | 'info';

export interface DropdownOptions {
  closeOnClick?: boolean;
  closeOnEscape?: boolean;
  closeOnOutsideClick?: boolean;
}

// Configuration management types
export interface ConfigurationTemplate {
  name: string;
  description: string;
  envVars: Record<string, string>;
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
}

// DOM Element interfaces for better type safety
export interface EnvVarPair extends HTMLElement {
  querySelector<T extends keyof HTMLElementTagNameMap>(selector: T): HTMLElementTagNameMap[T] | null;
  querySelectorAll<T extends keyof HTMLElementTagNameMap>(selector: T): NodeListOf<HTMLElementTagNameMap[T]>;
}

export interface ServiceForm extends HTMLFormElement {
  elements: HTMLFormControlsCollection & {
    [key: string]: HTMLInputElement | HTMLSelectElement | HTMLTextAreaElement;
  };
}

// Event-related types
export interface HTMXEvent extends Event {
  detail: {
    xhr?: XMLHttpRequest;
    target?: HTMLElement;
    requestConfig?: {
      elt: HTMLElement;
      path: string;
      verb: string;
    };
  };
}

// Storage keys for localStorage
export const STORAGE_KEYS = {
  SERVICE_CONFIG: (serviceName: string) => `nexsock_service_config_${serviceName}`,
  GIT_CONTENT_COLLAPSED: (contentId: string) => `git_${contentId}_collapsed`,
} as const;