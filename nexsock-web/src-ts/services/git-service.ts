/**
 * Git operations service for Nexsock
 * Handles git-related operations for services
 */

import {STORAGE_KEYS} from '../types/storage';
import {showMessage} from '../ui/messages';

/**
 * Shows a specific git tab (commits or branches)
 */
export function showGitTab(tabName: 'commits' | 'branches', serviceName: string): void {
  // Update tab button states
  document.querySelectorAll('.tab-button').forEach(btn => {
    btn.classList.remove('active');
  });

  // Find and activate the clicked tab button
  const clickedTab = (event as any)?.target as HTMLElement;
  if (clickedTab) {
    clickedTab.classList.add('active');
  }

  // Load the appropriate content
  const tabContent = document.getElementById('git-tab-content');
  if (!tabContent) return;

  if (tabName === 'commits') {
    tabContent.innerHTML = '<div class="loading">Loading commits...</div>';
    window.htmx.ajax('GET', `/api/templates/git-log?service=${serviceName}`, {
      target: '#git-tab-content',
      swap: 'innerHTML'
    });
  } else if (tabName === 'branches') {
    tabContent.innerHTML = '<div class="loading">Loading branches...</div>';
    window.htmx.ajax('GET', `/api/templates/git-branches?service=${serviceName}`, {
      target: '#git-tab-content',
      swap: 'innerHTML'
    });
  }
}

/**
 * Creates a new git branch
 */
export function createNewBranch(serviceName: string): void {
  const input = document.getElementById('new-branch-name') as HTMLInputElement;
  if (!input) return;

  const branchName = input.value.trim();
  if (!branchName) {
    showMessage('Please enter a branch name', 'warning');
    return;
  }

  if (!confirm(`Create new branch "${branchName}" and switch to it?`)) {
    return;
  }

  // Use fetch to create the branch
  const formData = new FormData();
  formData.append('branch', branchName);
  formData.append('create', 'true');

  fetch(`/api/services/${serviceName}/git/checkout/branch`, {
    method: 'POST',
    body: formData
  })
  .then(response => {
    if (!response.ok) {
      throw new Error(`HTTP error: ${response.status}`);
    }
    return response.json();
  })
  .then(data => {
    // Clear the input
    input.value = '';

    // Refresh the git section
    window.htmx.ajax('GET', `/api/templates/git-section?service=${serviceName}`, {
      target: '#git-section',
      swap: 'outerHTML'
    });

    showMessage(`Successfully created and switched to branch "${branchName}"`, 'success');
  })
  .catch(error => {
    console.error('Error creating branch:', error);
    showMessage('Failed to create branch', 'error');
  });
}

/**
 * Refreshes the git section for a service
 */
export function refreshGitSection(serviceName: string): void {
  window.htmx.ajax('GET', `/api/templates/git-section?service=${serviceName}`, {
    target: '#git-section',
    swap: 'outerHTML'
  });
}

/**
 * Toggles git content visibility (commits or branches)
 */
export function toggleGitContent(contentId: string): void {
  const content = document.getElementById(contentId);
  if (!content) return;

  content.classList.toggle('collapsed');

  // Update local storage to remember user preference
  const isCollapsed = content.classList.contains('collapsed');
  localStorage.setItem(STORAGE_KEYS.GIT_CONTENT_COLLAPSED(contentId), isCollapsed.toString());
}

/**
 * Restores git content visibility from user preferences
 */
export function restoreGitContentVisibility(): void {
  // Restore commits visibility
  const commitsCollapsed = localStorage.getItem(STORAGE_KEYS.GIT_CONTENT_COLLAPSED('git-commits-list')) === 'true';
  const commitsList = document.getElementById('git-commits-list');
  if (commitsList && commitsCollapsed) {
    commitsList.classList.add('collapsed');
  }

  // Restore branches visibility
  const branchesCollapsed = localStorage.getItem(STORAGE_KEYS.GIT_CONTENT_COLLAPSED('git-branches-list')) === 'true';
  const branchesList = document.getElementById('git-branches-list');
  if (branchesList && branchesCollapsed) {
    branchesList.classList.add('collapsed');
  }
}
