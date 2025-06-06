/**
 * Theme management service for dark/light mode toggle
 */

export type Theme = 'light' | 'dark' | 'auto';

const THEME_STORAGE_KEY = 'nexsock-theme';
const THEME_ICONS = {
  light: '☀️',
  dark: '🌙',
  auto: '💻'
};

export class ThemeService {
  private currentTheme: Theme = 'auto';
  private isInitialized: boolean = false;

  constructor() {
    if (!this.isInitialized) {
      this.initializeTheme();
      this.setupThemeToggle();
      this.isInitialized = true;
    }
  }

  /**
   * Set a specific theme
   */
  public setTheme(theme: Theme): void {
    if (!this.isValidTheme(theme)) {
      console.warn(`Invalid theme: ${theme}`);
      return;
    }

    this.currentTheme = theme;
    localStorage.setItem(THEME_STORAGE_KEY, theme);
    this.applyTheme();
    this.updateThemeIcon();
  }

  /**
   * Get the current theme
   */
  public getCurrentTheme(): Theme {
    return this.currentTheme;
  }

  /**
   * Get the effective theme (resolves 'auto' to actual theme)
   */
  public getEffectiveTheme(): 'light' | 'dark' {
    if (this.currentTheme === 'auto') {
      return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
    }
    return this.currentTheme;
  }

  /**
   * Initialize theme from localStorage or system preference
   */
  private initializeTheme(): void {
    const savedTheme = localStorage.getItem(THEME_STORAGE_KEY) as Theme;

    if (savedTheme && this.isValidTheme(savedTheme)) {
      this.currentTheme = savedTheme;
    } else {
      this.currentTheme = 'auto';
    }

    this.applyTheme();
    this.updateThemeIcon();
  }

  /**
   * Set up theme toggle button event listener
   */
  private setupThemeToggle(): void {
    const themeToggle = document.getElementById('theme-toggle');
    if (themeToggle && !themeToggle.hasAttribute('data-theme-listener')) {
      const clickHandler = () => {
        this.toggleTheme();
      };
      themeToggle.addEventListener('click', clickHandler);
      themeToggle.setAttribute('data-theme-listener', 'true');
    }
  }

  /**
   * Toggle between light, dark, and auto themes
   */
  private toggleTheme(): void {
    const themes: Theme[] = ['light', 'dark', 'auto'];
    const currentIndex = themes.indexOf(this.currentTheme);
    const nextIndex = (currentIndex + 1) % themes.length;
    const nextTheme = themes[nextIndex];

    if (nextTheme) {
      this.setTheme(nextTheme);
    }
  }

  /**
   * Apply the current theme to the document
   */
  private applyTheme(): void {
    document.documentElement.setAttribute('data-theme', this.currentTheme);
  }

  /**
   * Update the theme toggle icon
   */
  private updateThemeIcon(): void {
    const themeIcon = document.querySelector('.theme-icon');
    if (themeIcon) {
      themeIcon.textContent = THEME_ICONS[this.currentTheme];
    }

    const themeToggle = document.getElementById('theme-toggle');
    if (themeToggle) {
      themeToggle.setAttribute('aria-label', `Current theme: ${this.currentTheme}. Click to change theme.`);
    }
  }

  /**
   * Check if a theme value is valid
   */
  private isValidTheme(theme: string): theme is Theme {
    return ['light', 'dark', 'auto'].includes(theme);
  }
}

// Global instance
let themeService: ThemeService | null = null;

/**
 * Initialize the theme service (idempotent)
 */
export function initializeThemeService(): ThemeService {
  if (!themeService) {
    themeService = new ThemeService();
  }
  return themeService;
}

/**
 * Get the global theme service instance
 */
export function getThemeService(): ThemeService | null {
  return themeService;
}