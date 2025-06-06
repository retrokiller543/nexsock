# Nexsock Web TypeScript Structure

This directory contains the TypeScript source code for the Nexsock Web interface. The code is organized into a modular structure to improve maintainability and readability.

## Directory Structure

```
src-ts/
├── components/     # UI components using TSX
│   ├── Button.tsx  # Button component example
│   ├── ServiceCard.tsx  # Service card component
│   ├── index.ts    # Component registry
│   └── README.md   # Component documentation
├── core/           # Core utilities
│   └── jsx-utils.ts # JSX rendering with CSS scoping
├── services/       # Service-related functionality
│   ├── config-service.ts      # Configuration management
│   ├── env-vars-service.ts    # Environment variables management
│   ├── git-service.ts         # Git operations
│   └── service-management.ts  # Service management
├── types/          # Type definitions
│   └── index.ts    # Type definitions for the application
├── ui/             # UI utilities
│   ├── dropdowns.ts # Dropdown handling
│   ├── messages.ts  # Message handling
│   └── modals.ts    # Modal handling
└── main.ts         # Main application entry point
```

## CSS Scoping for Components

The Nexsock Web interface now supports automatic CSS scoping for components. This means that CSS styles defined within a component will only apply to that component, preventing style conflicts.

### How to Use CSS Scoping

1. Define your component styles as a template string:

```tsx
const styles = `
  .my-component {
    color: blue;
    padding: 10px;
  }
  
  .my-component h3 {
    font-size: 18px;
  }
`;
```

2. Pass the styles to your component:

```tsx
function MyComponent(props: MyComponentProps): JSX.Element {
  return (
    <div className={`my-component ${props.scopedClassName || ''}`}>
      <h3>My Component</h3>
    </div>
  );
}

export function createMyComponent(props: MyComponentProps): HTMLElement {
  return MyComponent({ 
    ...props, 
    styles 
  }) as HTMLElement;
}
```

3. The CSS scoping system will:
   - Generate a unique class name for your component
   - Prefix all your CSS selectors with this class name
   - Add the unique class to your component's root element
   - Inject the scoped CSS into the document

### How It Works

The CSS scoping system works by:

1. Generating a unique class name for each component instance
2. Transforming the CSS to prefix all selectors with this unique class
3. Injecting the transformed CSS into the document
4. Adding the unique class to the component's root element

This ensures that styles defined in one component don't affect other components or the global styles.

## Adding New Components

See the [components/README.md](src-ts/components/README.md) file for detailed instructions on adding new components.

## Building

The TypeScript code is compiled using Bun. See the `package.json` file for available build commands.