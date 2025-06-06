/**
 * JSX type definitions for Nexsock Web Interface
 */

// Standalone JSX type definitions (no React dependency)
declare global {
  export namespace JSX {
    export interface IntrinsicElements {
      // Common HTML elements with basic attributes
      div: HTMLAttributes;
      span: HTMLAttributes;
      p: HTMLAttributes;
      h1: HTMLAttributes;
      h2: HTMLAttributes;
      h3: HTMLAttributes;
      h4: HTMLAttributes;
      h5: HTMLAttributes;
      h6: HTMLAttributes;
      button: HTMLAttributes & { onClick?: () => void };
      input: HTMLAttributes & { type?: string; value?: string; placeholder?: string };
      form: HTMLAttributes & { onSubmit?: (e: Event) => void };
      label: HTMLAttributes & { htmlFor?: string };
      select: HTMLAttributes & { value?: string; onChange?: (e: Event) => void };
      option: HTMLAttributes & { value?: string; selected?: boolean };
      textarea: HTMLAttributes & { value?: string; rows?: number; cols?: number };
      a: HTMLAttributes & { href?: string; target?: string };
      img: HTMLAttributes & { src?: string; alt?: string; width?: number; height?: number };
      ul: HTMLAttributes;
      ol: HTMLAttributes;
      li: HTMLAttributes;
      table: HTMLAttributes;
      thead: HTMLAttributes;
      tbody: HTMLAttributes;
      tr: HTMLAttributes;
      th: HTMLAttributes;
      td: HTMLAttributes;

      [elemName: string]: any;
    }
    
    interface Element {
      type: string;
      props: any;
      children: any[];
    }
  }

  // Basic HTML attributes interface
  interface HTMLAttributes {
    className?: string;
    id?: string;
    style?: Record<string, string>;
    onClick?: (event: MouseEvent) => void;
    onInput?: (event: InputEvent) => void;
    onChange?: (event: Event) => void;
    onSubmit?: (event: SubmitEvent) => void;
    scopedClassName?: string; // Added for CSS scoping
    styles?: string; // Added for component styles
    [key: string]: any;
  }
}

export {};