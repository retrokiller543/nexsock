/**
 * Dropdown handling utilities for Nexsock UI
 */

/**
 * Toggles dropdown menu visibility
 */
export function toggleDropdown(dropdownId: string): void {
  const dropdown = document.getElementById(dropdownId);
  if (!dropdown) return;

  // Close all other dropdowns first
  document.querySelectorAll('.dropdown.active').forEach(dd => {
    if (dd.id !== dropdownId) {
      dd.classList.remove('active');
    }
  });

  // Toggle this dropdown
  dropdown.classList.toggle('active');
}

/**
 * Closes all open dropdowns
 */
export function closeAllDropdowns(): void {
  document.querySelectorAll('.dropdown.active').forEach(dropdown => {
    dropdown.classList.remove('active');
  });
}