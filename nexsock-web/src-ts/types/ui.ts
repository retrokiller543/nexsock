/**
 * UI-related type definitions for Nexsock Web Interface
 */

// UI-related types
export type MessageType = 'success' | 'error' | 'warning' | 'info';

export interface DropdownOptions {
  closeOnClick?: boolean;
  closeOnEscape?: boolean;
  closeOnOutsideClick?: boolean;
}

// Component-related types
export interface ComponentProps {
  scopedClassName?: string;
  styles?: string;
  [key: string]: any;
}