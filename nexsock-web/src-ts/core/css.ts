/**
 * CSS utilities for scoped styling
 */

let scopeCounter = 0;

/**
 * Creates a scoped CSS class name
 */
export function createScope(componentName?: string): string {
  const id = ++scopeCounter;
  const name = componentName || 'component';
  return `nx-${name}-${id}`;
}

/**
 * Injects CSS with scoping to a specific class
 */
export function injectScopedCSS(css: string, scopeClass: string): void {
  if (!css || !scopeClass) return;
  
  // Simple CSS scoping - prefix all selectors with the scope class
  const scopedCSS = css
    .replace(/([^{}]+){/g, (match, selector) => {
      const trimmedSelector = selector.trim();
      
      // Skip @rules and already scoped selectors
      if (trimmedSelector.startsWith('@') || trimmedSelector.includes(scopeClass)) {
        return match;
      }
      
      // Handle :root specially
      if (trimmedSelector === ':root') {
        return `.${scopeClass} {`;
      }
      
      // Split multiple selectors and scope each one
      const selectors = trimmedSelector.split(',').map((s: string) => {
        const cleaned = s.trim();
        return `.${scopeClass} ${cleaned}`;
      }).join(', ');
      
      return `${selectors} {`;
    });
  
  // Create style element
  const styleElement = document.createElement('style');
  styleElement.textContent = scopedCSS;
  styleElement.setAttribute('data-scope', scopeClass);
  document.head.appendChild(styleElement);
}

/**
 * Utility to easily use CSS modules
 * Usage: const styles = useCSSModule(css)
 */
export function useCSSModule(css: string, componentName?: string): string {
  const scopeClass = createScope(componentName);
  injectScopedCSS(css, scopeClass);
  return scopeClass;
}

/**
 * Type definition for CSS module imports
 */
export interface CSSModule {
  css: string;
  default: string;
}