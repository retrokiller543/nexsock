/**
 * Service-related type definitions for Nexsock Web Interface
 */

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

// Configuration management types
export interface ConfigurationTemplate {
  name: string;
  description: string;
  envVars: Record<string, string>;
}