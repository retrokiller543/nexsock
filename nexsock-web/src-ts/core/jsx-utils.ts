// Simple JSX factory functions for standalone JSX without React with CSS scoping support

// Forward declarations for JSX
type CreateElementType = (
  tag: string | Function,
  props: Record<string, any> | null,
  ...children: any[]
) => HTMLElement | DocumentFragment;

type FragmentType = ({ children }: { children: any[] }) => DocumentFragment;

// Make createElement and Fragment global for JSX
declare global {
  var createElement: CreateElementType;
  var Fragment: FragmentType;
}

// Context to track the current scope for transparent scoping
let currentScopeId: string | null = null;

/**
 * Generates a unique CSS class name for component scoping
 * @param componentName The name of the component
 * @returns A unique CSS class name
 */
function generateScopedClassName(componentName: string): string {
  return `nx-${componentName}-${Math.random().toString(36).substring(2, 8)}`;
}

/**
 * Injects scoped CSS into the document using data attributes
 * @param css The CSS to inject
 * @param scopeId The unique scope identifier
 */
export function injectScopedCSS(css: string, scopeId: string): void {
  if (!css || !scopeId) return;
  
  // Replace selectors to use data attributes instead of classes
  const scopedCSS = css
    .replace(/([^{}]+){/g, (match, selector) => {
      const trimmedSelector = selector.trim();
      
      // Skip @rules and already scoped selectors
      if (trimmedSelector.startsWith('@') || trimmedSelector.includes(`[data-scope="${scopeId}"]`)) {
        return match;
      }
      
      // Split multiple selectors and scope each one
      const selectors = trimmedSelector.split(',').map((s: string) => {
        const cleaned = s.trim();
        // Add data attribute selector to scope the CSS
        return `[data-scope="${scopeId}"] ${cleaned}`;
      }).join(', ');
      
      return `${selectors} {`;
    });
  
  // Create and append style element
  const style = document.createElement('style');
  style.textContent = scopedCSS;
  style.setAttribute('data-scope-id', scopeId);
  document.head.appendChild(style);
}

/**
 * Creates a DOM element from JSX with automatic CSS scoping
 */
export function createElement(
  tag: string | Function,
  props: Record<string, any> | null,
  ...children: any[]
): HTMLElement | DocumentFragment {
  // Handle functional components
  if (typeof tag === 'function') {
    // Check if component is a component object with CSS
    if (tag && typeof tag === 'object' && 'component' in tag && 'css' in tag) {
      const componentObj = tag as any;
      const componentName = componentObj.component.name || 'Component';
      const scopeId = generateScopedClassName(componentName);
      
      // Inject the scoped CSS
      injectScopedCSS(componentObj.css, scopeId);
      
      // Set the current scope context
      const previousScopeId = currentScopeId;
      currentScopeId = scopeId;
      
      // Call the component function
      const result = componentObj.component({ ...props, children });
      
      // Restore previous scope context
      currentScopeId = previousScopeId;
      
      return result;
    }
    
    // Check if function has a css property attached
    if ((tag as any).css) {
      const componentName = tag.name || 'Component';
      const scopeId = generateScopedClassName(componentName);
      
      // Inject the scoped CSS
      injectScopedCSS((tag as any).css, scopeId);
      
      // Set the current scope context
      const previousScopeId = currentScopeId;
      currentScopeId = scopeId;
      
      // Call the component function
      const result = tag({ ...props, children });
      
      // Restore previous scope context
      currentScopeId = previousScopeId;
      
      return result;
    }
    
    return tag({ ...props, children });
  }

  // Create the DOM element
  const element = document.createElement(tag);
  
  // Automatically apply current scope if one exists
  if (currentScopeId) {
    element.setAttribute('data-scope', currentScopeId);
  }
  
  if (props) {
    Object.entries(props).forEach(([key, value]) => {
      // Handle special props
      if (key === 'className') {
        element.className = value;
      } else if (key === 'css') {
        // Handle inline CSS with automatic scoping
        const scopeId = generateScopedClassName('inline');
        injectScopedCSS(value, scopeId);
        element.setAttribute('data-scope', scopeId);
      } else if (key.startsWith('on') && typeof value === 'function') {
        const event = key.toLowerCase().slice(2);
        element.addEventListener(event, value);
      } else {
        element.setAttribute(key, value);
      }
    });
  }

  // Append children
  children.flat().forEach(child => {
    if (typeof child === 'string' || typeof child === 'number') {
      element.appendChild(document.createTextNode(String(child)));
    } else if (child instanceof Node) {
      element.appendChild(child);
    }
  });

  return element;
}

/**
 * Creates a document fragment for JSX fragments
 */
export function Fragment({ children }: { children: any[] }): DocumentFragment {
  const fragment = document.createDocumentFragment();
  children.flat().forEach(child => {
    if (typeof child === 'string' || typeof child === 'number') {
      fragment.appendChild(document.createTextNode(String(child)));
    } else if (child instanceof Node) {
      fragment.appendChild(child);
    }
  });
  return fragment;
}

// Make createElement and Fragment available globally for JSX
(globalThis as any).createElement = createElement;
(globalThis as any).Fragment = Fragment;