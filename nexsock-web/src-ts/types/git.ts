/**
 * Git-related type definitions for Nexsock Web Interface
 */

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