/**
 * Storage-related type definitions for Nexsock Web Interface
 */

// Storage keys for localStorage
export const STORAGE_KEYS = {
  SERVICE_CONFIG: (serviceName: string) => `nexsock_service_config_${serviceName}`,
  GIT_CONTENT_COLLAPSED: (contentId: string) => `nexsock_git_${contentId}_collapsed`,
} as const;