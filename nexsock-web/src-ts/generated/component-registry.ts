// Auto-generated component registry - do not edit manually
// This file is regenerated on every build

import {NsCard} from '../components/NsCard';
import {NsBadge} from '../components/NsBadge';
import {NsButton} from '../components/NsButton';

import {registerComponent} from '../core/web-components';

export const COMPONENT_REGISTRY = {
  'ns-card': NsCard,
  'ns-badge': NsBadge,
  'ns-button': NsButton,
} as const;

export type ComponentTagName = keyof typeof COMPONENT_REGISTRY;

export function registerAllComponents(): void {
  Object.entries(COMPONENT_REGISTRY).forEach(([tagName, component]) => {
    registerComponent(tagName, component);
  });
  console.log('Auto-registered components:', Object.keys(COMPONENT_REGISTRY));
}
