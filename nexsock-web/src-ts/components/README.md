# TSX Components in Nexsock Web

This directory contains reusable TSX components for the Nexsock Web interface.

## How to Add a New Component

Adding a new TSX component is now simplified with the component registry system. Follow these steps:

### 1. Create a new component file

Create a new `.tsx` file in the `src-ts/components` directory. You can use `Button.tsx` as a template.

Basic structure of a component file:

```tsx
import { createElement, Fragment } from '../jsx-utils';

// Make createElement and Fragment global for JSX
declare global {
  var createElement: typeof import('../jsx-utils').createElement;
  var Fragment: typeof import('../jsx-utils').Fragment;
}

// Define your component's props interface
interface MyComponentProps {
  // Add your props here
}

/**
 * Your component implementation
 */
function MyComponent(props: MyComponentProps): JSX.Element {
  return (
    <div className="my-component">
      {/* Your JSX here */}
      <h3>My Component</h3>
    </div>
  );
}

/**
 * Export a factory function to create your component
 */
export function createMyComponent(props: MyComponentProps): HTMLElement {
  return MyComponent(props) as HTMLElement;
}
```

### 2. Register your component in the registry

Open `src-ts/components/index.ts` and add your component to the registry:

```typescript
// Import all components here
import { createServiceCard } from '../example';
import { createButton } from './Button';
import { createMyComponent } from './MyComponent'; // Add your import

// Export a registry of all components
export const componentRegistry = {
  createServiceCard,
  createButton,
  createMyComponent, // Add your component
};
```

### 3. Use your component

Your component will be automatically registered globally and available in two ways:

#### Option 1: Using the global function

```javascript
// In any JavaScript/TypeScript code
const myElement = createMyComponent({
  // Your props here
});
document.getElementById('container').appendChild(myElement);
```

#### Option 2: Using the component registry

```typescript
// In TypeScript code with access to the registry
import { componentRegistry } from './components';

const myElement = componentRegistry.createMyComponent({
  // Your props here
});
document.getElementById('container').appendChild(myElement);
```

## Testing Your Component

You can test your component by adding it to the `testTSX` function in `main.ts`:

```typescript
window.testTSX = function() {
  const container = document.getElementById('tsx-test-container');
  if (container) {
    // Clear previous content
    container.innerHTML = '';

    // Add your component
    const myElement = componentRegistry.createMyComponent({
      // Your props here
    });
    container.appendChild(myElement);
  }
};
```

Then click the "Test TSX" button in the footer of the application.

## Building

The TypeScript/TSX files are automatically compiled when you build the Rust project:

```bash
cargo build
```

For development with automatic recompilation, you can use:

```bash
bun run watch
```

This will watch for changes in the TypeScript/TSX files and recompile them automatically.
