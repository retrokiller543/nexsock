/**
 * DOM Element interface definitions for Nexsock Web Interface
 */

// DOM Element interfaces for better type safety
export interface EnvVarPair extends HTMLElement {
  querySelector<T extends keyof HTMLElementTagNameMap>(selector: T): HTMLElementTagNameMap[T] | null;
  querySelectorAll<T extends keyof HTMLElementTagNameMap>(selector: T): NodeListOf<HTMLElementTagNameMap[T]>;
}

export interface ServiceForm extends HTMLFormElement {
  elements: HTMLFormControlsCollection & {
    [key: string]: HTMLInputElement | HTMLSelectElement | HTMLTextAreaElement;
  };
}