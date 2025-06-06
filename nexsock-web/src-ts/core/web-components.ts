/**
 * Web Components bridge for JSX components
 * Allows JSX components to be used directly in HTML templates as custom elements
 */

interface ComponentDefinition {
  component: Function;
  css?: string;
  observedAttributes?: string[];
}

/**
 * Converts JSX component to Web Component
 */
export function createWebComponent(
  tagName: string,
  definition: ComponentDefinition
): void {
  class JSXWebComponent extends HTMLElement {
    private mounted = false;
    
    static get observedAttributes() {
      return definition.observedAttributes || [];
    }
    
    connectedCallback() {
      this.render();
      this.mounted = true;
    }
    
    attributeChangedCallback() {
      if (this.mounted) {
        this.render();
      }
    }
    
    private render() {
      // Convert attributes to props
      const props: Record<string, any> = {};
      
      // Get all attributes
      for (let i = 0; i < this.attributes.length; i++) {
        const attr = this.attributes[i];
        if (!attr) continue;
        let value: any = attr.value;
        
        // Try to parse JSON for complex props
        if (value.startsWith('{') || value.startsWith('[') || value === 'true' || value === 'false') {
          try {
            value = JSON.parse(value);
          } catch {
            // Keep as string if not valid JSON
          }
        }
        
        props[this.camelCase(attr.name)] = value;
      }
      
      // Add event listeners from attributes
      for (const key in props) {
        if (key.startsWith('on') && typeof props[key] === 'string') {
          // Convert string to function (for simple cases)
          const eventName = key.slice(2).toLowerCase();
          const funcBody = props[key];
          
          // Simple event handler support
          if (funcBody.includes('alert') || funcBody.includes('console')) {
            props[key] = new Function('event', funcBody);
          }
        }
      }
      
      // Capture current innerHTML as children before clearing
      const childrenHTML = this.innerHTML;
      const childrenElements: Node[] = [];
      
      // Convert innerHTML to actual DOM nodes if there's content
      if (childrenHTML.trim()) {
        const tempDiv = document.createElement('div');
        tempDiv.innerHTML = childrenHTML;
        childrenElements.push(...Array.from(tempDiv.childNodes));
      }
      
      // Pass children as props if any exist
      if (childrenElements.length > 0) {
        props.children = childrenElements;
      }
      
      // Clear previous content
      this.innerHTML = '';
      
      // Create and append the JSX component
      const element = definition.component(props);
      if (element instanceof Node) {
        this.appendChild(element);
      }
    }
    
    private camelCase(str: string): string {
      return str.replace(/-([a-z])/g, (match, letter) => letter.toUpperCase());
    }
  }
  
  customElements.define(tagName, JSXWebComponent);
}

/**
 * Helper to register a JSX component as a web component
 */
export function registerComponent(tagName: string, component: Function & { css?: string }): void {
  createWebComponent(tagName, {
    component,
    css: component.css,
    observedAttributes: ['*'] // Listen to all attribute changes
  });
}